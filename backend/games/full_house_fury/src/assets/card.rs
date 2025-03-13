use crate::error::FurryError;
use frame_support::pallet_prelude::{Decode, Encode, MaxEncodedLen, TypeInfo};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct CardAsset {
	pub suit: Suit,
	pub rank: Rank,
}

impl TryFrom<u32> for CardAsset {
	type Error = FurryError;

	fn try_from(card_index: u32) -> Result<Self, Self::Error> {
		if card_index > 51 {
			return Err(FurryError::InvalidCardIndex);
		}

		Ok(Self {
			suit: Suit::try_from(card_index / 13)?,
			rank: Rank::try_from(card_index % 13 + 1)?,
		})
	}
}

impl core::fmt::Display for CardAsset {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "{}{}", self.rank.symbol(), self.suit.symbol())
	}
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum Suit {
	Clubs = 0,
	Diamonds = 1,
	Hearts = 2,
	Spades = 3,
}

impl Suit {
	fn symbol(self) -> &'static str {
		match self {
			Suit::Clubs => "♣",
			Suit::Diamonds => "♦",
			Suit::Hearts => "♥",
			Suit::Spades => "♠",
		}
	}
}

impl TryFrom<u32> for Suit {
	type Error = FurryError;

	fn try_from(value: u32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Suit::Clubs),
			1 => Ok(Suit::Diamonds),
			2 => Ok(Suit::Hearts),
			3 => Ok(Suit::Spades),
			_ => Err(FurryError::InvalidSuit),
		}
	}
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum Rank {
	Ace = 1,
	Two = 2,
	Three = 3,
	Four = 4,
	Five = 5,
	Six = 6,
	Seven = 7,
	Eight = 8,
	Nine = 9,
	Ten = 10,
	Jack = 11,
	Queen = 12,
	King = 13,
}

impl Rank {
	fn symbol(self) -> &'static str {
		match self {
			Rank::Ace => "A",
			Rank::Two => "2",
			Rank::Three => "3",
			Rank::Four => "4",
			Rank::Five => "5",
			Rank::Six => "6",
			Rank::Seven => "7",
			Rank::Eight => "8",
			Rank::Nine => "9",
			Rank::Ten => "10",
			Rank::Jack => "J",
			Rank::Queen => "Q",
			Rank::King => "K",
		}
	}
}

impl TryFrom<u32> for Rank {
	type Error = FurryError;

	fn try_from(value: u32) -> Result<Self, Self::Error> {
		match value {
			1 => Ok(Rank::Ace),
			2 => Ok(Rank::Two),
			3 => Ok(Rank::Three),
			4 => Ok(Rank::Four),
			5 => Ok(Rank::Five),
			6 => Ok(Rank::Six),
			7 => Ok(Rank::Seven),
			8 => Ok(Rank::Eight),
			9 => Ok(Rank::Nine),
			10 => Ok(Rank::Ten),
			11 => Ok(Rank::Jack),
			12 => Ok(Rank::Queen),
			13 => Ok(Rank::King),
			_ => Err(FurryError::InvalidRank),
		}
	}
}
