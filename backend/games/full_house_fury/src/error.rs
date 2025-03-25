use sage_api::TransitionError;
use sp_core::Encode;
use sp_core::Decode;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Encode, Decode, PartialOrd, Ord)]
pub enum FuryError {
	InvalidSuit = 10,
	InvalidRank = 11,
	InvalidCardIndex = 12,
	CardAlreadyInDeck = 13,
	CardNotInDeck = 14,
	DeckIsEmpty = 15,
	InvalidHandPosition = 16,
	InvalidHandSize = 17,
	HandSlotIsEmpty = 18,
	TooManyCardsPicked = 19,
	GameNotInRunningPhase = 20,
	LevelStateIsNotPreparation = 21,
	LevelStateIsNotBattle = 22,
	NoHandPositionsFound = 23,
	DiscardLimitReached = 24,
}

impl FuryError {
	pub fn index(&self) -> u8 {
		*self as u8
	}
}

impl From<FuryError> for TransitionError {
	fn from(err: FuryError) -> Self {
		TransitionError::Transition { code: err.index() }
	}
}