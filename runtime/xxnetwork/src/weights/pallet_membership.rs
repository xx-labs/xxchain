
//! Autogenerated weights for `pallet_membership`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-24, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `workstation`, CPU: `AMD Ryzen 9 5900X 12-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/xxnetwork-chain
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_membership
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./weights/pallet_membership.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_membership`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_membership::WeightInfo for WeightInfo<T> {
	/// Storage: TechnicalMembership Members (r:1 w:1)
	/// Proof: TechnicalMembership Members (max_values: Some(1), max_size: Some(321), added: 816, mode: MaxEncodedLen)
	/// Storage: TechnicalCommittee Proposals (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Members (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[1, 9]`.
	fn add_member(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `137 + m * (64 ±0)`
		//  Estimated: `3699 + m * (192 ±0)`
		// Minimum execution time: 11_852_000 picoseconds.
		Weight::from_parts(12_371_234, 0)
			.saturating_add(Weight::from_parts(0, 3699))
			// Standard Error: 2_527
			.saturating_add(Weight::from_parts(91_121, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 192).saturating_mul(m.into()))
	}
	/// Storage: TechnicalMembership Members (r:1 w:1)
	/// Proof: TechnicalMembership Members (max_values: Some(1), max_size: Some(321), added: 816, mode: MaxEncodedLen)
	/// Storage: TechnicalCommittee Proposals (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalMembership Prime (r:1 w:0)
	/// Proof: TechnicalMembership Prime (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: TechnicalCommittee Members (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[2, 10]`.
	fn remove_member(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `241 + m * (64 ±0)`
		//  Estimated: `5531 + m * (192 ±0)`
		// Minimum execution time: 13_976_000 picoseconds.
		Weight::from_parts(14_526_356, 0)
			.saturating_add(Weight::from_parts(0, 5531))
			// Standard Error: 2_593
			.saturating_add(Weight::from_parts(71_843, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 192).saturating_mul(m.into()))
	}
	/// Storage: TechnicalMembership Members (r:1 w:1)
	/// Proof: TechnicalMembership Members (max_values: Some(1), max_size: Some(321), added: 816, mode: MaxEncodedLen)
	/// Storage: TechnicalCommittee Proposals (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalMembership Prime (r:1 w:0)
	/// Proof: TechnicalMembership Prime (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: TechnicalCommittee Members (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[2, 10]`.
	fn swap_member(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `241 + m * (64 ±0)`
		//  Estimated: `5531 + m * (192 ±0)`
		// Minimum execution time: 14_106_000 picoseconds.
		Weight::from_parts(14_782_137, 0)
			.saturating_add(Weight::from_parts(0, 5531))
			// Standard Error: 4_253
			.saturating_add(Weight::from_parts(59_548, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 192).saturating_mul(m.into()))
	}
	/// Storage: TechnicalMembership Members (r:1 w:1)
	/// Proof: TechnicalMembership Members (max_values: Some(1), max_size: Some(321), added: 816, mode: MaxEncodedLen)
	/// Storage: TechnicalCommittee Proposals (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalMembership Prime (r:1 w:0)
	/// Proof: TechnicalMembership Prime (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: TechnicalCommittee Members (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[1, 10]`.
	fn reset_member(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `241 + m * (64 ±0)`
		//  Estimated: `5528 + m * (192 ±0)`
		// Minimum execution time: 13_465_000 picoseconds.
		Weight::from_parts(14_201_159, 0)
			.saturating_add(Weight::from_parts(0, 5528))
			// Standard Error: 3_820
			.saturating_add(Weight::from_parts(197_127, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 192).saturating_mul(m.into()))
	}
	/// Storage: TechnicalMembership Members (r:1 w:1)
	/// Proof: TechnicalMembership Members (max_values: Some(1), max_size: Some(321), added: 816, mode: MaxEncodedLen)
	/// Storage: TechnicalCommittee Proposals (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalMembership Prime (r:1 w:1)
	/// Proof: TechnicalMembership Prime (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: TechnicalCommittee Members (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[1, 10]`.
	fn change_key(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `241 + m * (64 ±0)`
		//  Estimated: `5528 + m * (192 ±0)`
		// Minimum execution time: 14_547_000 picoseconds.
		Weight::from_parts(15_080_410, 0)
			.saturating_add(Weight::from_parts(0, 5528))
			// Standard Error: 2_548
			.saturating_add(Weight::from_parts(87_883, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(Weight::from_parts(0, 192).saturating_mul(m.into()))
	}
	/// Storage: TechnicalMembership Members (r:1 w:0)
	/// Proof: TechnicalMembership Members (max_values: Some(1), max_size: Some(321), added: 816, mode: MaxEncodedLen)
	/// Storage: TechnicalMembership Prime (r:0 w:1)
	/// Proof: TechnicalMembership Prime (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[1, 10]`.
	fn set_prime(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `32 + m * (32 ±0)`
		//  Estimated: `1838 + m * (32 ±0)`
		// Minimum execution time: 5_350_000 picoseconds.
		Weight::from_parts(5_652_138, 0)
			.saturating_add(Weight::from_parts(0, 1838))
			// Standard Error: 1_147
			.saturating_add(Weight::from_parts(25_765, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(m.into()))
	}
	/// Storage: TechnicalMembership Prime (r:0 w:1)
	/// Proof: TechnicalMembership Prime (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[1, 10]`.
	fn clear_prime(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_365_000 picoseconds.
		Weight::from_parts(2_599_310, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 818
			.saturating_add(Weight::from_parts(5_551, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
