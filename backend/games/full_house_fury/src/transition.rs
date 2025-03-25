use ajuna_primitives::{sage_api::SageApi};
use sage_api::{rules::*, traits::TransitionOutput, SageGameTransition, TransitionError};
use sp_std::vec;

use crate::{
	rules::ensure_account_has_no_asset_of_type,
	types::{deck::Deck, game::Game, AssetId, AssetType, BaseAsset},
};
use frame_support::{pallet_prelude::{Decode, Encode, TypeInfo}, BoundedVec, Parameter};
use frame_support::pallet_prelude::ConstU32;
use parity_scale_codec::{Codec, MaxEncodedLen};
use sp_core::H256;
use sp_runtime::{
	traits::{AtLeast32BitUnsigned, BlockNumber as BlockNumberT, Member},
};
use sp_std::{marker::PhantomData, vec::Vec};
use crate::error::FuryError;
use crate::types::game::{Attack, Boss, GameState, LevelState};

pub type TransitionConfig = ();

/// The extra stands for the hand positions of the cards chosen for an attack.
pub type HandPositions = Option<BoundedVec<u8, ConstU32<10>>>;

pub struct FullHouseFuryTransition<AccountId, BlockNumber, Sage> {
	_phantom: PhantomData<(AccountId, BlockNumber, Sage)>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum TransitionIdentifier {
	Start,
	// Play,
	Preparation,
	Battle,
	Discard,
	Score,
}

impl<AccountId, BlockNumber, Balance, Sage> FullHouseFuryTransition<AccountId, BlockNumber, Sage>
where
	AccountId: Member + Codec,
	BlockNumber: BlockNumberT,
	Balance: Member + Parameter + AtLeast32BitUnsigned + MaxEncodedLen,
	Sage: SageApi<
		AccountId = AccountId,
		AssetId = AssetId,
		Asset = BaseAsset<BlockNumber>,
		Balance = Balance,
		BlockNumber = BlockNumber,
		TransitionConfig = TransitionConfig,
		HashOutput = H256,
	>,
{
	fn try_get_assets(
		asset_ids: &[AssetId],
	) -> Result<Vec<(AssetId, BaseAsset<BlockNumber>)>, TransitionError> {
		asset_ids
			.iter()
			.copied()
			.map(|asset_id| Ok((asset_id, Sage::get_asset(&asset_id)?)))
			.collect::<Result<Vec<_>, _>>()
	}

	fn generate_asset_id() -> Result<AssetId, TransitionError> {
		Sage::create_next_asset_id().ok_or(TransitionError::CouldNotCreateAssetId)
	}

	fn verify_transition_rules(
		transition_id: &TransitionIdentifier,
		account_id: &AccountId,
		asset_ids: &[AssetId],
	) -> Result<Vec<(AssetId, BaseAsset<BlockNumber>)>, TransitionError> {
		let mut maybe_assets = None;

		match transition_id {
			TransitionIdentifier::Start => {
				ensure_asset_length(asset_ids, 0)?;
				ensure_account_has_no_asset_of_type::<_, _, Sage>(account_id, AssetType::Game)?;
			},
			_ => {},
		}

		if maybe_assets.is_none() {
			maybe_assets = Some(Self::try_get_assets(asset_ids)?);
		}

		Ok(maybe_assets.unwrap())
	}

	fn transition_assets(
		transition_id: &TransitionIdentifier,
		account_id: &AccountId,
		mut assets: Vec<(AssetId, BaseAsset<BlockNumber>)>,
		hand_positions: &HandPositions,
		_: Option<Sage::FungiblesAssetId>,
	) -> Result<Vec<TransitionOutput<AssetId, BaseAsset<BlockNumber>>>, TransitionError> {
		let output = match transition_id {
			TransitionIdentifier::Start => {
				let current_block = Sage::get_current_block_number();

				let game = Game::start_new();
				let deck = Deck::new_deck();

				let game_asset = BaseAsset {
					id: Self::generate_asset_id()?,
					collection_id: 0,
					genesis: current_block,
					asset_type: AssetType::Game,
					fury_asset: game.encode().try_into().unwrap(),
				};

				let deck_asset = BaseAsset {
					id: Self::generate_asset_id()?,
					collection_id: 0,
					genesis: current_block,
					asset_type: AssetType::Deck,
					fury_asset: deck.encode().try_into().unwrap(),
				};

				vec![TransitionOutput::Minted(game_asset), TransitionOutput::Minted(deck_asset)]
			},
			TransitionIdentifier::Preparation => {
				let (game_id, mut game_asset) = assets.pop().ok_or_else(|| TransitionError::AssetLength)?;
				let mut game = Game::decode(&mut game_asset.fury_asset.as_slice()).map_err(|_e| TransitionError::AssetCouldNotBeDecoded)?;

				let (deck_id, mut deck_asset) = assets.pop().ok_or_else(|| TransitionError::AssetLength)?;
				let mut deck = Deck::decode(&mut deck_asset.fury_asset.as_slice()).map_err(|_e| TransitionError::AssetCouldNotBeDecoded)?;

				if game.game_sate != GameState::Running {
					return Err(FuryError::GameNotInRunningPhase.into());
				}

				if game.level_state != LevelState::Preparation {
					return Err(FuryError::LevelStateIsNotPreparation.into());
				}

				if game.level > 1 {
					// todo: implement level preparation logic.
				}

				game.round = 1;
				game.level_state = LevelState::Battle;

				game.attack = Attack {
					hand: 0,
					attack_type: None,
					score: 0,
				};

				let subject = (&account_id, &game_id, &deck_id);
				deck.draw(game.player.hand_size, Sage::random_hash(subject.encode().as_slice())).map_err(|_e| TransitionError::Transition {code: 0})?;

				// write back changes to assets
				game_asset.fury_asset = game.encode().try_into().map_err(|_e| TransitionError::AssetDataTooLong)?;
				deck_asset.fury_asset = deck.encode().try_into().map_err(|_e| TransitionError::AssetDataTooLong)?;

				vec![TransitionOutput::Mutated(game_id, game_asset), TransitionOutput::Mutated(deck_id, deck_asset)]
			},
			TransitionIdentifier::Battle => {
				let (game_id, mut game_asset) = assets.pop().ok_or_else(|| TransitionError::AssetLength)?;
				let mut game = Game::decode(&mut game_asset.fury_asset.as_slice()).map_err(|_e| TransitionError::AssetCouldNotBeDecoded)?;

				let (deck_id, mut deck_asset) = assets.pop().ok_or_else(|| TransitionError::AssetLength)?;
				let mut deck = Deck::decode(&mut deck_asset.fury_asset.as_slice()).map_err(|_e| TransitionError::AssetCouldNotBeDecoded)?;

				if game.game_sate != GameState::Running {
					return Err(TransitionError::Transition {code: 0});
				}

				if game.level_state != LevelState::Battle {
					return Err(TransitionError::Transition {code: 0});
				}

				let attack_positions = hand_positions.as_ref().ok_or_else(|| FuryError::NoHandPositionsFound)?;

				let attack_cards = deck.hand.pick_multiple_cards(attack_positions)?;

				game.attack = Attack::create(&attack_cards)?;

				game.boss.add_damage(game.attack.score);

				game.player.decrease_endurance();


				// Continue the game for as long both parties are alive
				if game.boss.is_alive() && game.player.is_alive() {
					game.level_state = LevelState::Battle;

					game.round = game.round.saturating_add(1);


					let subject = (&account_id, &game_id, &deck_id);
					let random_hash = Sage::random_hash(subject.encode().as_slice());
					deck.draw(game.player.hand_size, random_hash)?;

				} else {
					game.level_state = LevelState::Score;
				}

				if !game.player.is_alive() || (deck.deck_size + deck.hand.cards_count()) == 0 {
					game.game_sate = GameState::Finished;
				}

				// write back changes to assets
				game_asset.fury_asset = game.encode().try_into().map_err(|_e| TransitionError::AssetDataTooLong)?;
				deck_asset.fury_asset = deck.encode().try_into().map_err(|_e| TransitionError::AssetDataTooLong)?;

				vec![TransitionOutput::Mutated(game_id, game_asset), TransitionOutput::Mutated(deck_id, deck_asset)]
			},
			TransitionIdentifier::Discard => {
				let (game_id, mut game_asset) = assets.pop().ok_or_else(|| TransitionError::AssetLength)?;
				let mut game = Game::decode(&mut game_asset.fury_asset.as_slice()).map_err(|_e| TransitionError::AssetCouldNotBeDecoded)?;

				let (deck_id, mut deck_asset) = assets.pop().ok_or_else(|| TransitionError::AssetLength)?;
				let mut deck = Deck::decode(&mut deck_asset.fury_asset.as_slice()).map_err(|_e| TransitionError::AssetCouldNotBeDecoded)?;

				if game.game_sate != GameState::Running {
					return Err(FuryError::GameNotInRunningPhase)?;
				}

				if game.level_state != LevelState::Battle {
					return Err(FuryError::LevelStateIsNotBattle)?;
				}

				if game.player.discard > 0 {
					game.player.discard -= 1;
				} else {
					return Err(FuryError::DiscardLimitReached)?;
				}

				let discard_positions = hand_positions.as_ref().ok_or_else(|| TransitionError::Transition { code: 0})?;

				// this step does also remove them from the hand, so we can simply ignore them.
				let _discard_cards = deck.hand.pick_multiple_cards(discard_positions)?;

				// draw new cards for the discarded cards
				let subject = (&account_id, &game_id, &deck_id);
				let random_hash = Sage::random_hash(subject.encode().as_slice());
				deck.draw(game.player.hand_size, random_hash)?;

				// write back changes to assets
				game_asset.fury_asset = game.encode().try_into().map_err(|_e| TransitionError::AssetDataTooLong)?;
				deck_asset.fury_asset = deck.encode().try_into().map_err(|_e| TransitionError::AssetDataTooLong)?;

				vec![TransitionOutput::Mutated(game_id, game_asset), TransitionOutput::Mutated(deck_id, deck_asset)]
			},
			TransitionIdentifier::Score => {
				let (game_id, mut game_asset) = assets.pop().ok_or_else(|| TransitionError::AssetLength)?;
				let mut game = Game::decode(&mut game_asset.fury_asset.as_slice()).map_err(|_e| TransitionError::AssetCouldNotBeDecoded)?;

				let (deck_id, mut deck_asset) = assets.pop().ok_or_else(|| TransitionError::AssetLength)?;

				game.clear_attack();

				game.level = game.level.saturating_add(1);

				game.boss = Boss {
					// convert u8 to u32 to prevent early saturation.
					max_health: (game.level as u32).saturating_pow(2).saturating_mul(100),
					damage: 0,
				};

				// do not reset player health for now, but reset endurance
				game.player.reset_endurance();

				// new deck and empty hand
				let deck = Deck::new_deck();

				game.level_state = LevelState::Preparation;

				// write back changes to assets
				game_asset.fury_asset = game.encode().try_into().map_err(|_e| TransitionError::AssetDataTooLong)?;
				deck_asset.fury_asset = deck.encode().try_into().map_err(|_e| TransitionError::AssetDataTooLong)?;

				vec![TransitionOutput::Mutated(game_id, game_asset), TransitionOutput::Mutated(deck_id, deck_asset)]
			},
		};

		Ok(output)
	}
}

impl<AccountId, BlockNumber, Balance, Sage> SageGameTransition
	for FullHouseFuryTransition<AccountId, BlockNumber, Sage>
where
	AccountId: Member + Codec,
	BlockNumber: BlockNumberT,
	Balance: Member + Parameter + AtLeast32BitUnsigned + MaxEncodedLen,
	Sage: SageApi<
		AccountId = AccountId,
		AssetId = AssetId,
		Asset = BaseAsset<BlockNumber>,
		Balance = Balance,
		BlockNumber = BlockNumber,
		TransitionConfig = TransitionConfig,
		HashOutput = H256,
	>,
{
	type TransitionId = TransitionIdentifier;
	type TransitionConfig = TransitionConfig;
	type AccountId = AccountId;
	type AssetId = AssetId;
	type Asset = BaseAsset<BlockNumber>;
	type Extra = HandPositions;
	type PaymentFungible = Sage::FungiblesAssetId;

	fn do_transition(
		transition_id: &Self::TransitionId,
		account_id: &Self::AccountId,
		assets_ids: &[Self::AssetId],
		positions: &Self::Extra,
		payment_asset: Option<Self::PaymentFungible>,
	) -> Result<Vec<TransitionOutput<Self::AssetId, Self::Asset>>, TransitionError> {
		let assets = Self::verify_transition_rules(transition_id, account_id, assets_ids)?;
		Self::transition_assets(transition_id, account_id, assets, &positions, payment_asset)
	}
}
