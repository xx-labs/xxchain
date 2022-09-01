
//! Autogenerated weights for `pallet_proxy`
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
// --pallet=pallet_proxy
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./weights/pallet_proxy.rs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{RefTimeWeight , Weight}};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_proxy`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_proxy::WeightInfo for WeightInfo<T> {
	// Storage: Proxy Proxies (r:1 w:0)
	fn proxy(p: u32, ) -> Weight {
		Weight::from_ref_time(20_300_000 as RefTimeWeight)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(118_000 as RefTimeWeight).saturating_mul(p as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
	}
	// Storage: Proxy Proxies (r:1 w:0)
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn proxy_announced(a: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(48_110_000 as RefTimeWeight)
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(449_000 as RefTimeWeight).saturating_mul(a as RefTimeWeight))
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(112_000 as RefTimeWeight).saturating_mul(p as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(3 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn remove_announcement(a: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(32_582_000 as RefTimeWeight)
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(457_000 as RefTimeWeight).saturating_mul(a as RefTimeWeight))
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(1_000 as RefTimeWeight).saturating_mul(p as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn reject_announcement(a: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(32_536_000 as RefTimeWeight)
			// Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(459_000 as RefTimeWeight).saturating_mul(a as RefTimeWeight))
			// Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(18_000 as RefTimeWeight).saturating_mul(p as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Proxy Proxies (r:1 w:0)
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn announce(a: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(45_430_000 as RefTimeWeight)
			// Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(483_000 as RefTimeWeight).saturating_mul(a as RefTimeWeight))
			// Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(125_000 as RefTimeWeight).saturating_mul(p as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(3 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	fn add_proxy(p: u32, ) -> Weight {
		Weight::from_ref_time(39_277_000 as RefTimeWeight)
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(177_000 as RefTimeWeight).saturating_mul(p as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	fn remove_proxy(p: u32, ) -> Weight {
		Weight::from_ref_time(32_171_000 as RefTimeWeight)
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(196_000 as RefTimeWeight).saturating_mul(p as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	fn remove_proxies(_p: u32, ) -> Weight {
		Weight::from_ref_time(34_083_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: Proxy Proxies (r:1 w:1)
	fn anonymous(p: u32, ) -> Weight {
		Weight::from_ref_time(44_605_000 as RefTimeWeight)
			// Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(12_000 as RefTimeWeight).saturating_mul(p as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	fn kill_anonymous(p: u32, ) -> Weight {
		Weight::from_ref_time(32_466_000 as RefTimeWeight)
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(123_000 as RefTimeWeight).saturating_mul(p as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
}
