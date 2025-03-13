use crate::error::FurryError;
use crate::types::game::PokerHand;

pub const COLLECTION_ID: u8 = 1;
pub const BLOCKTIME_SEC: u8 = 6;
pub const BLOCKS_PER_MINUTE: u32 = 10;
pub const BLOCKS_PER_HOUR: u32 = 60 * BLOCKS_PER_MINUTE;
pub const BLOCKS_PER_DAY: u32 = 24 * BLOCKS_PER_HOUR;

pub fn match_type(asset_type: u8, machine_subtype: u8) -> u8 {
	(asset_type << 4) | machine_subtype
}

pub fn is_straight(straight_ranks: &[u8], max_rank: u8, min_rank: u8) -> bool {
	let mut unique_ranks = [false; 14];
	let mut count = 0;

	for &rank in straight_ranks {
		if !unique_ranks[rank as usize] {
			unique_ranks[rank as usize] = true;
			count += 1;
		}
	}

	count == 5 && max_rank - min_rank == 4
}

pub fn evaluate(card_indexes: &[u8]) -> Result<(PokerHand, u16), FurryError> {
	if card_indexes.is_empty() || card_indexes.len() > 5 {
		return Err(FurryError::InvalidHandSize)
	}

	let mut rank_counts = [0u8; 14];
	let mut min_rank = 14;
	let mut max_rank = 0;
	let mut first_suit = None;
	let mut is_flush = card_indexes.len() == 5;
	let mut straight_ranks = [0u8; 5];
	let mut kicker_ranks = [0u8; 5];

	for (i, &index) in card_indexes.iter().enumerate() {
		if index > 51 {
			return Err(FurryError::InvalidHandSize)
		}
		let rank = (index % 13) + 1;
		let suit = index / 13;
		rank_counts[rank as usize] += 1;
		if rank < min_rank {
			min_rank = rank;
		}
		if rank > max_rank {
			max_rank = rank;
		}
		straight_ranks[i] = rank;
		kicker_ranks[i] = if rank == 1 { 14 } else { rank };
		if let Some(first) = first_suit {
			if suit != first {
				is_flush = false;
			}
		} else {
			first_suit = Some(suit);
		}
	}

	let is_straight = is_straight(&straight_ranks, max_rank, min_rank);
	let fours = rank_counts.iter().filter(|&&c| c == 4).count();
	let triples = rank_counts.iter().filter(|&&c| c == 3).count();
	let pairs = rank_counts.iter().filter(|&&c| c == 2).count();

	let category = if is_straight && is_flush {
		if straight_ranks.contains(&1) && straight_ranks.contains(&13) {
			PokerHand::RoyalFlush
		} else {
			PokerHand::StraightFlush
		}
	} else if fours == 1 {
		PokerHand::FourOfAKind
	} else if triples == 1 && pairs == 1 {
		PokerHand::FullHouse
	} else if is_flush {
		PokerHand::Flush
	} else if is_straight {
		PokerHand::Straight
	} else if triples == 1 {
		PokerHand::ThreeOfAKind
	} else if pairs == 2 {
		PokerHand::TwoPair
	} else if pairs == 1 {
		PokerHand::Pair
	} else {
		PokerHand::HighCard
	};

	let factor = category as u16;
	let kicker = *kicker_ranks.iter().max().unwrap() as u16;
	let score = factor * kicker + ((factor - 1) * (factor - 1) * 10);
	Ok((category, score))
}
