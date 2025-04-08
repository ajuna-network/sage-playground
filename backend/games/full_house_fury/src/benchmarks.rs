use ajuna_primitives::payment_handler::WithdrawKind;
use sage_api::benchmarks::SageBenchmarkHelper;

use frame_support::traits::fungible::NativeOrWithId;
use parity_scale_codec::Encode;
use sp_runtime::traits::BlockNumber as BlockNumberT;
use sp_std::{marker::PhantomData, vec::Vec};
use crate::transition::TransitionIdentifier;
use crate::types::{AssetId, AssetType, BaseAsset};
use crate::types::game::Game;

pub struct GameBenchmarkHelper<BlockNumber>(PhantomData<BlockNumber>);

impl<BlockNumber>
	SageBenchmarkHelper<
		AssetId,
		BaseAsset<BlockNumber>,
		TransitionIdentifier,
		(),
		(),
		WithdrawKind<NativeOrWithId<AssetId>>,
	> for GameBenchmarkHelper<BlockNumber>
where
	BlockNumber: BlockNumberT,
{
	fn create_asset(seed: u32) -> (AssetId, BaseAsset<BlockNumber>) {
		let asset_id = AssetId::from(seed);
		let game_asset = BaseAsset {
			id: asset_id,
			collection_id: 0,
			genesis: 0u8.into(),
			asset_type: AssetType::Game,
			fury_asset: Game::start_new().encode().try_into().unwrap(),
		};

		(asset_id, game_asset)
	}

	fn create_bench_transition() -> (TransitionIdentifier, Vec<AssetId>) {
		(TransitionIdentifier::Start, Vec::with_capacity(0))
	}

	fn create_trade_filter_for(_: &BaseAsset<BlockNumber>) -> () {
		()
	}

	fn create_transfer_filter_for(_: &BaseAsset<BlockNumber>) -> () {
		()
	}

	fn create_payment_kind() -> WithdrawKind<NativeOrWithId<AssetId>> {
		WithdrawKind::Payment(NativeOrWithId::Native)
	}
}
