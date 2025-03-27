use crate::effects::boons::Boons;
use crate::effects::boons::heart_heal::HeartHeal;
use crate::types::tower::{BonusType, MalusType};

pub mod banes;
pub mod boons;
pub mod context;
pub mod traits;
mod manager;

pub enum BoonsAndBanes {
	Boons(Boons),
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