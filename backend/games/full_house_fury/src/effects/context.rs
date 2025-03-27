use crate::types::{card::CardIndex, game::PokerHand};
use frame_support::pallet_prelude::{Decode, Encode, TypeInfo};
use sp_std::vec::Vec;

#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum EffectContext {
	Modify(ModifyContext),
	Attack(AttackContext),
	Level(u8),
	Round(u8),
	Cards(Vec<CardIndex>),
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct ModifyContext {
	pub level: u8,
	pub value: u8,
}

#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct AttackContext {
	pub poker_hand: PokerHand,
	pub score: u32,
	pub cards: Vec<CardIndex>,
}

impl AttackContext {
	pub fn new(poker_hand: PokerHand, score: u32, cards: Vec<CardIndex>) -> Self {
		Self { poker_hand, score, cards }
	}
}

pub fn round_ctx(round: u8) -> EffectContext {
	EffectContext::Round(round)
}

pub fn level_ctx(level: u8) -> EffectContext {
	EffectContext::Level(level)
}

pub fn card_ctx(cards: Vec<CardIndex>) -> EffectContext {
	EffectContext::Cards(cards)
}

impl From<AttackContext> for EffectContext {
	fn from(value: AttackContext) -> Self {
		EffectContext::Attack(value)
	}
}
