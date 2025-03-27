use crate::{
	effects::context::EffectContext,
	types::{deck::Deck, game::Game, tower::Tower},
};
use frame_support::pallet_prelude::{Decode, Encode, TypeInfo};

pub trait Effect {
	fn name(&self) -> &'static str;

	fn description(&self) -> &'static str;

	fn add(&self, game: Game, deck: Deck, tower: Tower, level: u8, context: EffectContext);

	fn remove(&self, game: Game, deck: Deck, tower: Tower, level: u8, context: EffectContext);

	fn apply(
		&self,
		game_event: GameEvent,
		game: &mut Game,
		deck: &mut Deck,
		tower: &mut Tower,
		level: u8,
		context: EffectContext,
	);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum GameEvent {
	None = 0,
	OnLevelStart,
	OnRoundStart,
	OnAttack,
	OnDraw,
	OnDiscard,
}
