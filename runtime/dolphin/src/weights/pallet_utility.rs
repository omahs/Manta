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

//! Autogenerated weights for pallet_utility
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-04-19, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dolphin-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/manta
// benchmark
// --chain=dolphin-dev
// --pallet=pallet_utility
// --extrinsic=*
// --execution=Wasm
// --wasm-execution=Compiled
// --heap-pages=4096
// --repeat=20
// --steps=50
// --template=.github/resources/frame-weight-template.hbs
// --output=pallet_utility.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_utility.
pub trait WeightInfo {
        fn batch(c: u32, ) -> Weight;
        fn as_derivative() -> Weight;
        fn batch_all(c: u32, ) -> Weight;
        fn dispatch_as() -> Weight;
}

/// Weights for pallet_utility using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_utility::WeightInfo for SubstrateWeight<T> {
        fn batch(c: u32, ) -> Weight {
                (108_304_000 as Weight)
                        // Standard Error: 5_000
                        .saturating_add((8_427_000 as Weight).saturating_mul(c as Weight))
        }
        fn as_derivative() -> Weight {
                (8_087_000 as Weight)
        }
        fn batch_all(c: u32, ) -> Weight {
                (44_611_000 as Weight)
                        // Standard Error: 3_000
                        .saturating_add((9_186_000 as Weight).saturating_mul(c as Weight))
        }
        fn dispatch_as() -> Weight {
                (26_327_000 as Weight)
        }
}

// For backwards compatibility and tests
impl WeightInfo for () {
        fn batch(c: u32, ) -> Weight {
                (108_304_000 as Weight)
                        // Standard Error: 5_000
                        .saturating_add((8_427_000 as Weight).saturating_mul(c as Weight))
        }
        fn as_derivative() -> Weight {
                (8_087_000 as Weight)
        }
        fn batch_all(c: u32, ) -> Weight {
                (44_611_000 as Weight)
                        // Standard Error: 3_000
                        .saturating_add((9_186_000 as Weight).saturating_mul(c as Weight))
        }
        fn dispatch_as() -> Weight {
                (26_327_000 as Weight)
        }
}
