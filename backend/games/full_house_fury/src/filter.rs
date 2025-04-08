use ajuna_primitives::trade_manager::*;

use crate::types::BaseAsset;
use sp_runtime::traits::BlockNumber as BlockNumberT;
use sp_std::marker::PhantomData;

#[derive(Default)]
pub struct GameFilter<BlockNumber>(PhantomData<BlockNumber>);

impl<BlockNumber> TradeManager for GameFilter<BlockNumber>
where
	BlockNumber: BlockNumberT,
{
	type TradeFilter = ();
	type Asset = BaseAsset<BlockNumber>;

	fn can_be_traded_using(_: &Self::Asset, _: &Self::TradeFilter) -> bool {
		true
	}
}

impl<BlockNumber> TransferManager for GameFilter<BlockNumber>
where
	BlockNumber: BlockNumberT,
{
	type TransferFilter = ();
	type Asset = BaseAsset<BlockNumber>;

	fn can_be_transferred_using(_: &Self::Asset, _: &Self::TransferFilter) -> bool {
		true
	}
}
