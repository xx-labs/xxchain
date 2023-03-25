
//! Autogenerated weights for `pallet_bags_list`
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
// --pallet=pallet_bags_list
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./weights/pallet_bags_list.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_bags_list`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_bags_list::WeightInfo for WeightInfo<T> {
	/// Storage: Staking Bonded (r:1 w:0)
	/// Proof: Staking Bonded (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Staking Ledger (r:1 w:0)
	/// Proof: Staking Ledger (max_values: None, max_size: Some(1124), added: 3599, mode: MaxEncodedLen)
	/// Storage: VoterList ListNodes (r:4 w:4)
	/// Proof: VoterList ListNodes (max_values: None, max_size: Some(154), added: 2629, mode: MaxEncodedLen)
	/// Storage: VoterList ListBags (r:1 w:1)
	/// Proof: VoterList ListBags (max_values: None, max_size: Some(82), added: 2557, mode: MaxEncodedLen)
	fn rebag_non_terminal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1714`
		//  Estimated: `23179`
		// Minimum execution time: 47_990_000 picoseconds.
		Weight::from_parts(48_520_000, 0)
			.saturating_add(Weight::from_parts(0, 23179))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: Staking Bonded (r:1 w:0)
	/// Proof: Staking Bonded (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Staking Ledger (r:1 w:0)
	/// Proof: Staking Ledger (max_values: None, max_size: Some(1124), added: 3599, mode: MaxEncodedLen)
	/// Storage: VoterList ListNodes (r:3 w:3)
	/// Proof: VoterList ListNodes (max_values: None, max_size: Some(154), added: 2629, mode: MaxEncodedLen)
	/// Storage: VoterList ListBags (r:2 w:2)
	/// Proof: VoterList ListBags (max_values: None, max_size: Some(82), added: 2557, mode: MaxEncodedLen)
	fn rebag_terminal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1608`
		//  Estimated: `23107`
		// Minimum execution time: 46_908_000 picoseconds.
		Weight::from_parts(47_468_000, 0)
			.saturating_add(Weight::from_parts(0, 23107))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: VoterList ListNodes (r:4 w:4)
	/// Proof: VoterList ListNodes (max_values: None, max_size: Some(154), added: 2629, mode: MaxEncodedLen)
	/// Storage: Staking Bonded (r:2 w:0)
	/// Proof: Staking Bonded (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Staking Ledger (r:2 w:0)
	/// Proof: Staking Ledger (max_values: None, max_size: Some(1124), added: 3599, mode: MaxEncodedLen)
	/// Storage: VoterList CounterForListNodes (r:1 w:1)
	/// Proof: VoterList CounterForListNodes (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: VoterList ListBags (r:1 w:1)
	/// Proof: VoterList ListBags (max_values: None, max_size: Some(82), added: 2557, mode: MaxEncodedLen)
	fn put_in_front_of() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1917`
		//  Estimated: `30814`
		// Minimum execution time: 52_738_000 picoseconds.
		Weight::from_parts(53_680_000, 0)
			.saturating_add(Weight::from_parts(0, 30814))
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(6))
	}
}