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
	SageAssets, SageBattleMogs, SageBattleMogsAffiliates, SageBattleMogsSeasons,
	SageBattleMogsTournament, SageRandom,
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
use game_battle_mogs::prelude::*;
use pallet_ajuna_affiliates::traits::AffiliateUnlockRules;
use pallet_ajuna_tournament::EntityRank;
use pallet_sage::*;

use frame_support::{
	pallet_prelude::{Decode, Encode, MaxEncodedLen, TypeInfo},
	parameter_types,
	traits::fungible::NativeOrWithId,
	PalletId,
};
use frame_system::pallet_prelude::BlockNumberFor;
use game_battle_mogs::BattleMogsAction;
use sp_core::H256;
use sp_runtime::DispatchError;
use sp_std::{cmp::Ordering, prelude::*};

pub type BattleMogsFeeHandler = SageFeeHandler<
	SageAssets,
	SageAssetId,
	SageBattleMogsAffiliates,
	AffiliateMaxLevel,
	SageBattleMogsTournament,
	DummyVoucherHandler,
>;

pub struct SageBattleMogsEngine;

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
			BattleMogsSeasonId,
			$transition_config,
			H256,
		);
	};
}

// Every new game we add can simply call that macro for another sage instance to
// implement the sage api given that the other types are identical.
impl_runtime_sage_api!(
	SageBattleMogsEngine,
	Runtime,
	SageBattleMogsInstance,
	SageBattleMogsSeasons,
	BattleMogsAssetId,
	BattleMogsAssetFor,
	BattleMogsTransitionConfig,
);

pub type BattleMogsAssetId = game_battle_mogs::asset::BattleMogsId;
parameter_types! {
	pub const SageBattleMogsId: PalletId = PalletId(*b"sage/btl");
}

pub type BattleMogsAssetFor = BattleMogsAsset<BlockNumberFor<Runtime>>;
pub type BattleMogsGameTransition =
	BattleMogsTransition<AccountId, BlockNumberFor<Runtime>, SageBattleMogsEngine>;

pub type BattleMogsAssetFilter = GameFilter<BlockNumberFor<Runtime>>;
#[cfg(feature = "runtime-benchmarks")]
pub type BattleMogsBenchmarkHelper = GameBenchmarkHelper<BlockNumberFor<Runtime>>;

type FungiblesAssetId = WithdrawKind<NativeOrWithId<SageAssetId>>;
type BattleMogsAssets = NativeAndAssets<SageAssets, SageAssetId>;
type TransferWithdraw =
	WithdrawCreditOrVoucher<WithdrawFungibles<AccountId, BattleMogsAssets>, DummyVoucherHandler>;

pub type SageBattleMogsInstance = pallet_sage::Instance2;
impl pallet_sage::Config<SageBattleMogsInstance> for Runtime {
	type PalletId = SageBattleMogsId;
	type SageGameTransition = BattleMogsGameTransition;
	type NextAssetIdProvider = IncrementingAssetIdProvider<BattleMogsAssetId>;
	type SeasonHandler = SageBattleMogsSeasons;
	type FeeHandler = BattleMogsFeeHandler;
	type TransferFunds = TransferFungibleAssets<TransferWithdraw, FungiblesAssetId>;
	type FungiblesAssetId = FungiblesAssetId;
	type FilterHandler = BattleMogsAssetFilter;
	type Fungible = Balances;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = BattleMogsBenchmarkHelper;
}

pub type BattleMogsSeasonId = u32;

pub type SeasonsBattleMogsInstance = pallet_ajuna_seasons::Instance2;
impl pallet_ajuna_seasons::Config<SeasonsBattleMogsInstance> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type SeasonId = BattleMogsSeasonId;
	type AssetId = BattleMogsAssetId;
	type AccountHandler = SageBattleMogs;
	type Currency = Balances;
	type WeightInfo = ();
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}

parameter_types! {
	pub const AffiliateMaxLevel: u32 = 2;
}

pub type BattleMogsRuleIdentifier = AffiliateMethods<BattleMogsAction>;

#[cfg(feature = "runtime-benchmarks")]
pub struct AffiliatesBattleMogsBenchmarkHelper;

#[cfg(feature = "runtime-benchmarks")]
impl pallet_ajuna_affiliates::BenchmarkHelper<BattleMogsRuleIdentifier, ()>
	for AffiliatesBattleMogsBenchmarkHelper
{
	fn create_rule_id(_id: u32) -> BattleMogsRuleIdentifier {
		AffiliateMethods::StateTransition(BattleMogsAction::CreateMogwai)
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

pub type AffiliatesBattleMogsInstance = pallet_ajuna_affiliates::Instance2;
impl pallet_ajuna_affiliates::Config<AffiliatesBattleMogsInstance> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type WhitelistKey = ();
	type AccountManager = SageBattleMogs;
	type RuleIdentifier = BattleMogsRuleIdentifier;
	type AffiliateMaxLevel = AffiliateMaxLevel;
	type UnlockParameters = ();
	type AffiliatesUnlockRules = MockAffiliateUnlockRules;
	type WeightInfo = ();
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = AffiliatesBattleMogsBenchmarkHelper;
}

parameter_types! {
	pub const TournamentPalletId1: PalletId = PalletId(*b"aj/trmt1");
	pub const MinimumTournamentPhaseDuration: BlockNumber = 100;
}

pub type CasinoTournamentCategoryId = u32;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct BattleMogsEntityRanker;

impl EntityRank for BattleMogsEntityRanker {
	type EntityId = BattleMogsAssetId;
	type Entity = BattleMogsAssetFor;

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
		BattleMogsEntityRanker,
		AccountId,
		BattleMogsAssetId,
		BattleMogsAssetFor,
	> for MockTournamentBenchmarkHelper
{
	fn create_category_id(id: u32) -> CasinoTournamentCategoryId {
		id
	}

	fn create_default_tournament_config() -> pallet_ajuna_tournament::TournamentConfig<
		BlockNumberFor<Runtime>,
		Balance,
		BattleMogsEntityRanker,
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
			ranker: BattleMogsEntityRanker,
		}
	}

	fn create_entities(_: AccountId, _: u32) -> Vec<(BattleMogsAssetId, BattleMogsAssetFor)> {
		vec![]
	}
}

type TournamentBattleMogsInstance = pallet_ajuna_tournament::Instance2;
impl pallet_ajuna_tournament::Config<TournamentBattleMogsInstance> for Runtime {
	type PalletId = TournamentPalletId1;
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type TournamentCategoryId = CasinoTournamentCategoryId;
	type EntityId = BattleMogsAssetId;
	type RankedEntity = BattleMogsAssetFor;
	type EntityRanker = BattleMogsEntityRanker;
	type AccountManager = SageBattleMogs;
	type AssetManager = SageBattleMogs;
	type MinimumTournamentPhaseDuration = MinimumTournamentPhaseDuration;
	type WeightInfo = ();
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = MockTournamentBenchmarkHelper;
}
