// Copyright 2020-2022 Manta Network.
// This file is part of Manta.

// Manta is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Manta is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Manta.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for pallet_asset_manager
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-06-07, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("calamari-dev"), DB CACHE: 1024

// Executed Command:
// manta
// benchmark
// --chain=calamari-dev
// --pallet=pallet_asset_manager
// --extrinsic=*
// --execution=Wasm
// --wasm-execution=Compiled
// --heap-pages=4096
// --repeat=20
// --steps=50
// --template=.github/resources/frame-weight-template.hbs
// --output=pallet_asset_manager.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_asset_manager.
pub trait WeightInfo {
    fn register_asset() -> Weight;
    fn set_units_per_second() -> Weight;
    fn update_asset_location() -> Weight;
    fn update_asset_metadata() -> Weight;
    fn mint_asset() -> Weight;
    fn set_min_xcm_fee() -> Weight;
}

/// Weights for pallet_asset_manager using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_asset_manager::WeightInfo for SubstrateWeight<T> {
    // Storage: AssetManager LocationAssetId (r:1 w:1)
    // Storage: AssetManager NextAssetId (r:1 w:1)
    // Storage: Assets Asset (r:1 w:1)
    // Storage: Assets Metadata (r:1 w:1)
    // Storage: AssetManager AssetIdMetadata (r:0 w:1)
    // Storage: AssetManager AssetIdLocation (r:0 w:1)
    fn register_asset() -> Weight {
        (37_954_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().writes(6 as Weight))
    }
    // Storage: AssetManager AssetIdLocation (r:1 w:0)
    // Storage: AssetManager UnitsPerSecond (r:0 w:1)
    fn set_units_per_second() -> Weight {
        (46_763_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: AssetManager AssetIdLocation (r:1 w:1)
    // Storage: AssetManager LocationAssetId (r:1 w:2)
    // Storage: AssetManager AllowedDestParaIds (r:1 w:1)
    fn update_asset_location() -> Weight {
        (62_667_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
    }
    // Storage: AssetManager AssetIdLocation (r:1 w:0)
    // Storage: Assets Asset (r:1 w:0)
    // Storage: Assets Metadata (r:1 w:1)
    // Storage: AssetManager AssetIdMetadata (r:0 w:1)
    fn update_asset_metadata() -> Weight {
        (63_177_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: AssetManager AssetIdLocation (r:1 w:0)
    // Storage: Assets Asset (r:1 w:1)
    // Storage: Assets Account (r:1 w:1)
    fn mint_asset() -> Weight {
        (73_558_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: AssetManager MinXcmFee (r:0 w:1)
    fn set_min_xcm_fee() -> Weight {
        (43_240_000 as Weight)
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    // Storage: AssetManager LocationAssetId (r:1 w:1)
    // Storage: AssetManager NextAssetId (r:1 w:1)
    // Storage: Assets Asset (r:1 w:1)
    // Storage: Assets Metadata (r:1 w:1)
    // Storage: AssetManager AssetIdMetadata (r:0 w:1)
    // Storage: AssetManager AssetIdLocation (r:0 w:1)
    fn register_asset() -> Weight {
        (37_954_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(4 as Weight))
            .saturating_add(RocksDbWeight::get().writes(6 as Weight))
    }
    // Storage: AssetManager AssetIdLocation (r:1 w:0)
    // Storage: AssetManager UnitsPerSecond (r:0 w:1)
    fn set_units_per_second() -> Weight {
        (46_763_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    // Storage: AssetManager AssetIdLocation (r:1 w:1)
    // Storage: AssetManager LocationAssetId (r:1 w:2)
    // Storage: AssetManager AllowedDestParaIds (r:1 w:1)
    fn update_asset_location() -> Weight {
        (62_667_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(3 as Weight))
            .saturating_add(RocksDbWeight::get().writes(4 as Weight))
    }
    // Storage: AssetManager AssetIdLocation (r:1 w:0)
    // Storage: Assets Asset (r:1 w:0)
    // Storage: Assets Metadata (r:1 w:1)
    // Storage: AssetManager AssetIdMetadata (r:0 w:1)
    fn update_asset_metadata() -> Weight {
        (63_177_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(3 as Weight))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
    }
    // Storage: AssetManager AssetIdLocation (r:1 w:0)
    // Storage: Assets Asset (r:1 w:1)
    // Storage: Assets Account (r:1 w:1)
    fn mint_asset() -> Weight {
        (73_558_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(3 as Weight))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
    }
    // Storage: AssetManager MinXcmFee (r:0 w:1)
    fn set_min_xcm_fee() -> Weight {
        (43_240_000 as Weight)
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
}
