use crate::{
	effects::{
		boons::heart_heal::HeartHeal,
		context::EffectContext,
		traits::{Effect, GameEvent},
	},
	types::{deck::Deck, game::Game, tower::Tower},
};

pub mod heart_heal;

pub enum Boons {
	HeartHeal(HeartHeal),
}

impl Effect for Boons {
	fn name(&self) -> &'static str {
		match self {
			Boons::HeartHeal(boon) => boon.name(),
		}
	}

	fn description(&self) -> &'static str {
		match self {
			Boons::HeartHeal(boon) => boon.description(),
		}
	}

	fn add(&self, game: Game, deck: Deck, tower: Tower, level: u8, context: EffectContext) {
		match self {
			Boons::HeartHeal(boon) => boon.add(game, deck, tower, level, context),
		}
	}

	fn remove(&self, game: Game, deck: Deck, tower: Tower, level: u8, context: EffectContext) {
		match self {
			Boons::HeartHeal(boon) => boon.remove(game, deck, tower, level, context),
		}
	}

	fn apply(
		&self,
		game_event: GameEvent,
		game: &mut Game,
		deck: &mut Deck,
		tower: &mut Tower,
		level: u8,
		context: Option<EffectContext>,
	) {
		match self {
			Boons::HeartHeal(boon) => boon.apply(game_event, game, deck, tower, level, context),
		}
	}
}
