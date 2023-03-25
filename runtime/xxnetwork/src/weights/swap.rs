
//! Autogenerated weights for `swap`
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
// --pallet=swap
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./weights/swap.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `swap`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> swap::WeightInfo for WeightInfo<T> {
	/// Storage: ChainBridge ChainNonces (r:1 w:1)
	/// Proof Skipped: ChainBridge ChainNonces (max_values: None, max_size: None, mode: Measured)
	/// Storage: Swap SwapFee (r:1 w:0)
	/// Proof Skipped: Swap SwapFee (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:3 w:3)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Swap FeeDestination (r:1 w:0)
	/// Proof Skipped: Swap FeeDestination (max_values: Some(1), max_size: None, mode: Measured)
	fn transfer_native() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `837`
		//  Estimated: `17745`
		// Minimum execution time: 48_411_000 picoseconds.
		Weight::from_parts(49_422_000, 0)
			.saturating_add(Weight::from_parts(0, 17745))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `192`
		//  Estimated: `6196`
		// Minimum execution time: 24_717_000 picoseconds.
		Weight::from_parts(25_147_000, 0)
			.saturating_add(Weight::from_parts(0, 6196))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Swap SwapFee (r:0 w:1)
	/// Proof Skipped: Swap SwapFee (max_values: Some(1), max_size: None, mode: Measured)
	fn set_swap_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_532_000 picoseconds.
		Weight::from_parts(6_803_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Swap FeeDestination (r:0 w:1)
	/// Proof Skipped: Swap FeeDestination (max_values: Some(1), max_size: None, mode: Measured)
	fn set_fee_destination() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_672_000 picoseconds.
		Weight::from_parts(6_993_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}