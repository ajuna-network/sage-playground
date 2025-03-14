use frame_support::BoundedVec;
use sage_api::{traits::GetId};
use frame_support::pallet_prelude::{ConstU32, Decode, Encode, MaxEncodedLen, TypeInfo};

pub type AssetId = u32;

pub const MAX_ASSET_LEN: u32 = 32;

pub mod best_poker_hand;
pub mod card;
pub mod deck;
pub mod game;
pub mod random_number_generator;

#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct BaseAsset<BlockNumber> {
	pub id: AssetId,
	pub collection_id: u8,
	pub genesis: BlockNumber,

	pub asset_type: AssetType,

	/// Encoded fury asset.
	pub fury_asset: [u8; 32],
}

impl<BlockNumber> GetId<AssetId> for BaseAsset<BlockNumber> {
	fn get_id(&self) -> AssetId {
		self.id
	}
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum AssetType {
	Tower = 1,
	Deck = 2,
	Game = 3,
}
