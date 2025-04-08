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
	SageAssets, SageFullHouseFury, SageFullHouseFuryAffiliates, SageFullHouseFurySeasons,
	SageFullHouseFuryTournament, SageRandom,
};

use ajuna_payment_handler::{
	TransferFungibleAssets, WithdrawCreditOrVoucher, WithdrawFungibles, WithdrawKind,
};
use ajuna_primitives::{
	asset_manager::{AssetFundsManager, AssetInspector, AssetManager},
	next_asset_id_provider::IncrementingAssetIdProvider,
	sage_api::SageApi,
	season_manager::{SeasonConfig, SeasonManager},
};
use pallet_ajuna_affiliates::traits::AffiliateUnlockRules;
use pallet_ajuna_tournament::EntityRank;
use pallet_sage::*;
use game_full_house_fury::transition::{FullHouseFuryTransition, TransitionIdentifier};
use game_full_house_fury::transition::FullHouseFuryTransitionConfig;

use frame_support::{
	pallet_prelude::{Decode, Encode, MaxEncodedLen, TypeInfo},
	parameter_types,
	traits::fungible::NativeOrWithId,
	PalletId,
};
use frame_system::pallet_prelude::BlockNumberFor;
use sp_core::H256;
use sp_runtime::DispatchError;
use sp_std::{cmp::Ordering, prelude::*};
use game_full_house_fury::benchmarks::GameBenchmarkHelper;
use game_full_house_fury::filter::GameFilter;
use game_full_house_fury::types::{AssetId, BaseAsset};

pub type FullHouseFuryFeeHandler = SageFeeHandler<
	SageAssets,
	SageAssetId,
	SageFullHouseFuryAffiliates,
	AffiliateMaxLevel,
	SageFullHouseFuryTournament,
	DummyVoucherHandler,
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

parameter_types! {
	pub const AffiliateMaxLevel: u32 = 2;
}

pub type FullHouseFuryRuleIdentifier = AffiliateMethods<TransitionIdentifier>;

#[cfg(feature = "runtime-benchmarks")]
pub struct AffiliatesFullHouseFuryBenchmarkHelper;

#[cfg(feature = "runtime-benchmarks")]
impl pallet_ajuna_affiliates::BenchmarkHelper<FullHouseFuryRuleIdentifier, ()>
for AffiliatesFullHouseFuryBenchmarkHelper
{
	fn create_rule_id(_id: u32) -> FullHouseFuryRuleIdentifier {
		AffiliateMethods::StateTransition(TransitionIdentifier::Start)
	}

	fn create_params(_id: u32) {}
}

pub struct MockAffiliateUnlockRules;

impl AffiliateUnlockRules for MockAffiliateUnlockRules {
	type AccountId = AccountId;
	type UnlockParameters = ();

	fn execute_unlock_rule_for(
		_account: &Self::AccountId,
		_params: Self::UnlockParameters,
	) -> Result<(), DispatchError> {
		Ok(())
	}
}

pub type AffiliatesFullHouseFuryInstance = pallet_ajuna_affiliates::Instance3;
impl pallet_ajuna_affiliates::Config<AffiliatesFullHouseFuryInstance> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type WhitelistKey = ();
	type AccountManager = SageFullHouseFury;
	type RuleIdentifier = FullHouseFuryRuleIdentifier;
	type AffiliateMaxLevel = AffiliateMaxLevel;
	type UnlockParameters = ();
	type AffiliatesUnlockRules = MockAffiliateUnlockRules;
	type WeightInfo = ();
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = AffiliatesFullHouseFuryBenchmarkHelper;
}

parameter_types! {
	pub const TournamentPalletId1: PalletId = PalletId(*b"aj/trmt1");
	pub const MinimumTournamentPhaseDuration: BlockNumber = 100;
}

pub type CasinoTournamentCategoryId = u32;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct FullHouseFuryEntityRanker;

impl EntityRank for FullHouseFuryEntityRanker {
	type EntityId = AssetId;
	type Entity = FullHouseFuryAsset;

	fn can_rank(&self, _: (&Self::EntityId, &Self::Entity)) -> bool {
		true
	}

	fn rank_against(
		&self,
		_: (&Self::EntityId, &Self::Entity),
		_: (&Self::EntityId, &Self::Entity),
	) -> Ordering {
		Ordering::Equal
	}
}

#[cfg(feature = "runtime-benchmarks")]
pub struct MockTournamentBenchmarkHelper;

#[cfg(feature = "runtime-benchmarks")]
impl
pallet_ajuna_tournament::BenchmarkHelper<
	CasinoTournamentCategoryId,
	BlockNumberFor<Runtime>,
	Balance,
	FullHouseFuryEntityRanker,
	AccountId,
	FullHouseFuryAssetId,
	FullHouseFuryAsset,
> for MockTournamentBenchmarkHelper
{
	fn create_category_id(id: u32) -> CasinoTournamentCategoryId {
		id
	}

	fn create_default_tournament_config() -> pallet_ajuna_tournament::TournamentConfig<
		BlockNumberFor<Runtime>,
		Balance,
		FullHouseFuryEntityRanker,
	> {
		pallet_ajuna_tournament::TournamentConfig {
			start: 0_u32,
			active_end: 100_u32,
			claim_end: 200_u32,
			initial_reward: None,
			max_reward: None,
			take_fee_percentage: None,
			reward_distribution: Default::default(),
			golden_duck_config: Default::default(),
			max_players: 0,
			ranker: FullHouseFuryEntityRanker,
		}
	}

	fn create_entities(_: AccountId, _: u32) -> Vec<(FullHouseFuryAssetId, FullHouseFuryAsset)> {
		vec![]
	}
}

type TournamentFullHouseFuryInstance = pallet_ajuna_tournament::Instance3;
impl pallet_ajuna_tournament::Config<TournamentFullHouseFuryInstance> for Runtime {
	type PalletId = TournamentPalletId1;
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type TournamentCategoryId = CasinoTournamentCategoryId;
	type EntityId = FullHouseFuryAssetId;
	type RankedEntity = FullHouseFuryAsset;
	type EntityRanker = FullHouseFuryEntityRanker;
	type AccountManager = SageFullHouseFury;
	type AssetManager = SageFullHouseFury;
	type MinimumTournamentPhaseDuration = MinimumTournamentPhaseDuration;
	type WeightInfo = ();
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = MockTournamentBenchmarkHelper;
}
