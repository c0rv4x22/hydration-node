// This file is part of HydraDX.

// Copyright (C) 2020-2023  Intergalactic, Limited (GIB).
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

//! Autogenerated weights for `pallet_asset_registry`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-12-12, STEPS: `5`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bench-bot`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// target/release/hydradx
// benchmark
// pallet
// --pallet=pallet-asset-registry
// --wasm-execution=compiled
// --heap-pages=4096
// --chain=dev
// --extrinsic=*
// --steps=5
// --repeat=20
// --output
// ./weights-1.1.0/registry.rs
// --template
// .maintain/pallet-weight-template-no-back.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for lbp.
pub trait WeightInfo {
	fn register() -> Weight;
	fn update() -> Weight;
	fn set_metadata() -> Weight;
	fn set_location() -> Weight;
}

/// Weights for lbp using the hack.hydraDX node and recommended hardware.
pub struct HydraWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for HydraWeight<T> {
	/// Storage: `AssetRegistry::AssetIds` (r:1 w:1)
	/// Proof: `AssetRegistry::AssetIds` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::NextAssetId` (r:1 w:1)
	/// Proof: `AssetRegistry::NextAssetId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::LocationAssets` (r:1 w:1)
	/// Proof: `AssetRegistry::LocationAssets` (`max_values`: None, `max_size`: Some(622), added: 3097, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::AssetLocations` (r:0 w:1)
	/// Proof: `AssetRegistry::AssetLocations` (`max_values`: None, `max_size`: Some(614), added: 3089, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::AssetMetadataMap` (r:0 w:1)
	/// Proof: `AssetRegistry::AssetMetadataMap` (`max_values`: None, `max_size`: Some(46), added: 2521, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:0 w:1)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(87), added: 2562, mode: `MaxEncodedLen`)
	fn register() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `305`
		//  Estimated: `4087`
		// Minimum execution time: 46_657_000 picoseconds.
		Weight::from_parts(47_059_000, 4087)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: `AssetRegistry::Assets` (r:1 w:1)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(87), added: 2562, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::AssetIds` (r:1 w:2)
	/// Proof: `AssetRegistry::AssetIds` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn update() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `372`
		//  Estimated: `3552`
		// Minimum execution time: 29_123_000 picoseconds.
		Weight::from_parts(29_400_000, 3552)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `AssetRegistry::Assets` (r:1 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(87), added: 2562, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::AssetMetadataMap` (r:0 w:1)
	/// Proof: `AssetRegistry::AssetMetadataMap` (`max_values`: None, `max_size`: Some(46), added: 2521, mode: `MaxEncodedLen`)
	fn set_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `300`
		//  Estimated: `3552`
		// Minimum execution time: 21_806_000 picoseconds.
		Weight::from_parts(22_285_000, 3552)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `AssetRegistry::Assets` (r:1 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(87), added: 2562, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::LocationAssets` (r:1 w:1)
	/// Proof: `AssetRegistry::LocationAssets` (`max_values`: None, `max_size`: Some(622), added: 3097, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::AssetLocations` (r:1 w:1)
	/// Proof: `AssetRegistry::AssetLocations` (`max_values`: None, `max_size`: Some(614), added: 3089, mode: `MaxEncodedLen`)
	fn set_location() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `274`
		//  Estimated: `4087`
		// Minimum execution time: 26_900_000 picoseconds.
		Weight::from_parts(27_350_000, 4087)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `AssetRegistry::AssetIds` (r:1 w:1)
	/// Proof: `AssetRegistry::AssetIds` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::NextAssetId` (r:1 w:1)
	/// Proof: `AssetRegistry::NextAssetId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::LocationAssets` (r:1 w:1)
	/// Proof: `AssetRegistry::LocationAssets` (`max_values`: None, `max_size`: Some(622), added: 3097, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::AssetLocations` (r:0 w:1)
	/// Proof: `AssetRegistry::AssetLocations` (`max_values`: None, `max_size`: Some(614), added: 3089, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::AssetMetadataMap` (r:0 w:1)
	/// Proof: `AssetRegistry::AssetMetadataMap` (`max_values`: None, `max_size`: Some(46), added: 2521, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:0 w:1)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(87), added: 2562, mode: `MaxEncodedLen`)
	fn register() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `305`
		//  Estimated: `4087`
		// Minimum execution time: 46_657_000 picoseconds.
		Weight::from_parts(47_059_000, 4087)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: `AssetRegistry::Assets` (r:1 w:1)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(87), added: 2562, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::AssetIds` (r:1 w:2)
	/// Proof: `AssetRegistry::AssetIds` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn update() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `372`
		//  Estimated: `3552`
		// Minimum execution time: 29_123_000 picoseconds.
		Weight::from_parts(29_400_000, 3552)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `AssetRegistry::Assets` (r:1 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(87), added: 2562, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::AssetMetadataMap` (r:0 w:1)
	/// Proof: `AssetRegistry::AssetMetadataMap` (`max_values`: None, `max_size`: Some(46), added: 2521, mode: `MaxEncodedLen`)
	fn set_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `300`
		//  Estimated: `3552`
		// Minimum execution time: 21_806_000 picoseconds.
		Weight::from_parts(22_285_000, 3552)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `AssetRegistry::Assets` (r:1 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(87), added: 2562, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::LocationAssets` (r:1 w:1)
	/// Proof: `AssetRegistry::LocationAssets` (`max_values`: None, `max_size`: Some(622), added: 3097, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::AssetLocations` (r:1 w:1)
	/// Proof: `AssetRegistry::AssetLocations` (`max_values`: None, `max_size`: Some(614), added: 3089, mode: `MaxEncodedLen`)
	fn set_location() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `274`
		//  Estimated: `4087`
		// Minimum execution time: 26_900_000 picoseconds.
		Weight::from_parts(27_350_000, 4087)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
}
