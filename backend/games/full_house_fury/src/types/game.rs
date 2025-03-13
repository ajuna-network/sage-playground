use crate::{error::FurryError, types::card::CardIndex};
use frame_support::pallet_prelude::{Decode, Encode, MaxEncodedLen, TypeInfo};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct Game {
	pub game_sate: GameSate,

	pub level_state: LevelState,

	pub level: u8,

	pub round: u8,

	pub boss: Boss,

	pub attack: Attack,

	pub player: Player,
}

impl Game {
	fn new() -> Self {
		Self {
			game_sate: GameSate::Running,
			level_state: LevelState::Preparation,
			level: 1,
			round: 0,
			boss: Boss::new(100),
			attack: Attack::default(),
			player: Player {
				max_player_endurance: 10,
				player_endurance: 10,
				discard: 3,
				hand_size: 7,
				max_player_health: 100,
				player_damage: 0,
				fatigue_damage: 1,
			},
		}
	}

	fn clear_attack(&mut self) {
		self.attack = Attack::default();
	}
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct Boss {
	pub max_health: u32,
	pub damage: u32,
}

impl Boss {
	pub fn new(max_health: u32) -> Self {
		Self { max_health, damage: 0 }
	}

	pub fn health(&self) -> u32 {
		self.max_health.saturating_sub(self.damage)
	}

	pub fn is_alive(&self) -> bool {
		self.health() > 0
	}
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct Player {
	pub max_player_endurance: u8,

	pub player_endurance: u8,

	pub discard: u8,

	pub hand_size: u8,

	pub max_player_health: u32,

	pub player_damage: u32,

	pub fatigue_damage: u32,
}

impl Player {
	pub fn health(&self) -> u32 {
		self.max_player_health.saturating_sub(self.player_damage)
	}

	pub fn is_alive(&self) -> bool {
		self.health() > 0
	}
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct Attack {
	pub hand: u32,
	pub attack_type: Option<PokerHand>,
	pub attack_score: u32,
}

impl Attack {
	pub fn get_card(&self, hand_position: u8) -> Result<CardIndex, FurryError> {
		if hand_position > 4 {
			return Err(FurryError::InvalidHandPosition);
		}

		let bit_offset = hand_position * 6;
		Ok(((self.hand >> bit_offset) & 0x3F) as CardIndex)
	}

	pub fn set_card(&mut self, hand_position: u8, card_index: CardIndex) -> Result<(), FurryError> {
		if hand_position > 4 {
			return Err(FurryError::InvalidHandPosition);
		}

		if card_index > 51 {
			return Err(FurryError::InvalidHandPosition);
		}

		let bit_offset = hand_position * 6;
		let mask = 0x3F << bit_offset; // 0x3F = 0b111111, masks 6 bits

		// Clear the existing bits at that position and insert the new value
		let new_hand = (self.hand & !mask) | ((card_index as u32 & 0x3F) << bit_offset);
		self.hand = new_hand;

		Ok(())
	}
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum GameSate {
	None = 0,
	Running = 1,
	Finished = 2,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum LevelState {
	None = 0,
	Preparation = 1,
	Battle = 2,
	Score = 3,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum PokerHand {
	HighCard = 1,
	Pair = 2,
	TwoPair = 3,
	ThreeOfAKind = 4,
	Straight = 5,
	Flush = 6,
	FullHouse = 7,
	FourOfAKind = 8,
	StraightFlush = 9,
	RoyalFlush = 10,
}
