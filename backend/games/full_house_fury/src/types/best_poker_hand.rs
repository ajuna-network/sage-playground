use crate::types::game::PokerHand;
use frame_support::pallet_prelude::{Decode, Encode, TypeInfo};
use sp_std::vec::Vec;

#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct BestPokerHand {
	pub category: PokerHand,

	pub score: u8,

	pub positions: Vec<u8>,

	pub card_indexes: Vec<u8>,
}
