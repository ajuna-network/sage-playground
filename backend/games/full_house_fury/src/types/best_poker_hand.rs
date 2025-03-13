use crate::types::game::PokerHand;
use frame_support::pallet_prelude::{ConstU32, Decode, Encode, MaxEncodedLen, TypeInfo};
use sp_core::ConstU8;
use sp_runtime::BoundedVec;

#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct BestPokerHand {
	pub category: PokerHand,

	pub score: u8,

	pub positions: Vec<u8>,

	pub card_indexes: Vec<u8>,
}

impl core::fmt::Display for BestPokerHand {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"Category: {:?}, Score: {}, Positions: [{}], Cards: [{}]",
			self.category,
			self.score,
			self.positions.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", "),
			self.card_indexes.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(", ")
		)
	}
}
