use crate::error::FuryError;
use frame_support::pallet_prelude::{Decode, Encode, MaxEncodedLen, TypeInfo};
use sp_std::vec::Vec;

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct Tower {
	pub tower_level: u32,

	pub boons_and_banes: u32,

	pub single_boons: u32,

	pub multi_boons: u32,

	pub single_banes: u32,

	pub multi_banes: u32,
}

pub type Position = u8;
pub type Index = u8;
pub type Level = u8;

impl Tower {
	pub fn new() -> Tower {
		Default::default()
	}

	pub fn get_boon_and_bane(
		&self,
		position: Position,
	) -> Result<(BonusType, MalusType), FuryError> {
		if position > 2 {
			return Err(FuryError::InvalidBoonAndBanePosition);
		}

		let boons_and_banes = self.boons_and_banes;
		let bit_offset = (position * 2) * 6;

		let boon = BonusType::from((boons_and_banes >> bit_offset) & 0x3F);
		let bane = MalusType::from((boons_and_banes >> (bit_offset + 6)) & 0x3F);

		Ok((boon, bane))
	}

	pub fn set_boons_and_banes(
		&mut self,
		position: Position,
		boon: BonusType,
		bane: MalusType,
	) -> Result<(), FuryError> {
		if position > 2 {
			return Err(FuryError::InvalidBoonAndBanePosition);
		}

		let mut boons_and_banes = self.boons_and_banes;

		let bit_offset = (position * 2) * 6;

		// Clear existing boon and bane values at the position
		let boon_mask = 0x3F << bit_offset;
		let bane_mask = 0x3F << (bit_offset + 6);

		boons_and_banes &= !(boon_mask | bane_mask); // Clear bits

		// Set new boon and bane values
		boons_and_banes |= ((boon as u32) & 0x3F) << bit_offset;
		boons_and_banes |= ((bane as u32) & 0x3F) << (bit_offset + 6);

		self.boons_and_banes = boons_and_banes;
		Ok(())
	}

	pub fn get_single_boon(&self, boon_index: Index) -> Result<u8, FuryError> {
		if boon_index > 31 {
			return Err(FuryError::InvalidSingleBoonIndex);
		};

		Ok(((self.single_boons >> boon_index) & 1) as u8)
	}

	pub fn set_single_boon(
		&mut self,
		boon_index: Index,
		boon_value: bool,
	) -> Result<(), FuryError> {
		if boon_index > 31 {
			return Err(FuryError::InvalidSingleBoonIndex);
		};

		self.single_boons =
			(self.single_boons & !(1 << boon_index)) | ((boon_value as u32) << boon_index);
		Ok(())
	}

	pub fn get_multi_boon(&self, boon_index: Index) -> Result<u8, FuryError> {
		if boon_index > 15 {
			return Err(FuryError::InvalidMultiBoonIndex);
		};

		Ok(((self.multi_boons >> boon_index) & 3) as u8)
	}

	pub fn set_multi_boon(&mut self, boon_index: Index, boon_value: u8) -> Result<(), FuryError> {
		if boon_index > 15 {
			return Err(FuryError::InvalidMultiBoonIndex);
		};

		if boon_value > 3 {
			return Err(FuryError::InvalidMultiBoonValue);
		};

		self.multi_boons =
			(self.multi_boons & !(3 << boon_index)) | ((boon_value as u32) << boon_index);
		Ok(())
	}

	pub fn get_single_bane(&self, bane_index: Index) -> Result<u8, FuryError> {
		if bane_index > 31 {
			return Err(FuryError::InvalidSingleBaneIndex);
		};

		Ok(((self.single_banes >> bane_index) & 1) as u8)
	}

	pub fn set_single_bane(
		&mut self,
		bane_index: Index,
		bane_value: bool,
	) -> Result<(), FuryError> {
		if bane_index > 31 {
			return Err(FuryError::InvalidSingleBaneIndex);
		};

		self.single_banes =
			(self.single_banes & !(1 << bane_index)) | ((bane_value as u32) << bane_index);
		Ok(())
	}

	pub fn get_multi_bane(&self, bane_index: Index) -> Result<u8, FuryError> {
		if bane_index > 15 {
			return Err(FuryError::InvalidMultiBaneIndex);
		};

		Ok(((self.multi_banes >> bane_index) & 3) as u8)
	}

	fn set_multi_bane(&mut self, bane_index: Index, bane_value: u8) -> Result<(), FuryError> {
		if bane_index > 15 {
			return Err(FuryError::InvalidMultiBaneIndex);
		};

		if bane_value > 3 {
			return Err(FuryError::InvalidMultiBaneValue);
		};

		self.multi_banes =
			(self.multi_banes & !(3 << bane_index)) | ((bane_value as u32) << bane_index);
		Ok(())
	}

	pub fn get_all_boons(&self) -> Vec<(BonusType, Level)> {
		let mut vec: Vec<(BonusType, Level)> = Vec::new();

		for index in 0..32 {
			let value = self.get_single_boon(index).unwrap_or_default();
			if value != 0 {
				// skip BonusType::None values
				let boon = BonusType::from(index as u32);
				vec.push((boon, 1))
			}
		}

		for index in 0..16 {
			let value = self.get_multi_boon(index).unwrap_or_default();
			if value != 0 {
				// skip BonusType::None values
				let boon = BonusType::from(index as u32);
				vec.push((boon, value))
			}
		}

		vec
	}

	pub fn get_all_banes(&self) -> Vec<(MalusType, Level)> {
		let mut vec: Vec<(MalusType, Level)> = Vec::new();

		for index in 0..32 {
			let value = self.get_single_bane(index).unwrap_or_default();
			if value != 0 {
				// skip BonusType::None values
				let bane = MalusType::from(index as u32);
				vec.push((bane, 1))
			}
		}

		for index in 0..16 {
			let value = self.get_multi_bane(index).unwrap_or_default();
			if value != 0 {
				// skip BonusType::None values
				let bane = MalusType::from(index as u32);
				vec.push((bane, value))
			}
		}

		vec
	}
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum BonusType {
	None = 0,
	DeckRefill = 1,
	ExtraEndurance = 2,
	HeartHeal = 3,
	DamageBoost = 4,
	ExtraCardDraw = 5,
	FaceCardBonus = 6,
	SuitDiversityBonus = 7,
	LuckyDraw = 8,
	CriticalStrikeChance = 9,
	RapidRecovery = 10,
	ShieldOfValor = 11,
	MysticInsight = 12,
	ArcaneSurge = 13,
	RighteousFury = 14,
	BlessedAura = 15,
	FortunesFavor = 16,
	NimbleFingers = 17,
	EagleEye = 18,
	UnyieldingSpirit = 19,
	DivineIntervention = 20,
	ZealousCharge = 21,
	RelentlessAssault = 22,
	VitalStrike = 23,
	PurityOfHeart = 24,
	CelestialGuidance = 25,
	SwiftReflexes = 26,
	InspiringPresence = 27,
	Serendipity = 28,
	ArcaneWisdom = 29,
	MajesticRoar = 30,
	FortuitousWinds = 31,
	StalwartResolve = 32,
}

/// Defines different types of maluses (negative effects) that can be applied in the game.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum MalusType {
	None = 0,
	HalvedDamage = 1,
	SpadeHeal = 2,
	ReducedEndurance = 3,
	IncreasedFatigueRate = 4,
	LowerCardValue = 5,
	NumberCardPenalty = 6,
	UniformSuitPenalty = 7,
	Misfortune = 8,
	SluggishRecovery = 9,
	CursedDraw = 10,
	WeakStrike = 11,
	UnluckyTiming = 12,
	BlightedAura = 13,
	CrumblingDeck = 14,
	DiminishedInsight = 15,
	VulnerableState = 16,
	RecklessPlay = 17,
	WeakenedSpirit = 18,
	GrimFate = 19,
	SlipperyFingers = 20,
	BloodCurse = 21,
	Despair = 22,
	FracturedWill = 23,
	EnervatingPresence = 24,
	MisalignedFocus = 25,
	HeavyBurden = 26,
	StumblingStep = 27,
	DimmedVision = 28,
	FadingResolve = 29,
	ToxicMiasma = 30,
	CursedFate = 31,
	SourLuck = 32,
}

impl From<u32> for BonusType {
	fn from(value: u32) -> Self {
		match value {
			0 => BonusType::None,
			1 => BonusType::DeckRefill,
			2 => BonusType::ExtraEndurance,
			3 => BonusType::HeartHeal,
			4 => BonusType::DamageBoost,
			5 => BonusType::ExtraCardDraw,
			6 => BonusType::FaceCardBonus,
			7 => BonusType::SuitDiversityBonus,
			8 => BonusType::LuckyDraw,
			9 => BonusType::CriticalStrikeChance,
			10 => BonusType::RapidRecovery,
			11 => BonusType::ShieldOfValor,
			12 => BonusType::MysticInsight,
			13 => BonusType::ArcaneSurge,
			14 => BonusType::RighteousFury,
			15 => BonusType::BlessedAura,
			16 => BonusType::FortunesFavor,
			17 => BonusType::NimbleFingers,
			18 => BonusType::EagleEye,
			19 => BonusType::UnyieldingSpirit,
			20 => BonusType::DivineIntervention,
			21 => BonusType::ZealousCharge,
			22 => BonusType::RelentlessAssault,
			23 => BonusType::VitalStrike,
			24 => BonusType::PurityOfHeart,
			25 => BonusType::CelestialGuidance,
			26 => BonusType::SwiftReflexes,
			27 => BonusType::InspiringPresence,
			28 => BonusType::Serendipity,
			29 => BonusType::ArcaneWisdom,
			30 => BonusType::MajesticRoar,
			31 => BonusType::FortuitousWinds,
			32 => BonusType::StalwartResolve,
			_ => BonusType::None,
		}
	}
}

impl From<u32> for MalusType {
	fn from(value: u32) -> Self {
		match value {
			0 => MalusType::None,
			1 => MalusType::HalvedDamage,
			2 => MalusType::SpadeHeal,
			3 => MalusType::ReducedEndurance,
			4 => MalusType::IncreasedFatigueRate,
			5 => MalusType::LowerCardValue,
			6 => MalusType::NumberCardPenalty,
			7 => MalusType::UniformSuitPenalty,
			8 => MalusType::Misfortune,
			9 => MalusType::SluggishRecovery,
			10 => MalusType::CursedDraw,
			11 => MalusType::WeakStrike,
			12 => MalusType::UnluckyTiming,
			13 => MalusType::BlightedAura,
			14 => MalusType::CrumblingDeck,
			15 => MalusType::DiminishedInsight,
			16 => MalusType::VulnerableState,
			17 => MalusType::RecklessPlay,
			18 => MalusType::WeakenedSpirit,
			19 => MalusType::GrimFate,
			20 => MalusType::SlipperyFingers,
			21 => MalusType::BloodCurse,
			22 => MalusType::Despair,
			23 => MalusType::FracturedWill,
			24 => MalusType::EnervatingPresence,
			25 => MalusType::MisalignedFocus,
			26 => MalusType::HeavyBurden,
			27 => MalusType::StumblingStep,
			28 => MalusType::DimmedVision,
			29 => MalusType::FadingResolve,
			30 => MalusType::ToxicMiasma,
			31 => MalusType::CursedFate,
			32 => MalusType::SourLuck,
			_ => MalusType::None,
		}
	}
}
