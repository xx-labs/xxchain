
//! Autogenerated weights for `xx_betanet_rewards`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-11-15, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("xxnetwork-dev"), DB CACHE: 128

// Executed Command:
// ./xxnetwork-chain
// benchmark
// --chain=xxnetwork-dev
// --steps=50
// --repeat=20
// --pallet=xx_betanet_rewards
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./weights/xx_betanet_rewards.rs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{RefTimeWeight , Weight}};
use sp_std::marker::PhantomData;

/// Weight functions for `xx_betanet_rewards`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> xx_betanet_rewards::WeightInfo for WeightInfo<T> {
	// Storage: XXBetanetRewards Accounts (r:1 w:1)
	fn select_option() -> Weight {
		Weight::from_ref_time(22_071_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: XXBetanetRewards Approved (r:0 w:1)
	fn approve() -> Weight {
		Weight::from_ref_time(15_409_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
}
