use crate::types::{card::CardIndex, game::PokerHand};
use frame_support::pallet_prelude::{Decode, Encode, TypeInfo};
use sp_std::vec::Vec;

#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum EffectContext {
	Modify(ModifyContext),
	Attack(AttackContext),
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct ModifyContext {
	pub level: u8,
	pub value: u8,
}

#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct AttackContext {
	pub poker_hand: PokerHand,
	pub score: u16,
	pub cards: Vec<CardIndex>,
}
