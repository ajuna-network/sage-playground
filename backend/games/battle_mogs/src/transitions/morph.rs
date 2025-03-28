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
	algorithm::Breeding,
	asset::{mogwai::PhaseType, BattleMogsAsset, BattleMogsId},
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
	pub(crate) fn morph_mogwai(
		owner: &AccountId,
		mogwai_id: &BattleMogsId,
		table_id: &BattleMogsId,
		payment_asset: Option<Sage::FungiblesAssetId>,
	) -> Result<BattleMogsTransitionOutput<BlockNumber>, TransitionError> {
		let mut asset = Self::get_owned_mogwai(owner, mogwai_id)?;
		let mut table_asset = Self::get_owned_achievement_table(owner, table_id)?;
		let mogwai = asset.as_mogwai()?;
		ensure!(mogwai.phase != PhaseType::Bred, BattleMogsError::from(MOGWAI_STILL_IN_BRED_PHASE));

		let pairing_price = Pricing::<Balance>::pairing(mogwai.rarity, mogwai.rarity);
		Self::deposit_funds_to_asset(mogwai_id, owner, payment_asset, pairing_price)?;

		let block_number = Sage::get_current_block_number();
		let breed_type = BreedType::calculate_breed_type(block_number);

		let mut dx = [0u8; 16];
		dx.copy_from_slice(&mogwai.dna[0][0..16]);

		let mut dy = [0u8; 16];
		dy.copy_from_slice(&mogwai.dna[0][16..32]);

		mogwai.dna[0] = Breeding::morph(breed_type, &dx, &dy);

		let table = table_asset.as_achievement()?;
		table.morpheus = table.morpheus.increase_by(1);

		Ok(sp_std::vec![TransitionOutput::Mutated(*mogwai_id, asset)])
	}
}
