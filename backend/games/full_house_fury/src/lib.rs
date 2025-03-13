#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::{Decode, Encode, MaxEncodedLen, TypeInfo};

pub mod error;
pub mod types;
