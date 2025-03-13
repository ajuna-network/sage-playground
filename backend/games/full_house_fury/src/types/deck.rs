use crate::{
	error::FuryError,
	types::{card::CardIndex, random_number_generator::RandomNumberGenerator},
};
use frame_support::pallet_prelude::{Decode, Encode, MaxEncodedLen, TypeInfo};
use sp_core::H256;
use sp_runtime::traits::BlakeTwo256;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct Deck {
	pub max_deck_size: u8,

	pub deck_size: u8,

	/// Each bit in the u64 represents if a card is present in the deck or not.
	pub deck: u64,

	/// Set of cards in the hand
	pub hand: Hand,
}

pub const HAND_LIMIT_SIZE: u8 = 10;
pub const HAND_EMPTY_SLOT: u8 = 63;

impl Deck {
	pub fn new_deck() -> Deck {
		Self { max_deck_size: 52, deck_size: 52, deck: u64::MAX, hand: Hand::new() }
	}

	/// Returns true if the card is in the deck, false otherwise.
	fn get_card_state(&self, card_index: u8) -> bool {
		if card_index > self.max_deck_size {
			false
		} else {
			(self.deck & (1 << card_index)) != 0
		}
	}

	/// Marks a card either as present in the deck or not.
	fn set_card_in_deck(
		&mut self,
		card_index: CardIndex,
		card_state: bool,
	) -> Result<(), FuryError> {
		if card_index > self.max_deck_size {
			return Err(FuryError::InvalidCardIndex);
		};

		if card_state {
			self.deck | (1 << card_index) // Set bit to 1
		} else {
			self.deck & !(1 << card_index) // Set bit to 0
		};

		Ok(())
	}

	/// Adds a card to the deck and returns an error if that card is already present.
	fn add_card_to_deck(&mut self, card_index: CardIndex) -> Result<(), FuryError> {
		if !self.get_card_state(card_index) {
			self.set_card_in_deck(card_index, true)?;
			self.deck_size = self.deck_size + 1;
			Ok(())
		} else {
			Err(FuryError::CardAlreadyInDeck)
		}
	}

	/// Removes a card to the deck and returns an error if that card the card is not present.
	fn remove_card_from_deck(&mut self, card_index: CardIndex) -> Result<(), FuryError> {
		if self.get_card_state(card_index) {
			self.set_card_in_deck(card_index, false)?;
			self.deck_size = self.deck_size - 1;
			Ok(())
		} else {
			Err(FuryError::CardNotInDeck)
		}
	}

	pub fn draw(&mut self, hand_size: u8, random_hash: H256) -> Result<(), FuryError> {
		if hand_size > HAND_LIMIT_SIZE {
			return Err(FuryError::InvalidHandSize)
		}

		let mut current_count = self.hand.cards_count();
		let mut rng = RandomNumberGenerator::<BlakeTwo256>::new(random_hash);

		for hand_position in 0..hand_size {
			if self.deck_size == 0 || current_count >= hand_size {
				break;
			}

			if self.hand.is_hand_position_empty(hand_position) {
				let drawn_card = self.draw_card(&mut rng)?;
				self.hand.set_hand_card(hand_position, drawn_card)?;
				current_count = current_count + 1;
			}
		}

		Ok(())
	}

	pub fn draw_card(
		&mut self,
		rng: &mut RandomNumberGenerator<BlakeTwo256>,
	) -> Result<CardIndex, FuryError> {
		if self.deck_size == 0 {
			return Err(FuryError::DeckIsEmpty)
		};

		let card_index = random_set_bit(self.deck, rng).ok_or(FuryError::DeckIsEmpty)?;

		Ok(card_index)
	}

	pub fn set_hand(&mut self, hand_position: u8, card_index: CardIndex) -> Result<(), FuryError> {
		if card_index > self.max_deck_size && card_index != HAND_EMPTY_SLOT {
			return Err(FuryError::InvalidCardIndex);
		}

		self.hand.set_hand_card(hand_position, card_index)
	}
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct Hand {
	/// Set of cards in the hand
	pub hand: u64,
}

impl Hand {
	fn new() -> Self {
		let hand = (0..HAND_LIMIT_SIZE)
			.map(|i| ((HAND_EMPTY_SLOT & 0x3F) as u64) << (i * 6))
			.fold(0, |acc, x| acc | x);

		Self { hand }
	}

	fn set_hand_card(
		&mut self,
		hand_position: u8,
		card_index: CardIndex,
	) -> Result<(), FuryError> {
		if hand_position > HAND_LIMIT_SIZE {
			return Err(FuryError::InvalidHandPosition);
		}

		let bit_offset = hand_position * 6;
		let mask = 0x3F << bit_offset; // 0x3F = 0b111111, masks 6 bits

		// Clear the existing bits at that position and insert the new value
		let new_hand = (self.hand & !mask) | ((card_index as u64 & 0x3F) << bit_offset);
		self.hand = new_hand;
		Ok(())
	}

	fn get_hand_card(&self, hand_position: u8) -> Result<u8, FuryError> {
		if hand_position > HAND_LIMIT_SIZE {
			return Err(FuryError::InvalidHandPosition);
		}

		let hand_value = self.hand;
		let bit_offset = hand_position * 6;
		Ok((hand_value >> bit_offset & 0x3F) as u8)
	}

	fn cards_count(&self) -> u8 {
		let mut count = 0;
		for hand_position in 0..HAND_LIMIT_SIZE {
			if self.is_hand_position_occupied(hand_position) {
				count += 1;
			}
		}

		count
	}

	/// Swallows `hand_position` out of bounds errors and defaults to false in that case.
	fn is_hand_position_empty(&self, hand_position: u8) -> bool {
		self.get_hand_card(hand_position)
			.map(|card_index| card_index == HAND_EMPTY_SLOT)
			.unwrap_or(true)
	}

	/// Swallows `hand_position` out of bounds errors and defaults to true in that case.
	fn is_hand_position_occupied(&self, hand_position: u8) -> bool {
		!self.is_hand_position_empty(hand_position)
	}
}

/// Choose a random set bit (1) of the lower 52 bytes.
fn random_set_bit(value: u64, rng: &mut RandomNumberGenerator<BlakeTwo256>) -> Option<u8> {
	let masked = value & ((1 << 52) - 1); // Mask out only the first 52 bits
	let count = masked.count_ones(); // Count set bits

	if count == 0 {
		return None; // No set bits
	}

	let target = rng.pick_u32(count); // Choose a random set bit index

	let mut value = masked;
	let mut selected_bit = 0;
	for _ in 0..=target {
		selected_bit = value.trailing_zeros() as u8;
		value &= value - 1; // Clear the lowest set bit
	}

	Some(selected_bit)
}
