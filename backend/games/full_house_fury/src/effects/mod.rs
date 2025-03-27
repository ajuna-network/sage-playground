use crate::{
	effects::{
		boons::{heart_heal::HeartHeal, Boons},
		context::EffectContext,
		traits::{Effect, GameEvent},
	},
	types::{
		deck::Deck,
		game::Game,
		tower::{BonusType, MalusType, Tower},
	},
};

pub mod banes;
pub mod boons;
pub mod context;
mod manager;
pub mod traits;

pub enum BoonsAndBanes {
	Boons(Boons),
}

impl Effect for BoonsAndBanes {
	fn name(&self) -> &'static str {
		match self {
			BoonsAndBanes::Boons(boons) => boons.name(),
		}
	}

	fn description(&self) -> &'static str {
		match self {
			BoonsAndBanes::Boons(boons) => boons.description(),
		}
	}

	fn add(&self, game: Game, deck: Deck, tower: Tower, level: u8, context: EffectContext) {
		match self {
			BoonsAndBanes::Boons(boons) => boons.add(game, deck, tower, level, context),
		}
	}

	fn remove(&self, game: Game, deck: Deck, tower: Tower, level: u8, context: EffectContext) {
		match self {
			BoonsAndBanes::Boons(boons) => boons.remove(game, deck, tower, level, context),
		}
	}

	fn apply(
		&self,
		game_event: GameEvent,
		game: &mut Game,
		deck: &mut Deck,
		tower: &mut Tower,
		level: u8,
		context: EffectContext,
	) {
		match self {
			BoonsAndBanes::Boons(boons) =>
				boons.apply(game_event, game, deck, tower, level, context),
		}
	}
}

impl From<BonusType> for BoonsAndBanes {
	fn from(bonus: BonusType) -> Self {
		match bonus {
			BonusType::HeartHeal => BoonsAndBanes::Boons(Boons::HeartHeal(HeartHeal)),
			_ => unimplemented!("Unimplemented BonusType: {:?}", bonus),
		}
	}
}

impl From<MalusType> for BoonsAndBanes {
	fn from(malus: MalusType) -> Self {
		match malus {
			_ => unimplemented!("Unimplemented BonusType: {:?}", malus),
		}
	}
}
