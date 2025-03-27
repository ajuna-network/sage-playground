use crate::effects::boons::Boons;

pub mod banes;
pub mod boons;
pub mod context;
pub mod traits;

pub enum BoonsAndBanes {
	Boons(Boons),
}
