use crate::{
	effects::{
		context::EffectContext,
		traits::{Effect, GameEvent},
		BoonsAndBanes,
	},
	types::{
		deck::Deck,
		game::Game,
		tower::{Level, Tower},
	},
};
use sp_std::vec::Vec;

pub struct FxManager<E: Effect> {
	effects: Vec<(E, Level)>,
}

impl FxManager<BoonsAndBanes> {
	pub fn new(tower: Tower) -> Self {
		let mut effects = Vec::default();
		for (boon, level) in tower.get_all_boons().into_iter() {
			let effect = BoonsAndBanes::from(boon);
			effects.push((effect, level));
		}

		for (banes, level) in tower.get_all_banes().into_iter() {
			let effect = BoonsAndBanes::from(banes);
			effects.push((effect, level));
		}

		Self { effects }
	}

	pub fn trigger_event(
		&self,
		game_event: GameEvent,
		game: &mut Game,
		deck: &mut Deck,
		tower: &mut Tower,
		context: Option<EffectContext>,
	) {
		for (effect, level) in self.effects.iter() {
			effect.apply(game_event, game, deck, tower, *level, context.clone());
		}
	}
}
