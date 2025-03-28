// Ajuna Node
// Copyright (C) 2022 BlogaTech AG

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use crate::{
	algorithm::{Breeding, Generation},
	asset::{
		mogwai::{Mogwai as MogwaiVariant, PhaseType, RarityType},
		BattleMogsAsset, BattleMogsId, BattleMogsVariant,
	},
	config::Pricing,
	error::*,
	transitions::{BattleMogsTransitionConfig, BattleMogsTransitionOutput, BreedType},
	BattleMogsTransition,
};

use ajuna_primitives::sage_api::SageApi;
use sage_api::{traits::TransitionOutput, TransitionError};

use frame_support::pallet_prelude::*;
use parity_scale_codec::Codec;
use sp_core::H256;
use sp_runtime::traits::{AtLeast32BitUnsigned, BlockNumber as BlockNumberT, Member};

impl<AccountId, BlockNumber, Balance, Sage> BattleMogsTransition<AccountId, BlockNumber, Sage>
where
	AccountId: Member + Codec,
	BlockNumber: BlockNumberT,
	Balance: Member + Parameter + AtLeast32BitUnsigned + MaxEncodedLen,
	Sage: SageApi<
		AccountId = AccountId,
		AssetId = BattleMogsId,
		Asset = BattleMogsAsset<BlockNumber>,
		Balance = Balance,
		BlockNumber = BlockNumber,
		TransitionConfig = BattleMogsTransitionConfig,
		HashOutput = H256,
	>,
{
	pub(crate) fn breed_mogwais(
		owner: &AccountId,
		mogwai_id_1: &BattleMogsId,
		mogwai_id_2: &BattleMogsId,
		table_id: &BattleMogsId,
		payment_asset: Option<Sage::FungiblesAssetId>,
	) -> Result<BattleMogsTransitionOutput<BlockNumber>, TransitionError> {
		ensure!(
			mogwai_id_1 != mogwai_id_2,
			BattleMogsError::from(CANNOT_USE_SAME_ASSET_FOR_BREEDING),
		);
		Self::ensure_not_max_mogwais(owner)?;

		let mut asset_1 = Self::get_owned_mogwai(owner, mogwai_id_1)?;
		let mogwai_1 = asset_1.as_mogwai()?;
		ensure!(
			mogwai_1.phase != PhaseType::Bred,
			BattleMogsError::from(MOGWAI_STILL_IN_BRED_PHASE)
		);

		let mut asset_2 = Self::get_mogwai(mogwai_id_2)?;
		let mogwai_2 = asset_2.as_mogwai()?;
		ensure!(
			mogwai_2.phase != PhaseType::Bred,
			BattleMogsError::from(MOGWAI_STILL_IN_BRED_PHASE)
		);

		let mut table_asset = Self::get_owned_achievement_table(owner, table_id)?;

		let mogwai_id = Self::new_asset_id()?;

		// `next_gen_hash` is static for the duration of one block per unique
		// owner/mogwai_id pair.
		let subject = (owner, mogwai_id, b"breed_next_gen").encode();
		let next_gen_hash = Sage::random_hash(&subject).0;

		let (rarity, next_gen, max_rarity) = Generation::next_gen(
			mogwai_1.generation,
			mogwai_1.rarity,
			mogwai_2.generation,
			mogwai_2.rarity,
			&next_gen_hash,
		);

		let block_number = Sage::get_current_block_number();
		let breed_type = BreedType::calculate_breed_type(block_number);

		let pairing_price = Pricing::<Balance>::pairing(mogwai_1.rarity, mogwai_2.rarity);
		Self::deposit_funds_to_asset(mogwai_id_2, owner, payment_asset, pairing_price)?;

		let final_dna = Breeding::pairing(breed_type, &mogwai_1.dna[0], &mogwai_2.dna[0]);
		let mogwai_rarity = RarityType::from(((max_rarity as u8) << 4) + rarity as u8);

		let bred_mogwai =
			MogwaiVariant { dna: final_dna, generation: next_gen, rarity, phase: PhaseType::Bred };

		let bred_asset = BattleMogsAsset {
			id: mogwai_id,
			genesis: block_number,
			variant: BattleMogsVariant::Mogwai(bred_mogwai),
		};

		if mogwai_rarity == RarityType::Mythical {
			let table = table_asset.as_achievement()?;
			table.legend_breeder = table.legend_breeder.increase_by(1);
		}

		let is_mogwai_2_owned = Sage::ensure_ownership(owner, mogwai_id_2).is_ok();
		if !is_mogwai_2_owned {
			let table = table_asset.as_achievement()?;
			table.promiscuous = table.promiscuous.increase_by(1);
		}

		Ok(sp_std::vec![TransitionOutput::Minted(bred_asset)])
	}
}
