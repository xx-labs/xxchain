
//! Autogenerated weights for `claims`
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
	/// Storage: Claims Claims (r:1 w:1)
	/// Proof Skipped: Claims Claims (max_values: None, max_size: None, mode: Measured)
	/// Storage: Claims Signing (r:1 w:1)
	/// Proof Skipped: Claims Signing (max_values: None, max_size: None, mode: Measured)
	/// Storage: Claims Total (r:1 w:1)
	/// Proof Skipped: Claims Total (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Claims Vesting (r:1 w:1)
	/// Proof Skipped: Claims Vesting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1057), added: 3532, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Claims Rewards (r:1 w:0)
	/// Proof Skipped: Claims Rewards (max_values: None, max_size: None, mode: Measured)
	fn claim() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `617`
		//  Estimated: `31309`
		// Minimum execution time: 124_081_000 picoseconds.
		Weight::from_parts(127_367_000, 0)
			.saturating_add(Weight::from_parts(0, 31309))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: Claims Total (r:1 w:1)
	/// Proof Skipped: Claims Total (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Claims Vesting (r:0 w:1)
	/// Proof Skipped: Claims Vesting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Claims Claims (r:0 w:1)
	/// Proof Skipped: Claims Claims (max_values: None, max_size: None, mode: Measured)
	/// Storage: Claims Signing (r:0 w:1)
	/// Proof Skipped: Claims Signing (max_values: None, max_size: None, mode: Measured)
	fn mint_claim() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `145`
		//  Estimated: `2065`
		// Minimum execution time: 9_328_000 picoseconds.
		Weight::from_parts(9_768_000, 0)
			.saturating_add(Weight::from_parts(0, 2065))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Claims Claims (r:1 w:1)
	/// Proof Skipped: Claims Claims (max_values: None, max_size: None, mode: Measured)
	/// Storage: Claims Signing (r:1 w:1)
	/// Proof Skipped: Claims Signing (max_values: None, max_size: None, mode: Measured)
	/// Storage: Claims Total (r:1 w:1)
	/// Proof Skipped: Claims Total (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Claims Vesting (r:1 w:1)
	/// Proof Skipped: Claims Vesting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1057), added: 3532, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Claims Rewards (r:1 w:0)
	/// Proof Skipped: Claims Rewards (max_values: None, max_size: None, mode: Measured)
	fn claim_attest() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `617`
		//  Estimated: `31309`
		// Minimum execution time: 128_090_000 picoseconds.
		Weight::from_parts(131_615_000, 0)
			.saturating_add(Weight::from_parts(0, 31309))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: Claims Preclaims (r:1 w:1)
	/// Proof Skipped: Claims Preclaims (max_values: None, max_size: None, mode: Measured)
	/// Storage: Claims Signing (r:1 w:1)
	/// Proof Skipped: Claims Signing (max_values: None, max_size: None, mode: Measured)
	/// Storage: Claims Claims (r:1 w:1)
	/// Proof Skipped: Claims Claims (max_values: None, max_size: None, mode: Measured)
	/// Storage: Claims Total (r:1 w:1)
	/// Proof Skipped: Claims Total (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Claims Vesting (r:1 w:1)
	/// Proof Skipped: Claims Vesting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1057), added: 3532, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Claims Rewards (r:1 w:0)
	/// Proof Skipped: Claims Rewards (max_values: None, max_size: None, mode: Measured)
	fn attest() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `691`
		//  Estimated: `35835`
		// Minimum execution time: 59_241_000 picoseconds.
		Weight::from_parts(63_298_000, 0)
			.saturating_add(Weight::from_parts(0, 35835))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	/// Storage: Claims Claims (r:1 w:2)
	/// Proof Skipped: Claims Claims (max_values: None, max_size: None, mode: Measured)
	/// Storage: Claims Vesting (r:1 w:2)
	/// Proof Skipped: Claims Vesting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Claims Signing (r:1 w:2)
	/// Proof Skipped: Claims Signing (max_values: None, max_size: None, mode: Measured)
	/// Storage: Claims Preclaims (r:1 w:1)
	/// Proof Skipped: Claims Preclaims (max_values: None, max_size: None, mode: Measured)
	fn move_claim() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `370`
		//  Estimated: `15340`
		// Minimum execution time: 19_596_000 picoseconds.
		Weight::from_parts(20_278_000, 0)
			.saturating_add(Weight::from_parts(0, 15340))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(7))
	}
}
