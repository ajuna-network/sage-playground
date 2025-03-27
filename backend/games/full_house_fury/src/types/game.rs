use crate::{error::FuryError, types::card::CardIndex, utils::evaluate};
use frame_support::pallet_prelude::{Decode, Encode, MaxEncodedLen, TypeInfo};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct Game {
	pub game_sate: GameState,

	pub level_state: LevelState,

	pub level: u8,

	pub round: u8,

	pub boss: Boss,

	pub attack: Attack,

	pub player: Player,
}

impl Game {
	pub fn start_new() -> Self {
		Self {
			game_sate: GameState::Running,
			level_state: LevelState::Preparation,
			level: 1,
			round: 0,
			boss: Boss::new(100),
			attack: Attack::default(),
			player: Player {
				max_endurance: 10,
				endurance: 10,
				discard: 3,
				hand_size: 7,
				max_player_health: 100,
				player_damage: 0,
				fatigue_damage: 1,
			},
		}
	}

	pub fn clear_attack(&mut self) {
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

	pub fn add_damage(&mut self, damage: u32) {
		self.damage = self.damage.saturating_add(damage);
	}

	pub fn health(&self) -> u32 {
		self.max_health.saturating_sub(self.damage)
	}

	pub fn is_alive(&self) -> bool {
		self.health() > 0
	}
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct Player {
	pub max_endurance: u8,

	pub endurance: u8,

	pub discard: u8,

	pub hand_size: u8,

	pub max_player_health: u32,

	pub player_damage: u32,

	pub fatigue_damage: u32,
}

impl Player {
	pub fn add_damage(&mut self, damage: u32) {
		self.player_damage = self.player_damage.saturating_add(damage);
	}

	pub fn reset_endurance(&mut self) {
		self.endurance = self.max_endurance;
	}

	pub fn decrease_endurance(&mut self) {
		if self.endurance > 0 {
			self.endurance = self.endurance - 1;
		} else {
			self.add_damage(self.fatigue_damage);
			self.increase_fatigue();
		}
	}

	pub fn increase_fatigue(&mut self) {
		self.fatigue_damage = self.fatigue_damage.saturating_mul(2);
	}

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
	pub score: u32,
}

impl Attack {
	pub fn create(cards: &[u8]) -> Result<Self, FuryError> {
		if cards.len() > 5 {
			return Err(FuryError::InvalidHandSize);
		}

		let mut attack = Attack::default();

		for (i, c) in cards.iter().enumerate() {
			attack.set_card(i as u8, *c)?
		}

		let (poker_hand, score) = evaluate(cards)?;

		attack.attack_type = Some(poker_hand);
		attack.score = score as u32;

		Ok(attack)
	}

	pub fn get_card(&self, hand_position: u8) -> Result<CardIndex, FuryError> {
		if hand_position > 4 {
			return Err(FuryError::InvalidHandPosition);
		}

		let bit_offset = hand_position * 6;
		Ok(((self.hand >> bit_offset) & 0x3F) as CardIndex)
	}

	pub fn set_card(&mut self, hand_position: u8, card_index: CardIndex) -> Result<(), FuryError> {
		if hand_position > 4 {
			return Err(FuryError::InvalidHandPosition);
		}

		if card_index > 51 {
			return Err(FuryError::InvalidHandPosition);
		}

		let bit_offset = hand_position * 6;
		let mask = 0x3F << bit_offset; // 0x3F = 0b111111, masks 6 bits

		// Clear the existing bits at that position and insert the new value
		let new_hand = (self.hand & !mask) | ((card_index as u32 & 0x3F) << bit_offset);
		self.hand = new_hand;

		Ok(())
	}
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum GameState {
	#[default]
	None = 0,
	Running = 1,
	Finished = 2,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum LevelState {
	#[default]
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
