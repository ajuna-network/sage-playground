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

use super::fee_handler::*;
use crate::{
	configs::SageAssetId, AccountId, Balance, Balances, BlockNumber, Runtime, RuntimeEvent,
	SageAssets, SageFullHouseFury, SageFullHouseFurySeasons,
	SageRandom,
};

use ajuna_payment_handler::{TransferFungibleAssets, WithdrawCreditOrVoucher, WithdrawFungibles, WithdrawKind};
use ajuna_primitives::{
	asset_manager::{AssetFundsManager, AssetInspector, AssetManager},
	next_asset_id_provider::IncrementingAssetIdProvider,
	sage_api::SageApi,
	season_manager::{SeasonConfig, SeasonManager},
};
use game_full_house_fury::transition::{FullHouseFuryTransition, FullHouseFuryTransitionConfig, TransitionIdentifier};
use pallet_sage::*;

use frame_support::{
	parameter_types,
	traits::fungible::NativeOrWithId,
	PalletId,
};
use frame_system::pallet_prelude::BlockNumberFor;
use game_full_house_fury::{
	filter::GameFilter,
	types::{BaseAsset},
};
use sp_core::H256;
use sp_runtime::DispatchError;

#[cfg(feature = "runtime-benchmarks")]
use game_full_house_fury::{
	benchmarks::GameBenchmarkHelper,
};

pub type FullHouseFuryFeeHandler = AjunaTakeNoFeeHandler<
	SageAssetId,
	TransitionIdentifier,
	FullHouseFurySeasonId
>;

pub struct SageFullHouseFuryEngine;

/// Runtime specific sage implementation so that we don't have to
/// pass our type definitions all the time.
macro_rules! impl_runtime_sage_api {
	(
		$impl_target:ident,
		$runtime:ident,
		$sage_instance:ident,
		$season_manager:ident,
		$asset_id:ident,
		$asset:ident,
		$transition_config:ident,
	) => {
		impl_sage_api!(
			$impl_target,
			$runtime,
			$sage_instance,
			$season_manager,
			SageRandom,
			AccountId,
			$asset_id,
			$asset,
			FungiblesAssetId,
			Balance,
			BlockNumber,
			FullHouseFurySeasonId,
			$transition_config,
			H256,
		);
	};
}

// Every new game we add can simply call that macro for another sage instance to
// implement the sage api given that the other types are identical.
impl_runtime_sage_api!(
	SageFullHouseFuryEngine,
	Runtime,
	SageFullHouseFuryInstance,
	SageFullHouseFurySeasons,
	FullHouseFuryAssetId,
	FullHouseFuryAsset,
	FullHouseFuryTransitionConfig,
);

parameter_types! {
	pub const SageFullHouseFuryId: PalletId = PalletId(*b"sage/hjm");
}

pub type FullHouseFuryAssetId = game_casino_jam::asset::AssetId;
pub type FullHouseFuryAsset = BaseAsset<BlockNumberFor<Runtime>>;
pub type FullHouseFuryGameTransition =
	FullHouseFuryTransition<AccountId, BlockNumberFor<Runtime>, SageFullHouseFuryEngine>;

pub type FullHouseFuryAssetFilter = GameFilter<BlockNumberFor<Runtime>>;
#[cfg(feature = "runtime-benchmarks")]
pub type FullHouseFuryBenchmarkHelper = GameBenchmarkHelper<BlockNumberFor<Runtime>>;

type FungiblesAssetId = WithdrawKind<NativeOrWithId<SageAssetId>>;
type FullHouseFuryAssets = NativeAndAssets<SageAssets, SageAssetId>;
type TransferWithdraw =
	WithdrawCreditOrVoucher<WithdrawFungibles<AccountId, FullHouseFuryAssets>, DummyVoucherHandler>;

pub type SageFullHouseFuryInstance = pallet_sage::Instance3;
impl pallet_sage::Config<SageFullHouseFuryInstance> for Runtime {
	type PalletId = SageFullHouseFuryId;
	type SageGameTransition = FullHouseFuryGameTransition;
	type NextAssetIdProvider = IncrementingAssetIdProvider<FullHouseFuryAssetId>;
	type SeasonHandler = SageFullHouseFurySeasons;
	type FeeHandler = FullHouseFuryFeeHandler;
	type TransferFunds = TransferFungibleAssets<TransferWithdraw, FungiblesAssetId>;
	type FungiblesAssetId = FungiblesAssetId;
	type FilterHandler = FullHouseFuryAssetFilter;
	type Fungible = Balances;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = FullHouseFuryBenchmarkHelper;
}

pub type FullHouseFurySeasonId = u32;

pub type SeasonsFullHouseFuryInstance = pallet_ajuna_seasons::Instance3;
impl pallet_ajuna_seasons::Config<SeasonsFullHouseFuryInstance> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type SeasonId = FullHouseFurySeasonId;
	type AssetId = FullHouseFuryAssetId;
	type AccountHandler = SageFullHouseFury;
	type Currency = Balances;
	type WeightInfo = ();
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}