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

//! Autogenerated weights for xx_public
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-11-09, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("xxnetwork-dev"), DB CACHE: 128

// Executed Command:
// target/release/xxnetwork-chain
// benchmark
// --chain=xxnetwork-dev
// --steps=50
// --repeat=20
// --pallet=xx-public
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./weights/xx-betanet-rewards-weights.rs
// --template=./scripts/frame-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for xx_public.
pub trait WeightInfo {
    fn set_testnet_manager_account() -> Weight;
    fn set_sale_manager_account() -> Weight;
    fn tesnet_distribute() -> Weight;
    fn sale_distribute() -> Weight;
}

/// Weights for xx_public using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    // Storage: XXBetanetRewards Accounts (r:0 w:1)
    fn set_testnet_manager_account() -> Weight {
        (36_000_000 as Weight)
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: XXBetanetRewards Accounts (r:0 w:1)
    fn set_sale_manager_account() -> Weight {
        (36_000_000 as Weight)
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: XXBetanetRewards Accounts (r:0 w:1)
    fn tesnet_distribute() -> Weight {
        (36_000_000 as Weight)
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: XXBetanetRewards Accounts (r:0 w:1)
    fn sale_distribute() -> Weight {
        (36_000_000 as Weight)
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    // Storage: XXBetanetRewards Accounts (r:0 w:1)
    fn set_testnet_manager_account() -> Weight {
        (36_000_000 as Weight)
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    // Storage: XXBetanetRewards Accounts (r:0 w:1)
    fn set_sale_manager_account() -> Weight {
        (36_000_000 as Weight)
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    // Storage: XXBetanetRewards Accounts (r:0 w:1)
    fn tesnet_distribute() -> Weight {
        (36_000_000 as Weight)
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    // Storage: XXBetanetRewards Accounts (r:0 w:1)
    fn sale_distribute() -> Weight {
        (36_000_000 as Weight)
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
}
