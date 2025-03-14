use crate::error::*;
use ajuna_primitives::{payment_handler::NativeId, sage_api::SageApi};
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
	SaturatedConversion,
};
use sp_std::{marker::PhantomData, vec::Vec};
use crate::types::game::{Attack, GameState, LevelState};

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
	fn try_get_asset(asset_id: &AssetId) -> Result<BaseAsset<BlockNumber>, TransitionError> {
		let asset = Sage::get_asset(asset_id)
			// Fixme, this kind of error should be in handled in the api
			.map_err(|_| TransitionError::Transition { code: 0 })?;

		Ok(asset)
	}

	fn try_get_assets(
		asset_ids: &[AssetId],
	) -> Result<Vec<(AssetId, BaseAsset<BlockNumber>)>, TransitionError> {
		asset_ids
			.iter()
			.copied()
			.map(|asset_id| Ok((asset_id, Self::try_get_asset(&asset_id)?)))
			.collect::<Result<Vec<_>, _>>()
	}

	fn get_asset_funds(
		asset_id: &AssetId,
		payment_asset: Option<&Sage::FungiblesAssetId>,
	) -> Balance {
		let fund_id = if let Some(payment) = payment_asset {
			payment
		} else {
			&Sage::FungiblesAssetId::get_native_id()
		};

		Sage::inspect_asset_funds(asset_id, fund_id)
	}

	fn deposit_funds_to_asset(
		asset_id: &AssetId,
		from: &AccountId,
		amount: Balance,
	) -> Result<(), TransitionError> {
		let fund_id = Sage::FungiblesAssetId::get_native_id();
		Sage::deposit_funds_to_asset(asset_id, from, fund_id, amount)
			.map_err(|_| TransitionError::Transition { code: 0 })
	}

	fn withdraw_funds_from_asset(
		asset_id: &AssetId,
		to: &AccountId,
		amount: Balance,
	) -> Result<(), TransitionError> {
		let fund_id = Sage::FungiblesAssetId::get_native_id();
		Sage::transfer_funds_from_asset(asset_id, to, fund_id, amount)
			.map_err(|_| TransitionError::Transition { code: 0 })
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
		attack_positions: &HandPositions,
		payment_asset: Option<Sage::FungiblesAssetId>,
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
			// TransitionIdentifier::Play => {
				// let (game_id, game_asset) = assets.pop().ok_or_else(|| TransitionError::Transition { code: 0})?;
				// let game = Game::decode(&mut game_asset.fury_asset.as_slice()).map_err(|_e| TransitionError::Transition { code: 0})?;
				//
				// let (deck_id, deck_asset) = assets.pop().ok_or_else(|| TransitionError::Transition { code: 0})?;
				// let deck = Game::decode(&mut deck_asset.fury_asset.as_slice()).map_err(|_e| TransitionError::Transition { code: 0})?;
				//
				// if game.game_sate == GameSate::Running {
				// 	return Ok(vec![TransitionOutput::Mutated(game_id, game_asset), TransitionOutput::Mutated(deck_id, deck_asset)]);
				// }
				//
				// game.


				// let game = assets.get(1).map(|a| Game::decode(|a.1.fury_asset.as_mut()))
			// },
			TransitionIdentifier::Preparation => {
				let (game_id, game_asset) = assets.pop().ok_or_else(|| TransitionError::Transition { code: 0})?;
				let mut game = Game::decode(&mut game_asset.fury_asset.as_slice()).map_err(|_e| TransitionError::Transition { code: 0})?;

				let (deck_id, deck_asset) = assets.pop().ok_or_else(|| TransitionError::Transition { code: 0})?;
				let mut deck = Deck::decode(&mut deck_asset.fury_asset.as_slice()).map_err(|_e| TransitionError::Transition { code: 0})?;

				if game.game_sate != GameState::Running {
					return Err(TransitionError::Transition {code: 0});
				}

				if game.level_state != LevelState::Preparation {
					return Err(TransitionError::Transition {code: 0});
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

				vec![TransitionOutput::Mutated(game_id, game_asset), TransitionOutput::Mutated(deck_id, deck_asset)]
			},
			TransitionIdentifier::Battle => {
				let (game_id, game_asset) = assets.pop().ok_or_else(|| TransitionError::Transition { code: 0})?;
				let mut game = Game::decode(&mut game_asset.fury_asset.as_slice()).map_err(|_e| TransitionError::Transition { code: 0})?;

				let (deck_id, deck_asset) = assets.pop().ok_or_else(|| TransitionError::Transition { code: 0})?;
				let mut deck = Deck::decode(&mut deck_asset.fury_asset.as_slice()).map_err(|_e| TransitionError::Transition { code: 0})?;

				if game.game_sate != GameState::Running {
					return Err(TransitionError::Transition {code: 0});
				}

				if game.level_state != LevelState::Battle {
					return Err(TransitionError::Transition {code: 0});
				}

				let attack_positions = attack_positions.as_ref().ok_or_else(|| TransitionError::Transition { code: 0})?;

				let attack_cards = deck.hand.pick_multiple_cards(attack_positions)
					.map_err(|_e| TransitionError::Transition {code: 0})?;

				game.attack = Attack::create(&attack_cards).map_err(|_e| TransitionError::Transition {code: 0})?;

				game.boss.add_damage(game.attack.score);

				game.player.decrease_endurance();


				// Continue the game for as long both parties are alive
				if game.boss.is_alive() && game.player.is_alive() {
					game.level_state = LevelState::Battle;

					game.round = game.round.saturating_add(1);


					let subject = (&account_id, &game_id, &deck_id);
					let random_hash = Sage::random_hash(subject.encode().as_slice());
					deck.draw(game.player.hand_size, random_hash)
						.map_err(|e| TransitionError::Transition {code: 0})?;

				} else {
					game.level_state = LevelState::Score;
				}

				if !game.player.is_alive() || (deck.deck_size + deck.hand.cards_count()) == 0 {
					game.game_sate = GameState::Finished;
				}

				vec![TransitionOutput::Mutated(game_id, game_asset), TransitionOutput::Mutated(deck_id, deck_asset)]
			},
			TransitionIdentifier::Discard => Default::default(),
			TransitionIdentifier::Score =>Default::default(),
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
