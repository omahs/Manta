// Copyright 2020-2022 Manta Network.
// This file is part of Manta.
//
// Manta is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Manta is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Manta.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for pallet_manta_pay
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-09, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("calamari-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/manta
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --pallet=pallet_manta_pay
// --chain=calamari-dev
// --template=.github/resources/frame-weight-template.hbs
// --output=pallet_manta_pay.rs

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use core::marker::PhantomData;
use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};
use frame_system::Config;

/// Weight functions needed for pallet_manta_pay.
pub trait WeightInfo {
    fn to_private_native() -> Weight;
    fn to_private_non_native() -> Weight;
    fn to_public_native() -> Weight;
    fn to_public_non_native() -> Weight;
    fn private_transfer_native() -> Weight;
    fn private_transfer_non_native() -> Weight;
    fn public_transfer_native() -> Weight;
    fn public_transfer_non_native() -> Weight;
}

/// Weights for pallet_manta_pay using the Substrate node and recommended hardware.pub struct SubstrateWeight<T>(PhantomData<T>);
pub struct SubstrateWeight<T>(PhantomData<T>);

impl<T> WeightInfo for SubstrateWeight<T>
where
    T: Config,
{
    // Storage: MantaPay UtxoSet (r:1 w:1)
    // Storage: MantaPay NullifierSetSize (r:1 w:0)
    // Storage: MantaPay ShardTrees (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: MantaPay UtxoAccumulatorOutputs (r:0 w:1)
    // Storage: MantaPay Shards (r:0 w:1)
    fn to_private_native() -> Weight {
        (26_934_980_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().writes(5 as Weight))
    }
    // Storage: Assets Asset (r:1 w:1)
    // Storage: Assets Account (r:2 w:2)
    // Storage: MantaPay UtxoSet (r:1 w:1)
    // Storage: MantaPay NullifierSetSize (r:1 w:0)
    // Storage: MantaPay ShardTrees (r:1 w:1)
    // Storage: MantaPay UtxoAccumulatorOutputs (r:0 w:1)
    // Storage: MantaPay Shards (r:0 w:1)
    fn to_private_non_native() -> Weight {
        (27_130_516_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(6 as Weight))
            .saturating_add(T::DbWeight::get().writes(7 as Weight))
    }
    // Storage: MantaPay UtxoAccumulatorOutputs (r:2 w:1)
    // Storage: MantaPay NullifierCommitmentSet (r:2 w:2)
    // Storage: MantaPay UtxoSet (r:1 w:1)
    // Storage: MantaPay NullifierSetSize (r:1 w:1)
    // Storage: MantaPay ShardTrees (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: MantaPay NullifierSetInsertionOrder (r:0 w:2)
    // Storage: MantaPay Shards (r:0 w:1)
    fn to_public_native() -> Weight {
        (35_563_143_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(8 as Weight))
            .saturating_add(T::DbWeight::get().writes(10 as Weight))
    }
    // Storage: Assets Asset (r:1 w:1)
    // Storage: Assets Account (r:2 w:2)
    // Storage: MantaPay UtxoAccumulatorOutputs (r:2 w:1)
    // Storage: MantaPay NullifierCommitmentSet (r:2 w:2)
    // Storage: MantaPay UtxoSet (r:1 w:1)
    // Storage: MantaPay NullifierSetSize (r:1 w:1)
    // Storage: MantaPay ShardTrees (r:1 w:1)
    // Storage: MantaPay NullifierSetInsertionOrder (r:0 w:2)
    // Storage: MantaPay Shards (r:0 w:1)
    fn to_public_non_native() -> Weight {
        (35_601_447_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(10 as Weight))
            .saturating_add(T::DbWeight::get().writes(12 as Weight))
    }
    // Storage: MantaPay UtxoAccumulatorOutputs (r:2 w:2)
    // Storage: MantaPay NullifierCommitmentSet (r:2 w:2)
    // Storage: MantaPay UtxoSet (r:2 w:2)
    // Storage: MantaPay NullifierSetSize (r:1 w:1)
    // Storage: MantaPay ShardTrees (r:2 w:2)
    // Storage: MantaPay NullifierSetInsertionOrder (r:0 w:2)
    // Storage: MantaPay Shards (r:0 w:2)
    fn private_transfer_native() -> Weight {
        (48_265_336_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(9 as Weight))
            .saturating_add(T::DbWeight::get().writes(13 as Weight))
    }
    // Storage: MantaPay UtxoAccumulatorOutputs (r:2 w:2)
    // Storage: MantaPay NullifierCommitmentSet (r:2 w:2)
    // Storage: MantaPay UtxoSet (r:2 w:2)
    // Storage: MantaPay NullifierSetSize (r:1 w:1)
    // Storage: MantaPay ShardTrees (r:2 w:2)
    // Storage: MantaPay NullifierSetInsertionOrder (r:0 w:2)
    // Storage: MantaPay Shards (r:0 w:2)
    fn private_transfer_non_native() -> Weight {
        (47_666_690_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(9 as Weight))
            .saturating_add(T::DbWeight::get().writes(13 as Weight))
    }
    // Storage: System Account (r:1 w:1)
    fn public_transfer_native() -> Weight {
        (32_532_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Assets Asset (r:1 w:1)
    // Storage: Assets Account (r:2 w:2)
    fn public_transfer_non_native() -> Weight {
        (43_141_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    // Storage: MantaPay UtxoSet (r:1 w:1)
    // Storage: MantaPay NullifierSetSize (r:1 w:0)
    // Storage: MantaPay ShardTrees (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: MantaPay UtxoAccumulatorOutputs (r:0 w:1)
    // Storage: MantaPay Shards (r:0 w:1)
    fn to_private_native() -> Weight {
        (26_934_980_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(4 as Weight))
            .saturating_add(RocksDbWeight::get().writes(5 as Weight))
    }
    // Storage: Assets Asset (r:1 w:1)
    // Storage: Assets Account (r:2 w:2)
    // Storage: MantaPay UtxoSet (r:1 w:1)
    // Storage: MantaPay NullifierSetSize (r:1 w:0)
    // Storage: MantaPay ShardTrees (r:1 w:1)
    // Storage: MantaPay UtxoAccumulatorOutputs (r:0 w:1)
    // Storage: MantaPay Shards (r:0 w:1)
    fn to_private_non_native() -> Weight {
        (27_130_516_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(6 as Weight))
            .saturating_add(RocksDbWeight::get().writes(7 as Weight))
    }
    // Storage: MantaPay UtxoAccumulatorOutputs (r:2 w:1)
    // Storage: MantaPay NullifierCommitmentSet (r:2 w:2)
    // Storage: MantaPay UtxoSet (r:1 w:1)
    // Storage: MantaPay NullifierSetSize (r:1 w:1)
    // Storage: MantaPay ShardTrees (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: MantaPay NullifierSetInsertionOrder (r:0 w:2)
    // Storage: MantaPay Shards (r:0 w:1)
    fn to_public_native() -> Weight {
        (35_563_143_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(8 as Weight))
            .saturating_add(RocksDbWeight::get().writes(10 as Weight))
    }
    // Storage: Assets Asset (r:1 w:1)
    // Storage: Assets Account (r:2 w:2)
    // Storage: MantaPay UtxoAccumulatorOutputs (r:2 w:1)
    // Storage: MantaPay NullifierCommitmentSet (r:2 w:2)
    // Storage: MantaPay UtxoSet (r:1 w:1)
    // Storage: MantaPay NullifierSetSize (r:1 w:1)
    // Storage: MantaPay ShardTrees (r:1 w:1)
    // Storage: MantaPay NullifierSetInsertionOrder (r:0 w:2)
    // Storage: MantaPay Shards (r:0 w:1)
    fn to_public_non_native() -> Weight {
        (35_601_447_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(10 as Weight))
            .saturating_add(RocksDbWeight::get().writes(12 as Weight))
    }
    // Storage: MantaPay UtxoAccumulatorOutputs (r:2 w:2)
    // Storage: MantaPay NullifierCommitmentSet (r:2 w:2)
    // Storage: MantaPay UtxoSet (r:2 w:2)
    // Storage: MantaPay NullifierSetSize (r:1 w:1)
    // Storage: MantaPay ShardTrees (r:2 w:2)
    // Storage: MantaPay NullifierSetInsertionOrder (r:0 w:2)
    // Storage: MantaPay Shards (r:0 w:2)
    fn private_transfer_native() -> Weight {
        (48_265_336_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(9 as Weight))
            .saturating_add(RocksDbWeight::get().writes(13 as Weight))
    }
    // Storage: MantaPay UtxoAccumulatorOutputs (r:2 w:2)
    // Storage: MantaPay NullifierCommitmentSet (r:2 w:2)
    // Storage: MantaPay UtxoSet (r:2 w:2)
    // Storage: MantaPay NullifierSetSize (r:1 w:1)
    // Storage: MantaPay ShardTrees (r:2 w:2)
    // Storage: MantaPay NullifierSetInsertionOrder (r:0 w:2)
    // Storage: MantaPay Shards (r:0 w:2)
    fn private_transfer_non_native() -> Weight {
        (47_666_690_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(9 as Weight))
            .saturating_add(RocksDbWeight::get().writes(13 as Weight))
    }
    // Storage: System Account (r:1 w:1)
    fn public_transfer_native() -> Weight {
        (32_532_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    // Storage: Assets Asset (r:1 w:1)
    // Storage: Assets Account (r:2 w:2)
    fn public_transfer_non_native() -> Weight {
        (43_141_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(3 as Weight))
            .saturating_add(RocksDbWeight::get().writes(3 as Weight))
    }
}
