use frame_support::BoundedVec;
use sage_api::{traits::GetId, TransitionError};

use crate::error::FurryError;
use frame_support::pallet_prelude::{ConstU32, Decode, Encode, MaxEncodedLen, TypeInfo};
use sp_runtime::traits::BlockNumber as BlockNumberT;

pub type AssetId = u32;

pub const MAX_ASSET_LEN: u32 = 32;

pub mod card;
pub mod deck;
pub mod random_number_generator;

#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct BaseAsset<BlockNumber> {
	pub id: AssetId,
	pub collection_id: u8,
	pub genesis: BlockNumber,

	pub asset_type: AssetType,

	/// Encoded furry asset.
	pub furry_asset: BoundedVec<u8, ConstU32<MAX_ASSET_LEN>>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum AssetType {
	None = 0,
	Tower = 1,
	Deck = 2,
	Game = 3,
}

pub enum PokerHand {
	None = 0,
	HighCard = 1,
	Pair = 2,
	TwoPair = 3,
	ThreeOfAKind = 4,
	Straight = 5,
	Flush = 6,
	FullHouse = 7,
	FourOfAKind = 8,
	StraightFlush = 9,
	RoyalFlush = 10,
}
