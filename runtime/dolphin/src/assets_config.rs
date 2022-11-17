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

use super::{
    cDOL, weights, xcm_config::SelfReserve, AssetManager, Assets, Balances, Event,
    NativeTokenExistentialDeposit, Origin, Runtime, DOL,
};

use manta_primitives::{
    assets::{
        AssetConfig, AssetIdType, AssetLocation, AssetRegistry, AssetRegistryMetadata,
        AssetStorageMetadata, BalanceType, LocationType, NativeAndNonNative,
    },
    constants::{ASSET_MANAGER_PALLET_ID, DOLPHIN_DECIMAL, MANTA_PAY_PALLET_ID},
    types::{AccountId, Balance, DolphinAssetId},
};

use frame_support::{pallet_prelude::DispatchResult, parameter_types, PalletId};
use frame_system::EnsureRoot;
use xcm::VersionedMultiLocation;

parameter_types! {
    pub const AssetDeposit: Balance = DOL;
    pub const AssetAccountDeposit: Balance =  NativeTokenExistentialDeposit::get();
    pub const ApprovalDeposit: Balance = 10 * cDOL;
    pub const AssetsStringLimit: u32 = 50;
    pub const MetadataDepositBase: Balance = DOL;
    pub const MetadataDepositPerByte: Balance = cDOL;
}

impl pallet_assets::Config for Runtime {
    type Event = Event;
    type Balance = Balance;
    type AssetId = DolphinAssetId;
    type Currency = Balances;
    type ForceOrigin = EnsureRoot<AccountId>;
    type AssetDeposit = AssetDeposit;
    type AssetAccountDeposit = AssetAccountDeposit;
    type MetadataDepositBase = MetadataDepositBase;
    type MetadataDepositPerByte = MetadataDepositPerByte;
    type ApprovalDeposit = ApprovalDeposit;
    type StringLimit = AssetsStringLimit;
    type Freezer = ();
    type Extra = ();
    type WeightInfo = weights::pallet_assets::SubstrateWeight<Runtime>;
}

pub struct MantaAssetRegistry;
impl BalanceType for MantaAssetRegistry {
    type Balance = Balance;
}
impl AssetIdType for MantaAssetRegistry {
    type AssetId = DolphinAssetId;
}
impl AssetRegistry<Runtime> for MantaAssetRegistry {
    type Metadata = AssetStorageMetadata;
    type Error = sp_runtime::DispatchError;

    fn create_asset(
        who: Origin,
        asset_id: DolphinAssetId,
        admin: AccountId,
        metadata: AssetStorageMetadata,
        min_balance: Balance,
    ) -> DispatchResult {
        Assets::create(
            who.clone(),
            asset_id,
            sp_runtime::MultiAddress::Id(admin),
            min_balance,
        )?;

        Assets::set_metadata(
            who,
            asset_id,
            metadata.name,
            metadata.symbol,
            metadata.decimals,
        )
    }

    fn force_create_asset(
        asset_id: DolphinAssetId,
        metadata: AssetStorageMetadata,
        min_balance: Balance,
        is_sufficient: bool,
    ) -> DispatchResult {
        Assets::force_create(
            Origin::root(),
            asset_id,
            sp_runtime::MultiAddress::Id(AssetManager::account_id()),
            is_sufficient,
            min_balance,
        )?;

        Assets::force_set_metadata(
            Origin::root(),
            asset_id,
            metadata.name,
            metadata.symbol,
            metadata.decimals,
            metadata.is_frozen,
        )
    }

    fn update_metadata(
        origin: Origin,
        asset_id: &DolphinAssetId,
        metadata: AssetStorageMetadata,
    ) -> DispatchResult {
        Assets::set_metadata(
            origin,
            *asset_id,
            metadata.name,
            metadata.symbol,
            metadata.decimals,
        )
    }

    fn force_update_metadata(
        asset_id: &DolphinAssetId,
        metadata: AssetStorageMetadata,
    ) -> DispatchResult {
        Assets::force_set_metadata(
            Origin::root(),
            *asset_id,
            metadata.name,
            metadata.symbol,
            metadata.decimals,
            metadata.is_frozen,
        )
    }

    fn mint_asset(
        origin: Origin,
        asset_id: &DolphinAssetId,
        beneficiary: AccountId,
        amount: Balance,
    ) -> DispatchResult {
        Assets::mint(
            origin,
            *asset_id,
            sp_runtime::MultiAddress::Id(beneficiary),
            amount,
        )
    }
}

parameter_types! {
    pub const StartNonNativeAssetId: DolphinAssetId = 8;
    pub const NativeAssetId: DolphinAssetId = 1;
    pub NativeAssetLocation: AssetLocation = AssetLocation(
        VersionedMultiLocation::V1(SelfReserve::get()));
    pub NativeAssetMetadata: AssetRegistryMetadata<Balance> = AssetRegistryMetadata {
        metadata: AssetStorageMetadata {
            name: b"Dolphin".to_vec(),
            symbol: b"DOL".to_vec(),
            decimals: DOLPHIN_DECIMAL,
            is_frozen: false,
        },
        min_balance: NativeTokenExistentialDeposit::get(),
        is_sufficient: true,
    };
    pub const AssetManagerPalletId: PalletId = ASSET_MANAGER_PALLET_ID;
}

pub type DolphinConcreteFungibleLedger =
    NativeAndNonNative<Runtime, DolphinAssetConfig, Balances, Assets>;

/// AssetConfig implementations for this runtime
#[derive(Clone, Eq, PartialEq)]
pub struct DolphinAssetConfig;
impl LocationType for DolphinAssetConfig {
    type Location = AssetLocation;
}
impl BalanceType for DolphinAssetConfig {
    type Balance = Balance;
}
impl AssetIdType for DolphinAssetConfig {
    type AssetId = DolphinAssetId;
}
impl AssetConfig<Runtime> for DolphinAssetConfig {
    type StartNonNativeAssetId = StartNonNativeAssetId;
    type NativeAssetId = NativeAssetId;
    type AssetRegistryMetadata = AssetRegistryMetadata<Balance>;
    type NativeAssetLocation = NativeAssetLocation;
    type NativeAssetMetadata = NativeAssetMetadata;
    type StorageMetadata = AssetStorageMetadata;
    type AssetRegistry = MantaAssetRegistry;
    type FungibleLedger = DolphinConcreteFungibleLedger;
}

impl pallet_asset_manager::Config for Runtime {
    type Event = Event;
    type AssetId = DolphinAssetId;
    type Balance = Balance;
    type Location = AssetLocation;
    type AssetConfig = DolphinAssetConfig;
    type ModifierOrigin = EnsureRoot<AccountId>;
    type PalletId = AssetManagerPalletId;
    type WeightInfo = weights::pallet_asset_manager::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub const MantaPayPalletId: PalletId = MANTA_PAY_PALLET_ID;
}

impl pallet_manta_pay::Config for Runtime {
    type Event = Event;
    type WeightInfo = weights::pallet_manta_pay::SubstrateWeight<Runtime>;
    type AssetConfig = DolphinAssetConfig;
    type PalletId = MantaPayPalletId;
}
