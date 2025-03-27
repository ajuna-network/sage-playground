use frame_support::pallet_prelude::{Decode, Encode, TypeInfo};
use crate::effects::context::EffectContext;
use crate::types::deck::Deck;
use crate::types::game::Game;
use crate::types::tower::Tower;

pub trait Effect {

    fn name() -> &'static str;

    fn description() -> &'static str;

    fn add(game: Game, deck: Deck, tower: Tower, level: u8, context: EffectContext);

    fn remove(game: Game, deck: Deck, tower: Tower, level: u8, context: EffectContext);

    fn apply(game_event: GameEvent, game: &mut Game, deck: &mut Deck, tower: &mut Tower, level: u8, context: EffectContext);

}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum GameEvent
{
    None = 0,
    OnLevelStart,
    OnRoundStart,
    OnAttack,
    OnDraw,
    OnDiscard
}