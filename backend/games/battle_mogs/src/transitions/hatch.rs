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
	asset::{
		mogwai::{Mogwai, PhaseType, RarityType},
		BattleMogsAsset, BattleMogsId,
	},
	config::GameEventType,
	error::*,
	transitions::{BattleMogsTransitionConfig, BattleMogsTransitionOutput},
	BattleMogsTransition,
};

use ajuna_primitives::sage_api::SageApi;
use sage_api::{traits::TransitionOutput, TransitionError};

use frame_support::pallet_prelude::*;
use parity_scale_codec::Codec;
use sp_core::H256;
use sp_runtime::{
	traits::{AtLeast32BitUnsigned, BlockNumber as BlockNumberT, Member},
	SaturatedConversion,
};

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
	pub(crate) fn hatch_mogwai(
		owner: &AccountId,
		mogwai_id: &BattleMogsId,
		table_id: &BattleMogsId,
	) -> Result<BattleMogsTransitionOutput<BlockNumber>, TransitionError> {
		let mut asset = Self::get_owned_mogwai(owner, mogwai_id)?;
		let mut table_asset = Self::get_owned_achievement_table(owner, table_id)?;

		let block_number = Sage::get_current_block_number();
		let time_till_hatch = GameEventType::time_till(GameEventType::Hatch);
		ensure!(
			block_number.saturating_sub(asset.genesis) >= time_till_hatch.saturated_into(),
			BattleMogsError::from(ASSET_COULD_NOT_RECEIVE_FUNDS)
		);

		let mogwai = asset.as_mogwai()?;
		ensure!(mogwai.phase == PhaseType::Bred, BattleMogsError::from(MOGWAI_NOT_IN_BRED_PHASE));

		// `block_hash` is static for the duration of one block per unique owner, mogwai_id pair.
		let subject = (owner, mogwai_id, b"mogwai_hatch").encode();
		let random_hash = Sage::random_hash(&subject).0;
		let (dna, rarity) = Self::segment_and_bake(mogwai, random_hash);

		mogwai.phase = PhaseType::Hatched;
		mogwai.rarity = rarity;
		mogwai.dna = dna;

		let table = table_asset.as_achievement()?;
		table.egg_hatcher = table.egg_hatcher.increase_by(1);

		Ok(sp_std::vec![
			TransitionOutput::Mutated(*mogwai_id, asset),
			TransitionOutput::Mutated(*table_id, table_asset)
		])
	}

	fn segment_and_bake(mogwai: &mut Mogwai, hash: [u8; 32]) -> ([[u8; 32]; 2], RarityType) {
		(Breeding::segmenting(mogwai.dna, hash), Breeding::bake(mogwai.rarity, hash))
	}
}
