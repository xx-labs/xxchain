// This file is part of Substrate.

// Copyright (C) 2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for xx_cmix
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-09-13, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("xxnetwork-dev"), DB CACHE: 128

// Executed Command:
// target/release/xxnetwork-chain
// benchmark
// --chain=xxnetwork-dev
// --steps=50
// --repeat=20
// --pallet=xx-cmix
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./xx-cmix/src/weights.rs
// --template=./scripts/frame-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for xx_cmix.
pub trait WeightInfo {
	fn set_cmix_hashes() -> Weight;
	fn set_scheduling_account() -> Weight;
	fn set_next_cmix_variables() -> Weight;
	fn submit_cmix_points(n: u32, ) -> Weight;
	fn submit_cmix_deductions(n: u32, ) -> Weight;
	fn set_cmix_address_space() -> Weight;
	fn set_admin_permission() -> Weight;
}

/// Weights for xx_cmix using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: XXCmix AdminPermission (r:1 w:0)
	// Storage: XXCmix CmixHashes (r:0 w:1)
	fn set_cmix_hashes() -> Weight {
		Weight::from_ref_time(28_827_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: XXCmix SchedulingAccount (r:0 w:1)
	fn set_scheduling_account() -> Weight {
		Weight::from_ref_time(18_741_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: XXCmix NextCmixVariables (r:0 w:1)
	fn set_next_cmix_variables() -> Weight {
		Weight::from_ref_time(5_291_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Staking ErasRewardPoints (r:1 w:1)
	// Storage: Staking ActiveEra (r:1 w:0)
	// Storage: XXCmix SchedulingAccount (r:1 w:0)
	fn submit_cmix_points(n: u32, ) -> Weight {
		Weight::from_ref_time(34_311_000 as u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(383_000 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Staking ActiveEra (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:1)
	// Storage: XXCmix SchedulingAccount (r:1 w:0)
	fn submit_cmix_deductions(n: u32, ) -> Weight {
		Weight::from_ref_time(34_593_000 as u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(374_000 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: XXCmix CmixAddressSpace (r:0 w:1)
	// Storage: XXCmix SchedulingAccount (r:1 w:0)
	fn set_cmix_address_space() -> Weight {
		Weight::from_ref_time(23_948_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: XXCmix AdminPermission (r:0 w:1)
	fn set_admin_permission() -> Weight {
		Weight::from_ref_time(18_172_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: XXCmix AdminPermission (r:1 w:0)
	// Storage: XXCmix CmixHashes (r:0 w:1)
	fn set_cmix_hashes() -> Weight {
		Weight::from_ref_time(28_827_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: XXCmix SchedulingAccount (r:0 w:1)
	fn set_scheduling_account() -> Weight {
		Weight::from_ref_time(18_741_000 as u64)
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: XXCmix NextCmixVariables (r:0 w:1)
	fn set_next_cmix_variables() -> Weight {
		Weight::from_ref_time(5_291_000 as u64)
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Staking ErasRewardPoints (r:1 w:1)
	// Storage: Staking ActiveEra (r:1 w:0)
	// Storage: XXCmix SchedulingAccount (r:1 w:0)
	fn submit_cmix_points(n: u32, ) -> Weight {
		Weight::from_ref_time(34_311_000 as u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(383_000 as u64).saturating_mul(n as u64))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Staking ActiveEra (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:1)
	// Storage: XXCmix SchedulingAccount (r:1 w:0)
	fn submit_cmix_deductions(n: u32, ) -> Weight {
		Weight::from_ref_time(34_593_000 as u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(374_000 as u64).saturating_mul(n as u64))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: XXCmix CmixAddressSpace (r:0 w:1)
	// Storage: XXCmix SchedulingAccount (r:1 w:0)
	fn set_cmix_address_space() -> Weight {
		Weight::from_ref_time(23_948_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: XXCmix AdminPermission (r:0 w:1)
	fn set_admin_permission() -> Weight {
		Weight::from_ref_time(18_172_000 as u64)
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
}
