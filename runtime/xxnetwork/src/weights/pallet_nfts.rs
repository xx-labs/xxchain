
//! Autogenerated weights for `pallet_nfts`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-03-21, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `workstation`, CPU: `AMD Ryzen 9 5900X 12-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/xxnetwork-chain
// benchmark
// pallet
// --chain
// dev
// --steps=50
// --repeat=20
// --pallet=pallet_nfts
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/xxnetwork/src/weights/pallet_nfts.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_nfts`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_nfts::WeightInfo for WeightInfo<T> {
	/// Storage: Nfts NextCollectionId (r:1 w:1)
	/// Proof: Nfts NextCollectionId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Nfts Collection (r:1 w:1)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionRoleOf (r:0 w:1)
	/// Proof: Nfts CollectionRoleOf (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionConfigOf (r:0 w:1)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionAccount (r:0 w:1)
	/// Proof: Nfts CollectionAccount (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `216`
		//  Estimated: `5038`
		// Minimum execution time: 25_638_000 picoseconds.
		Weight::from_parts(26_149_000, 0)
			.saturating_add(Weight::from_parts(0, 5038))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: Nfts NextCollectionId (r:1 w:1)
	/// Proof: Nfts NextCollectionId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Nfts Collection (r:1 w:1)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionRoleOf (r:0 w:1)
	/// Proof: Nfts CollectionRoleOf (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionConfigOf (r:0 w:1)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionAccount (r:0 w:1)
	/// Proof: Nfts CollectionAccount (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn force_create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `5038`
		// Minimum execution time: 16_621_000 picoseconds.
		Weight::from_parts(16_952_000, 0)
			.saturating_add(Weight::from_parts(0, 5038))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: Nfts Collection (r:1 w:1)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts ItemMetadataOf (r:1 w:0)
	/// Proof: Nfts ItemMetadataOf (max_values: None, max_size: Some(140), added: 2615, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionRoleOf (r:1 w:1)
	/// Proof: Nfts CollectionRoleOf (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	/// Storage: Nfts Attribute (r:1001 w:1000)
	/// Proof: Nfts Attribute (max_values: None, max_size: Some(446), added: 2921, mode: MaxEncodedLen)
	/// Storage: Nfts ItemConfigOf (r:1000 w:1000)
	/// Proof: Nfts ItemConfigOf (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionMetadataOf (r:0 w:1)
	/// Proof: Nfts CollectionMetadataOf (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionConfigOf (r:0 w:1)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionAccount (r:0 w:1)
	/// Proof: Nfts CollectionAccount (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	/// The range of component `m` is `[0, 1000]`.
	/// The range of component `c` is `[0, 1000]`.
	/// The range of component `a` is `[0, 1000]`.
	fn destroy(_m: u32, c: u32, a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `32186 + a * (332 ±0)`
		//  Estimated: `2538589 + a * (2921 ±0)`
		// Minimum execution time: 893_471_000 picoseconds.
		Weight::from_parts(917_973_062, 0)
			.saturating_add(Weight::from_parts(0, 2538589))
			// Standard Error: 5_338
			.saturating_add(Weight::from_parts(36_329, 0).saturating_mul(c.into()))
			// Standard Error: 5_338
			.saturating_add(Weight::from_parts(4_386_630, 0).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(1004))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(a.into())))
			.saturating_add(T::DbWeight::get().writes(1005))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(a.into())))
			.saturating_add(Weight::from_parts(0, 2921).saturating_mul(a.into()))
	}
	/// Storage: Nfts CollectionConfigOf (r:1 w:0)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// Storage: Nfts Item (r:1 w:1)
	/// Proof: Nfts Item (max_values: None, max_size: Some(861), added: 3336, mode: MaxEncodedLen)
	/// Storage: Nfts Collection (r:1 w:1)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionRoleOf (r:1 w:0)
	/// Proof: Nfts CollectionRoleOf (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	/// Storage: Nfts ItemConfigOf (r:1 w:1)
	/// Proof: Nfts ItemConfigOf (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Nfts Account (r:0 w:1)
	/// Proof: Nfts Account (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	fn mint() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `421`
		//  Estimated: `18460`
		// Minimum execution time: 33_873_000 picoseconds.
		Weight::from_parts(34_664_000, 0)
			.saturating_add(Weight::from_parts(0, 18460))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Nfts CollectionRoleOf (r:1 w:0)
	/// Proof: Nfts CollectionRoleOf (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	/// Storage: Nfts Item (r:1 w:1)
	/// Proof: Nfts Item (max_values: None, max_size: Some(861), added: 3336, mode: MaxEncodedLen)
	/// Storage: Nfts Collection (r:1 w:1)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionConfigOf (r:1 w:0)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// Storage: Nfts ItemConfigOf (r:1 w:1)
	/// Proof: Nfts ItemConfigOf (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Nfts Account (r:0 w:1)
	/// Proof: Nfts Account (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	fn force_mint() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `421`
		//  Estimated: `18460`
		// Minimum execution time: 32_591_000 picoseconds.
		Weight::from_parts(33_011_000, 0)
			.saturating_add(Weight::from_parts(0, 18460))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Nfts ItemConfigOf (r:1 w:1)
	/// Proof: Nfts ItemConfigOf (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Nfts Collection (r:1 w:1)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts Item (r:1 w:1)
	/// Proof: Nfts Item (max_values: None, max_size: Some(861), added: 3336, mode: MaxEncodedLen)
	/// Storage: Nfts ItemMetadataOf (r:1 w:0)
	/// Proof: Nfts ItemMetadataOf (max_values: None, max_size: Some(140), added: 2615, mode: MaxEncodedLen)
	/// Storage: Nfts Account (r:0 w:1)
	/// Proof: Nfts Account (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	/// Storage: Nfts ItemPriceOf (r:0 w:1)
	/// Proof: Nfts ItemPriceOf (max_values: None, max_size: Some(89), added: 2564, mode: MaxEncodedLen)
	/// Storage: Nfts ItemAttributesApprovalsOf (r:0 w:1)
	/// Proof: Nfts ItemAttributesApprovalsOf (max_values: None, max_size: Some(681), added: 3156, mode: MaxEncodedLen)
	/// Storage: Nfts PendingSwapOf (r:0 w:1)
	/// Proof: Nfts PendingSwapOf (max_values: None, max_size: Some(71), added: 2546, mode: MaxEncodedLen)
	fn burn() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `530`
		//  Estimated: `14993`
		// Minimum execution time: 33_373_000 picoseconds.
		Weight::from_parts(33_813_000, 0)
			.saturating_add(Weight::from_parts(0, 14993))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	/// Storage: Nfts Collection (r:1 w:0)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionConfigOf (r:1 w:0)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// Storage: Nfts ItemConfigOf (r:1 w:0)
	/// Proof: Nfts ItemConfigOf (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Nfts Item (r:1 w:1)
	/// Proof: Nfts Item (max_values: None, max_size: Some(861), added: 3336, mode: MaxEncodedLen)
	/// Storage: Nfts Account (r:0 w:2)
	/// Proof: Nfts Account (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	/// Storage: Nfts ItemPriceOf (r:0 w:1)
	/// Proof: Nfts ItemPriceOf (max_values: None, max_size: Some(89), added: 2564, mode: MaxEncodedLen)
	/// Storage: Nfts PendingSwapOf (r:0 w:1)
	/// Proof: Nfts PendingSwapOf (max_values: None, max_size: Some(71), added: 2546, mode: MaxEncodedLen)
	fn transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `559`
		//  Estimated: `14926`
		// Minimum execution time: 27_021_000 picoseconds.
		Weight::from_parts(27_331_000, 0)
			.saturating_add(Weight::from_parts(0, 14926))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: Nfts Collection (r:1 w:0)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionConfigOf (r:1 w:0)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// Storage: Nfts Item (r:5000 w:5000)
	/// Proof: Nfts Item (max_values: None, max_size: Some(861), added: 3336, mode: MaxEncodedLen)
	/// The range of component `i` is `[0, 5000]`.
	fn redeposit(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `729 + i * (108 ±0)`
		//  Estimated: `8077 + i * (3336 ±0)`
		// Minimum execution time: 11_682_000 picoseconds.
		Weight::from_parts(11_862_000, 0)
			.saturating_add(Weight::from_parts(0, 8077))
			// Standard Error: 10_927
			.saturating_add(Weight::from_parts(11_305_954, 0).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(i.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i.into())))
			.saturating_add(Weight::from_parts(0, 3336).saturating_mul(i.into()))
	}
	/// Storage: Nfts CollectionRoleOf (r:1 w:0)
	/// Proof: Nfts CollectionRoleOf (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	/// Storage: Nfts ItemConfigOf (r:1 w:1)
	/// Proof: Nfts ItemConfigOf (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	fn lock_item_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `401`
		//  Estimated: `7047`
		// Minimum execution time: 15_268_000 picoseconds.
		Weight::from_parts(15_779_000, 0)
			.saturating_add(Weight::from_parts(0, 7047))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Nfts CollectionRoleOf (r:1 w:0)
	/// Proof: Nfts CollectionRoleOf (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	/// Storage: Nfts ItemConfigOf (r:1 w:1)
	/// Proof: Nfts ItemConfigOf (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	fn unlock_item_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `401`
		//  Estimated: `7047`
		// Minimum execution time: 15_579_000 picoseconds.
		Weight::from_parts(15_980_000, 0)
			.saturating_add(Weight::from_parts(0, 7047))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Nfts Collection (r:1 w:0)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionConfigOf (r:1 w:1)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	fn lock_collection() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `306`
		//  Estimated: `7087`
		// Minimum execution time: 12_864_000 picoseconds.
		Weight::from_parts(13_235_000, 0)
			.saturating_add(Weight::from_parts(0, 7087))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Nfts OwnershipAcceptance (r:1 w:1)
	/// Proof: Nfts OwnershipAcceptance (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: Nfts Collection (r:1 w:1)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionAccount (r:0 w:2)
	/// Proof: Nfts CollectionAccount (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn transfer_ownership() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `354`
		//  Estimated: `7066`
		// Minimum execution time: 17_493_000 picoseconds.
		Weight::from_parts(17_973_000, 0)
			.saturating_add(Weight::from_parts(0, 7066))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Nfts Collection (r:1 w:1)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionRoleOf (r:2 w:4)
	/// Proof: Nfts CollectionRoleOf (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	fn set_team() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `335`
		//  Estimated: `9627`
		// Minimum execution time: 31_509_000 picoseconds.
		Weight::from_parts(32_190_000, 0)
			.saturating_add(Weight::from_parts(0, 9627))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: Nfts Collection (r:1 w:1)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionAccount (r:0 w:2)
	/// Proof: Nfts CollectionAccount (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn force_collection_owner() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `277`
		//  Estimated: `3549`
		// Minimum execution time: 13_435_000 picoseconds.
		Weight::from_parts(13_736_000, 0)
			.saturating_add(Weight::from_parts(0, 3549))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Nfts Collection (r:1 w:0)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionConfigOf (r:0 w:1)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	fn force_collection_config() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `242`
		//  Estimated: `3549`
		// Minimum execution time: 10_690_000 picoseconds.
		Weight::from_parts(11_211_000, 0)
			.saturating_add(Weight::from_parts(0, 3549))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Nfts CollectionRoleOf (r:1 w:0)
	/// Proof: Nfts CollectionRoleOf (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	/// Storage: Nfts ItemConfigOf (r:1 w:1)
	/// Proof: Nfts ItemConfigOf (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	fn lock_item_properties() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `401`
		//  Estimated: `7047`
		// Minimum execution time: 15_189_000 picoseconds.
		Weight::from_parts(15_529_000, 0)
			.saturating_add(Weight::from_parts(0, 7047))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Nfts Collection (r:1 w:1)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionRoleOf (r:1 w:0)
	/// Proof: Nfts CollectionRoleOf (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionConfigOf (r:1 w:0)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// Storage: Nfts ItemConfigOf (r:1 w:0)
	/// Proof: Nfts ItemConfigOf (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Nfts Attribute (r:1 w:1)
	/// Proof: Nfts Attribute (max_values: None, max_size: Some(446), added: 2921, mode: MaxEncodedLen)
	fn set_attribute() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `505`
		//  Estimated: `18045`
		// Minimum execution time: 37_480_000 picoseconds.
		Weight::from_parts(38_081_000, 0)
			.saturating_add(Weight::from_parts(0, 18045))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Nfts Collection (r:1 w:1)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts Attribute (r:1 w:1)
	/// Proof: Nfts Attribute (max_values: None, max_size: Some(446), added: 2921, mode: MaxEncodedLen)
	fn force_set_attribute() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `310`
		//  Estimated: `7460`
		// Minimum execution time: 20_939_000 picoseconds.
		Weight::from_parts(21_330_000, 0)
			.saturating_add(Weight::from_parts(0, 7460))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Nfts Attribute (r:1 w:1)
	/// Proof: Nfts Attribute (max_values: None, max_size: Some(446), added: 2921, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionRoleOf (r:1 w:0)
	/// Proof: Nfts CollectionRoleOf (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	/// Storage: Nfts ItemConfigOf (r:1 w:0)
	/// Proof: Nfts ItemConfigOf (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Nfts Collection (r:1 w:1)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	fn clear_attribute() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `916`
		//  Estimated: `14507`
		// Minimum execution time: 34_204_000 picoseconds.
		Weight::from_parts(34_575_000, 0)
			.saturating_add(Weight::from_parts(0, 14507))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Nfts Item (r:1 w:0)
	/// Proof: Nfts Item (max_values: None, max_size: Some(861), added: 3336, mode: MaxEncodedLen)
	/// Storage: Nfts ItemAttributesApprovalsOf (r:1 w:1)
	/// Proof: Nfts ItemAttributesApprovalsOf (max_values: None, max_size: Some(681), added: 3156, mode: MaxEncodedLen)
	fn approve_item_attributes() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `347`
		//  Estimated: `8472`
		// Minimum execution time: 13_796_000 picoseconds.
		Weight::from_parts(14_197_000, 0)
			.saturating_add(Weight::from_parts(0, 8472))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Nfts Item (r:1 w:0)
	/// Proof: Nfts Item (max_values: None, max_size: Some(861), added: 3336, mode: MaxEncodedLen)
	/// Storage: Nfts ItemAttributesApprovalsOf (r:1 w:1)
	/// Proof: Nfts ItemAttributesApprovalsOf (max_values: None, max_size: Some(681), added: 3156, mode: MaxEncodedLen)
	/// Storage: Nfts Attribute (r:1001 w:1000)
	/// Proof: Nfts Attribute (max_values: None, max_size: Some(446), added: 2921, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 1000]`.
	fn cancel_item_attributes_approval(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `836 + n * (364 ±0)`
		//  Estimated: `15976 + n * (2921 ±0)`
		// Minimum execution time: 20_899_000 picoseconds.
		Weight::from_parts(5_773_159, 0)
			.saturating_add(Weight::from_parts(0, 15976))
			// Standard Error: 3_100
			.saturating_add(Weight::from_parts(4_407_343, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 2921).saturating_mul(n.into()))
	}
	/// Storage: Nfts CollectionRoleOf (r:1 w:0)
	/// Proof: Nfts CollectionRoleOf (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	/// Storage: Nfts Collection (r:1 w:1)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts ItemConfigOf (r:1 w:0)
	/// Proof: Nfts ItemConfigOf (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionConfigOf (r:1 w:0)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// Storage: Nfts ItemMetadataOf (r:1 w:1)
	/// Proof: Nfts ItemMetadataOf (max_values: None, max_size: Some(140), added: 2615, mode: MaxEncodedLen)
	fn set_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `505`
		//  Estimated: `17739`
		// Minimum execution time: 28_212_000 picoseconds.
		Weight::from_parts(28_634_000, 0)
			.saturating_add(Weight::from_parts(0, 17739))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Nfts CollectionRoleOf (r:1 w:0)
	/// Proof: Nfts CollectionRoleOf (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	/// Storage: Nfts ItemMetadataOf (r:1 w:1)
	/// Proof: Nfts ItemMetadataOf (max_values: None, max_size: Some(140), added: 2615, mode: MaxEncodedLen)
	/// Storage: Nfts Collection (r:1 w:1)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts ItemConfigOf (r:1 w:0)
	/// Proof: Nfts ItemConfigOf (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	fn clear_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `608`
		//  Estimated: `14201`
		// Minimum execution time: 26_961_000 picoseconds.
		Weight::from_parts(27_241_000, 0)
			.saturating_add(Weight::from_parts(0, 14201))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Nfts CollectionRoleOf (r:1 w:0)
	/// Proof: Nfts CollectionRoleOf (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionConfigOf (r:1 w:0)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// Storage: Nfts Collection (r:1 w:1)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionMetadataOf (r:1 w:1)
	/// Proof: Nfts CollectionMetadataOf (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	fn set_collection_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `364`
		//  Estimated: `14173`
		// Minimum execution time: 24_657_000 picoseconds.
		Weight::from_parts(25_137_000, 0)
			.saturating_add(Weight::from_parts(0, 14173))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Nfts CollectionRoleOf (r:1 w:0)
	/// Proof: Nfts CollectionRoleOf (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	/// Storage: Nfts Collection (r:1 w:0)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionConfigOf (r:1 w:0)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionMetadataOf (r:1 w:1)
	/// Proof: Nfts CollectionMetadataOf (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	fn clear_collection_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `475`
		//  Estimated: `14173`
		// Minimum execution time: 24_155_000 picoseconds.
		Weight::from_parts(24_656_000, 0)
			.saturating_add(Weight::from_parts(0, 14173))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Nfts Item (r:1 w:1)
	/// Proof: Nfts Item (max_values: None, max_size: Some(861), added: 3336, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionConfigOf (r:1 w:0)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	fn approve_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `376`
		//  Estimated: `7864`
		// Minimum execution time: 15_860_000 picoseconds.
		Weight::from_parts(16_151_000, 0)
			.saturating_add(Weight::from_parts(0, 7864))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Nfts Item (r:1 w:1)
	/// Proof: Nfts Item (max_values: None, max_size: Some(861), added: 3336, mode: MaxEncodedLen)
	fn cancel_approval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `384`
		//  Estimated: `4326`
		// Minimum execution time: 14_346_000 picoseconds.
		Weight::from_parts(14_527_000, 0)
			.saturating_add(Weight::from_parts(0, 4326))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Nfts Item (r:1 w:1)
	/// Proof: Nfts Item (max_values: None, max_size: Some(861), added: 3336, mode: MaxEncodedLen)
	fn clear_all_transfer_approvals() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `384`
		//  Estimated: `4326`
		// Minimum execution time: 13_355_000 picoseconds.
		Weight::from_parts(13_626_000, 0)
			.saturating_add(Weight::from_parts(0, 4326))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Nfts OwnershipAcceptance (r:1 w:1)
	/// Proof: Nfts OwnershipAcceptance (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn set_accept_ownership() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3517`
		// Minimum execution time: 11_080_000 picoseconds.
		Weight::from_parts(11_722_000, 0)
			.saturating_add(Weight::from_parts(0, 3517))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Nfts CollectionConfigOf (r:1 w:1)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// Storage: Nfts Collection (r:1 w:0)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	fn set_collection_max_supply() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `306`
		//  Estimated: `7087`
		// Minimum execution time: 14_076_000 picoseconds.
		Weight::from_parts(14_527_000, 0)
			.saturating_add(Weight::from_parts(0, 7087))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Nfts CollectionRoleOf (r:1 w:0)
	/// Proof: Nfts CollectionRoleOf (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionConfigOf (r:1 w:1)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	fn update_mint_settings() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `289`
		//  Estimated: `7072`
		// Minimum execution time: 13_866_000 picoseconds.
		Weight::from_parts(14_197_000, 0)
			.saturating_add(Weight::from_parts(0, 7072))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Nfts Item (r:1 w:0)
	/// Proof: Nfts Item (max_values: None, max_size: Some(861), added: 3336, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionConfigOf (r:1 w:0)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// Storage: Nfts ItemConfigOf (r:1 w:0)
	/// Proof: Nfts ItemConfigOf (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Nfts ItemPriceOf (r:0 w:1)
	/// Proof: Nfts ItemPriceOf (max_values: None, max_size: Some(89), added: 2564, mode: MaxEncodedLen)
	fn set_price() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `484`
		//  Estimated: `11377`
		// Minimum execution time: 18_064_000 picoseconds.
		Weight::from_parts(18_494_000, 0)
			.saturating_add(Weight::from_parts(0, 11377))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Nfts Item (r:1 w:1)
	/// Proof: Nfts Item (max_values: None, max_size: Some(861), added: 3336, mode: MaxEncodedLen)
	/// Storage: Nfts ItemPriceOf (r:1 w:1)
	/// Proof: Nfts ItemPriceOf (max_values: None, max_size: Some(89), added: 2564, mode: MaxEncodedLen)
	/// Storage: Nfts Collection (r:1 w:0)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionConfigOf (r:1 w:0)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// Storage: Nfts ItemConfigOf (r:1 w:0)
	/// Proof: Nfts ItemConfigOf (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Nfts Account (r:0 w:2)
	/// Proof: Nfts Account (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	/// Storage: Nfts PendingSwapOf (r:0 w:1)
	/// Proof: Nfts PendingSwapOf (max_values: None, max_size: Some(71), added: 2546, mode: MaxEncodedLen)
	fn buy_item() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `671`
		//  Estimated: `18480`
		// Minimum execution time: 36_148_000 picoseconds.
		Weight::from_parts(36_930_000, 0)
			.saturating_add(Weight::from_parts(0, 18480))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// The range of component `n` is `[0, 10]`.
	fn pay_tips(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_763_000 picoseconds.
		Weight::from_parts(3_488_739, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 7_162
			.saturating_add(Weight::from_parts(2_878_567, 0).saturating_mul(n.into()))
	}
	/// Storage: Nfts Item (r:2 w:0)
	/// Proof: Nfts Item (max_values: None, max_size: Some(861), added: 3336, mode: MaxEncodedLen)
	/// Storage: Nfts PendingSwapOf (r:0 w:1)
	/// Proof: Nfts PendingSwapOf (max_values: None, max_size: Some(71), added: 2546, mode: MaxEncodedLen)
	fn create_swap() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `460`
		//  Estimated: `7662`
		// Minimum execution time: 16_060_000 picoseconds.
		Weight::from_parts(16_571_000, 0)
			.saturating_add(Weight::from_parts(0, 7662))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Nfts PendingSwapOf (r:1 w:1)
	/// Proof: Nfts PendingSwapOf (max_values: None, max_size: Some(71), added: 2546, mode: MaxEncodedLen)
	/// Storage: Nfts Item (r:1 w:0)
	/// Proof: Nfts Item (max_values: None, max_size: Some(861), added: 3336, mode: MaxEncodedLen)
	fn cancel_swap() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `479`
		//  Estimated: `7862`
		// Minimum execution time: 15_599_000 picoseconds.
		Weight::from_parts(15_960_000, 0)
			.saturating_add(Weight::from_parts(0, 7862))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Nfts Item (r:2 w:2)
	/// Proof: Nfts Item (max_values: None, max_size: Some(861), added: 3336, mode: MaxEncodedLen)
	/// Storage: Nfts PendingSwapOf (r:1 w:2)
	/// Proof: Nfts PendingSwapOf (max_values: None, max_size: Some(71), added: 2546, mode: MaxEncodedLen)
	/// Storage: Nfts Collection (r:1 w:0)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionConfigOf (r:1 w:0)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// Storage: Nfts ItemConfigOf (r:2 w:0)
	/// Proof: Nfts ItemConfigOf (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Nfts Account (r:0 w:4)
	/// Proof: Nfts Account (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	/// Storage: Nfts ItemPriceOf (r:0 w:2)
	/// Proof: Nfts ItemPriceOf (max_values: None, max_size: Some(89), added: 2564, mode: MaxEncodedLen)
	fn claim_swap() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `800`
		//  Estimated: `24321`
		// Minimum execution time: 57_017_000 picoseconds.
		Weight::from_parts(57_879_000, 0)
			.saturating_add(Weight::from_parts(0, 24321))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(10))
	}
	/// Storage: Nfts CollectionRoleOf (r:2 w:0)
	/// Proof: Nfts CollectionRoleOf (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionConfigOf (r:1 w:0)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// Storage: Nfts Item (r:1 w:1)
	/// Proof: Nfts Item (max_values: None, max_size: Some(861), added: 3336, mode: MaxEncodedLen)
	/// Storage: Nfts Collection (r:1 w:1)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts ItemConfigOf (r:1 w:1)
	/// Proof: Nfts ItemConfigOf (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Nfts Attribute (r:10 w:10)
	/// Proof: Nfts Attribute (max_values: None, max_size: Some(446), added: 2921, mode: MaxEncodedLen)
	/// Storage: Nfts ItemMetadataOf (r:1 w:1)
	/// Proof: Nfts ItemMetadataOf (max_values: None, max_size: Some(140), added: 2615, mode: MaxEncodedLen)
	/// Storage: Nfts Account (r:0 w:1)
	/// Proof: Nfts Account (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 10]`.
	fn mint_pre_signed(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `628`
		//  Estimated: `29192 + n * (2921 ±0)`
		// Minimum execution time: 103_744_000 picoseconds.
		Weight::from_parts(109_358_273, 0)
			.saturating_add(Weight::from_parts(0, 29192))
			// Standard Error: 23_607
			.saturating_add(Weight::from_parts(21_638_572, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(6))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 2921).saturating_mul(n.into()))
	}
	/// Storage: Nfts Item (r:1 w:0)
	/// Proof: Nfts Item (max_values: None, max_size: Some(861), added: 3336, mode: MaxEncodedLen)
	/// Storage: Nfts ItemAttributesApprovalsOf (r:1 w:1)
	/// Proof: Nfts ItemAttributesApprovalsOf (max_values: None, max_size: Some(681), added: 3156, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionConfigOf (r:1 w:0)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// Storage: Nfts Collection (r:1 w:1)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts Attribute (r:10 w:10)
	/// Proof: Nfts Attribute (max_values: None, max_size: Some(446), added: 2921, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 10]`.
	fn set_attributes_pre_signed(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `553`
		//  Estimated: `20142 + n * (2921 ±0)`
		// Minimum execution time: 63_248_000 picoseconds.
		Weight::from_parts(71_639_563, 0)
			.saturating_add(Weight::from_parts(0, 20142))
			// Standard Error: 38_842
			.saturating_add(Weight::from_parts(21_374_225, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 2921).saturating_mul(n.into()))
	}
}