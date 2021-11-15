
//! Autogenerated weights for `claims`
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
// --pallet=claims
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./weights/claims.rs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `claims`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> claims::WeightInfo for WeightInfo<T> {
	// Storage: Claims Claims (r:1 w:1)
	// Storage: Claims Signing (r:1 w:1)
	// Storage: Claims Total (r:1 w:1)
	// Storage: Claims Vesting (r:1 w:1)
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: System Account (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Claims Rewards (r:1 w:0)
	fn claim() -> Weight {
		(439_488_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: Claims Total (r:1 w:1)
	// Storage: Claims Vesting (r:0 w:1)
	// Storage: Claims Claims (r:0 w:1)
	// Storage: Claims Signing (r:0 w:1)
	fn mint_claim() -> Weight {
		(11_351_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Claims Claims (r:1 w:1)
	// Storage: Claims Signing (r:1 w:1)
	// Storage: Claims Total (r:1 w:1)
	// Storage: Claims Vesting (r:1 w:1)
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: System Account (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Claims Rewards (r:1 w:0)
	fn claim_attest() -> Weight {
		(447_012_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: Claims Preclaims (r:1 w:1)
	// Storage: Claims Signing (r:1 w:1)
	// Storage: Claims Claims (r:1 w:1)
	// Storage: Claims Total (r:1 w:1)
	// Storage: Claims Vesting (r:1 w:1)
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: System Account (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Claims Rewards (r:1 w:0)
	fn attest() -> Weight {
		(108_304_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	// Storage: Claims Claims (r:1 w:2)
	// Storage: Claims Vesting (r:1 w:2)
	// Storage: Claims Signing (r:1 w:2)
	// Storage: Claims Preclaims (r:1 w:1)
	fn move_claim() -> Weight {
		(24_376_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
}
