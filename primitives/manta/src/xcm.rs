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
    assets::{AssetConfig, FungibleLedger},
    types::{AssetId, Balance},
};

use sp_runtime::traits::{CheckedConversion, Convert, Zero};
use sp_std::{marker::PhantomData, result, vec};

use frame_support::{
    pallet_prelude::Get,
    traits::{fungibles::Mutate, tokens::ExistenceRequirement},
    weights::{constants::WEIGHT_PER_SECOND, Weight},
};

use crate::{
    assets::{AssetIdLocationGetter, UnitsToWeightRatio},
};
use xcm::{
    latest::{
        prelude::{
            All, Any, BuyExecution, ClearOrigin, Concrete, DepositAsset, InitiateReserveWithdraw,
            Limited, MultiAssets, ReserveAssetDeposited, TransferReserveAsset, Wild, WithdrawAsset,
        },
        Error as XcmError, Result as XcmResult, Xcm,
    },
    v1::{
        AssetId as xcmAssetId, Fungibility,
        Fungibility::*,
        Junction::{AccountId32, Parachain},
        Junctions::*,
        MultiAsset, MultiLocation, NetworkId,
    },
};
use xcm_builder::TakeRevenue;
use xcm_executor::traits::{
    Convert as XcmConvert, FilterAssetLocation, MatchesFungible, MatchesFungibles,
    TransactAsset, WeightTrader,
};

pub trait Reserve {
    /// Returns assets reserve location.
    fn reserve(&self) -> Option<MultiLocation>;
}

// Takes the chain part of a MultiAsset
impl Reserve for MultiAsset {
    fn reserve(&self) -> Option<MultiLocation> {
        // We only care about concrete location now.
        if let xcmAssetId::Concrete(location) = self.id.clone() {
            let first_interior = location.first_interior();
            let parents = location.parent_count();
            match (parents, first_interior) {
                (0, Some(Parachain(id))) => Some(MultiLocation::new(0, X1(Parachain(*id)))),
                (1, Some(Parachain(id))) => Some(MultiLocation::new(1, X1(Parachain(*id)))),
                (1, _) => Some(MultiLocation::parent()),
                _ => None,
            }
        } else {
            None
        }
    }
}

/// A `FilterAssetLocation` implementation. Filters multi native assets whose
/// reserve is same with `origin`.
pub struct MultiNativeAsset;
impl FilterAssetLocation for MultiNativeAsset {
    fn filter_asset_location(asset: &MultiAsset, origin: &MultiLocation) -> bool {
        asset.reserve().map(|r| r == *origin).unwrap_or(false)
    }
}

pub struct AccountIdToMultiLocation<AccountId>(PhantomData<AccountId>);
impl<AccountId> Convert<AccountId, MultiLocation> for AccountIdToMultiLocation<AccountId>
where
    AccountId: Into<[u8; 32]> + Clone,
{
    fn convert(account: AccountId) -> MultiLocation {
        MultiLocation {
            parents: 0,
            interior: X1(AccountId32 {
                network: NetworkId::Any,
                id: account.into(),
            }),
        }
    }
}

// This trader defines how to charge a XCM call.
// This takes the first fungible asset, and takes UnitPerSecondGetter that implements
// UnitToWeightRatio trait.
pub struct FirstAssetTrader<
    AssetId: Clone,
    AssetLocation: From<MultiLocation> + Clone,
    AssetIdInfoGetter: UnitsToWeightRatio + AssetIdLocationGetter<AssetLocation>,
    R: TakeRevenue,
> {
    weight: Weight,
    refund_cache: Option<(MultiLocation, u128, u128)>,
    __: sp_std::marker::PhantomData<(AssetId, AssetLocation, AssetIdInfoGetter, R)>,
}

impl<
        AssetId: Clone,
        AssetLocation: From<MultiLocation> + Clone,
        AssetIdInfoGetter: UnitsToWeightRatio + AssetIdLocationGetter<AssetLocation>,
        R: TakeRevenue,
    > WeightTrader for FirstAssetTrader<AssetId, AssetLocation, AssetIdInfoGetter, R>
{
    fn new() -> Self {
        FirstAssetTrader {
            weight: Zero::zero(),
            refund_cache: None,
            __: sp_std::marker::PhantomData,
        }
    }

    /// buy weight for XCM execution. We always return `TooExpensive` error if this fails.
    fn buy_weight(
        &mut self,
        weight: Weight,
        payment: xcm_executor::Assets,
    ) -> Result<xcm_executor::Assets, XcmError> {
        log::debug!(
            target: "FirstAssetTrader::buy_weight",
            "weight: {:?}, payment: {:?}",
            weight,
            payment
        );

        let first_asset = payment.fungible_assets_iter().next().ok_or({
            log::debug!(
                target: "FirstAssetTrader::buy_weight",
                "no assets in payment: {:?}",
                payment,
            );
            XcmError::TooExpensive
        })?;

        // Check the first asset
        match (first_asset.id, first_asset.fun) {
            (xcmAssetId::Concrete(id), Fungibility::Fungible(_)) => {
                let asset_loc: AssetLocation = id.clone().into();

                let asset_id = AssetIdInfoGetter::get_asset_id(&asset_loc).ok_or({
                    log::debug!(
                        target: "FirstAssetTrader::buy_weight",
                        "asset_id missing for asset_loc with id: {:?}",
                        id,
                    );
                    XcmError::TooExpensive
                })?;

                let units_per_second =
                    AssetIdInfoGetter::get_units_per_second(asset_id).ok_or({
                        log::debug!(
                            target: "FirstAssetTrader::buy_weight",
                            "units_per_second missing for asset with id: {:?}",
                            id,
                        );
                        XcmError::TooExpensive
                    })?;

                let amount = units_per_second * (weight as u128) / (WEIGHT_PER_SECOND as u128);
                // we don't need to proceed if amount is zero.
                // This is very useful in tests.
                if amount.is_zero() {
                    return Ok(payment);
                }
                let required = MultiAsset {
                    fun: Fungibility::Fungible(amount),
                    id: xcmAssetId::Concrete(id.clone()),
                };

                log::debug!(
                    target: "FirstAssetTrader::buy_weight",
                    "payment: {:?}, required: {:?}",
                    payment,
                    required,
                );
                let unused = payment.checked_sub(required).map_err(|_| {
                    log::debug!(
                        target: "FirstAssetTrader::buy_weight",
                        "not enough required assets in payment",
                    );
                    XcmError::TooExpensive
                })?;
                self.weight = self.weight.saturating_add(weight);

                // In case the asset matches the one the trader already stored before, add
                // to later refund

                // Else we are always going to subtract the weight if we can, but we latter do
                // not refund it

                // In short, we only refund on the asset the trader first successfully was able
                // to pay for an execution
                let new_asset = match self.refund_cache.clone() {
                    Some((prev_id, prev_amount, units_per_second)) => {
                        if prev_id == id {
                            Some((id, prev_amount.saturating_add(amount), units_per_second))
                        } else {
                            None
                        }
                    }
                    None => Some((id, amount, units_per_second)),
                };

                // Due to the trait bound, we can only refund one asset.
                if let Some(new_asset) = new_asset {
                    self.weight = self.weight.saturating_add(weight);
                    self.refund_cache = Some(new_asset);
                };
                Ok(unused)
            }
            _ => {
                log::debug!(
                    target: "FirstAssetTrader::buy_weight",
                    "no matching xcmAssetId for first_asset in payment: {:?}",
                    payment,
                );

                Err(XcmError::TooExpensive)
            }
        }
    }

    fn refund_weight(&mut self, weight: Weight) -> Option<MultiAsset> {
        if let Some((id, prev_amount, units_per_second)) = self.refund_cache.clone() {
            let weight = weight.min(self.weight);
            self.weight -= weight;
            let amount = units_per_second * (weight as u128) / (WEIGHT_PER_SECOND as u128);
            self.refund_cache = Some((
                id.clone(),
                prev_amount.saturating_sub(amount),
                units_per_second,
            ));
            Some(MultiAsset {
                fun: Fungibility::Fungible(amount),
                id: xcmAssetId::Concrete(id),
            })
        } else {
            None
        }
    }
}

/// Handle spent fees, deposit them as defined by R
impl<
        AssetId: Clone,
        AssetLocation: From<MultiLocation> + Clone,
        AssetIdInfoGetter: UnitsToWeightRatio + AssetIdLocationGetter<AssetLocation>,
        R: TakeRevenue,
    > Drop for FirstAssetTrader<AssetId, AssetLocation, AssetIdInfoGetter, R>
{
    fn drop(&mut self) {
        if let Some((id, amount, _)) = self.refund_cache.clone() {
            R::take_revenue((id, amount).into());
        }
    }
}

/// XCM fee depositor to which we implement the TakeRevenue trait
/// It receives a fungibles::Mutate implemented argument, a matcher to convert MultiAsset into
/// AssetId and amount, and the fee receiver account
pub struct XcmFeesToAccount<Assets, Matcher, AccountId, ReceiverAccount>(
    PhantomData<(Assets, Matcher, AccountId, ReceiverAccount)>,
);
impl<
        Assets: Mutate<AccountId>,
        Matcher: MatchesFungibles<Assets::AssetId, Assets::Balance>,
        AccountId: Clone,
        ReceiverAccount: Get<AccountId>,
    > TakeRevenue for XcmFeesToAccount<Assets, Matcher, AccountId, ReceiverAccount>
{
    fn take_revenue(revenue: MultiAsset) {
        match Matcher::matches_fungibles(&revenue) {
            Ok((asset_id, amount)) => {
                if !amount.is_zero() {
                    Assets::mint_into(asset_id, &ReceiverAccount::get(), amount)
                        .map_err(
                            |err| log::debug!(target: "manta-xcm", "mint_into failed with {:?}", err),
                        )
                        .ok();
                }
            }
            Err(_) => log::debug!(
                target: "manta-xcm",
                "take revenue failed matching fungible"
            ),
        }
    }
}

/// Manta's `MatchFungible` implementation.
/// It resolves the reanchoring logic as well, i.e. it recognize `here()` as
/// `../parachain(id)`.
/// `T` should specify a `SelfLocation` in the form of absolute path to the
/// relaychain.
pub struct IsNativeConcrete<T>(PhantomData<T>);
impl<T, Balance> MatchesFungible<Balance> for IsNativeConcrete<T>
where
    T: Get<MultiLocation>,
    Balance: TryFrom<u128>,
{
    fn matches_fungible(a: &MultiAsset) -> Option<Balance> {
        if let (Fungible(ref amount), Concrete(ref location)) = (&a.fun, &a.id) {
            if location == &T::get() || MultiLocation::is_here(location) {
                return CheckedConversion::checked_from(*amount);
            }
        }
        None
    }
}

pub struct MultiAssetAdapter<
    Runtime,
    AccountIdConverter,
    NativeMatcher,
    NonNativeMatcher,
    MultiAdapterFungibleLedger,
    MultiAdapterAssetConfig,
>(
    PhantomData<(
        Runtime,
        NativeMatcher,
        AccountIdConverter,
        NonNativeMatcher,
        MultiAdapterFungibleLedger,
        MultiAdapterAssetConfig,
    )>,
);

impl<
        Runtime: frame_system::Config,
        AccountIdConverter: XcmConvert<MultiLocation, Runtime::AccountId>,
        NativeMatcher: MatchesFungible<Balance>,
        NonNativeMatcher: MatchesFungibles<AssetId, Balance>,
        MultiAdapterFungibleLedger: FungibleLedger<Runtime>,
        MultiAdapterAssetConfig: AssetConfig<Runtime>,
    > TransactAsset
    for MultiAssetAdapter<
        Runtime,
        AccountIdConverter,
        NativeMatcher,
        NonNativeMatcher,
        MultiAdapterFungibleLedger,
        MultiAdapterAssetConfig,
    >
{
    fn deposit_asset(asset: &MultiAsset, location: &MultiLocation) -> XcmResult {
        log::debug!(
            target: "xcm::multi_asset_adapter",
            "deposit_asset asset: {:?}, location: {:?}",
            asset, location,
        );

        let (asset_id, amount, who) = Self::match_asset_and_location(asset, location)?;

        // If it's non-native asset we want to check with increase in total supply.
        // Otherwise it will just use false, as it is assumed the native asset supply cannot be changed.
        MultiAdapterFungibleLedger::can_deposit(asset_id, &who, amount, true).map_err(|_| {
            XcmError::FailedToTransactAsset("Failed MultiAdapterFungibleLedger::can_deposit")
        })?;

        MultiAdapterFungibleLedger::deposit_can_mint(asset_id, &who, amount).map_err(|_| {
            XcmError::FailedToTransactAsset("Failed MultiAdapterFungibleLedger::deposit_can_mint")
        })?;

        Ok(())
    }

    fn withdraw_asset(
        asset: &MultiAsset,
        location: &MultiLocation,
    ) -> result::Result<xcm_executor::Assets, XcmError> {
        log::debug!(
            target: "xcm::multi_asset_adapter",
            "withdraw_asset asset: {:?}, location: {:?}",
            asset, location,
        );

        let (asset_id, amount, who) = Self::match_asset_and_location(asset, location)?;

        MultiAdapterFungibleLedger::withdraw_can_burn(
            asset_id,
            &who,
            amount,
            ExistenceRequirement::AllowDeath,
        )
        .map_err(|_| XcmError::FailedToTransactAsset("Failed Burn"))?;

        Ok(asset.clone().into())
    }
}

impl<
        Runtime: frame_system::Config,
        AccountIdConverter: XcmConvert<MultiLocation, Runtime::AccountId>,
        NativeMatcher: MatchesFungible<Balance>,
        NonNativeMatcher: MatchesFungibles<AssetId, Balance>,
        MultiAdapterFungibleLedger: FungibleLedger<Runtime>,
        MultiAdapterAssetConfig: AssetConfig<Runtime>,
    >
    MultiAssetAdapter<
        Runtime,
        AccountIdConverter,
        NativeMatcher,
        NonNativeMatcher,
        MultiAdapterFungibleLedger,
        MultiAdapterAssetConfig,
    >
{
    /// Matches the incoming `asset` to an `asset_id` and `amount` on this chain.
    /// Matches the incoming `location` to a `receiver` account on this chain.
    /// Uses the matcher implementation of both native and non-native assets.
    /// Returns the `asset_id`, `amount` and `receiver` if all three were matched.
    fn match_asset_and_location(
        asset: &MultiAsset,
        location: &MultiLocation,
    ) -> result::Result<(AssetId, Balance, Runtime::AccountId), XcmError> {
        let receiver = AccountIdConverter::convert_ref(location).map_err(|_| {
            XcmError::FailedToTransactAsset("Failed Location to AccountId Conversion")
        })?;

        let (asset_id, amount) = match (
            NativeMatcher::matches_fungible(asset),
            NonNativeMatcher::matches_fungibles(asset),
        ) {
            // native asset
            (Some(amount), _) => (MultiAdapterAssetConfig::NativeAssetId::get(), amount),
            // assets asset
            (_, Ok((asset_id, amount))) => (asset_id, amount),
            // unknown asset
            _ => return Err(XcmError::FailedToTransactAsset("Unknown Asset")),
        };

        Ok((asset_id, amount, receiver))
    }
}

// 4_000_000_000 is a typical configuration value provided to dApp developers for `dest_weight`
// argument when sending xcm message to Calamari. ie moonbeam, sub-wallet, phala, etc
pub const ADVERTISED_DEST_WEIGHT: u64 = 4_000_000_000;

// Composition of self_reserve message composed by xTokens on the sender side
pub fn self_reserve_xcm_message_receiver_side<T>() -> Xcm<T> {
    Xcm(vec![
        ReserveAssetDeposited(MultiAssets::from(vec![MultiAsset {
            id: Concrete(MultiLocation {
                parents: 1,
                interior: X1(Parachain(1)),
            }),
            fun: Fungible(10000000000000),
        }])),
        ClearOrigin,
        BuyExecution {
            fees: MultiAsset {
                id: Concrete(MultiLocation {
                    parents: 1,
                    interior: X1(Parachain(1)),
                }),
                fun: Fungible(10000000000000),
            },
            weight_limit: Limited(3999999999),
        },
        DepositAsset {
            assets: Wild(All),
            max_assets: 1,
            beneficiary: MultiLocation {
                parents: 0,
                interior: X1(AccountId32 {
                    network: Any,
                    id: [
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0,
                    ],
                }),
            },
        },
    ])
}

// Composition of to_reserve message composed by xTokens on the receiver side
pub fn to_reserve_xcm_message_receiver_side<T>() -> Xcm<T> {
    Xcm(vec![
        WithdrawAsset(MultiAssets::from(vec![MultiAsset {
            id: Concrete(MultiLocation {
                parents: 1,
                interior: X1(Parachain(1)),
            }),
            fun: Fungible(10000000000000),
        }])),
        ClearOrigin,
        BuyExecution {
            fees: MultiAsset {
                id: Concrete(MultiLocation {
                    parents: 1,
                    interior: X1(Parachain(1)),
                }),
                fun: Fungible(10000000000000),
            },
            weight_limit: Limited(3999999999),
        },
        DepositAsset {
            assets: Wild(All),
            max_assets: 1,
            beneficiary: MultiLocation {
                parents: 0,
                interior: X1(AccountId32 {
                    network: Any,
                    id: [
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0,
                    ],
                }),
            },
        },
    ])
}

// Composition of to_reserve message composed by xTokens on the sender side
pub fn to_reserve_xcm_message_sender_side<T>() -> Xcm<T> {
    let dummy_multi_location = MultiLocation {
        parents: 1,
        interior: X1(Parachain(1)),
    };
    let dummy_assets = MultiAssets::from(vec![MultiAsset {
        id: Concrete(MultiLocation {
            parents: 1,
            interior: X1(Parachain(1)),
        }),
        fun: Fungible(10000000000000),
    }]);
    Xcm(vec![
        WithdrawAsset(dummy_assets),
        InitiateReserveWithdraw {
            assets: Wild(All),
            reserve: dummy_multi_location.clone(),
            xcm: Xcm(vec![
                BuyExecution {
                    fees: MultiAsset {
                        id: Concrete(dummy_multi_location),
                        fun: Fungible(10000000000000),
                    },
                    weight_limit: Limited(3999999999),
                },
                DepositAsset {
                    assets: Wild(All),
                    max_assets: 1,
                    beneficiary: MultiLocation {
                        parents: 0,
                        interior: X1(AccountId32 {
                            network: Any,
                            id: [
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                            ],
                        }),
                    },
                },
            ]),
        },
    ])
}

// Composition of self_reserve message composed by xTokens on the sender side
pub fn self_reserve_xcm_message_sender_side<T>() -> Xcm<T> {
    let dummy_multi_location = MultiLocation {
        parents: 1,
        interior: X1(Parachain(1)),
    };
    let dummy_assets = MultiAssets::from(vec![MultiAsset {
        id: Concrete(MultiLocation {
            parents: 1,
            interior: X1(Parachain(1)),
        }),
        fun: Fungible(10000000000000),
    }]);
    Xcm(vec![TransferReserveAsset {
        assets: dummy_assets,
        dest: dummy_multi_location.clone(),
        xcm: Xcm(vec![
            BuyExecution {
                fees: MultiAsset {
                    id: Concrete(dummy_multi_location),
                    fun: Fungible(10000000000000),
                },
                weight_limit: Limited(3999999999),
            },
            DepositAsset {
                assets: Wild(All),
                max_assets: 1,
                beneficiary: MultiLocation {
                    parents: 0,
                    interior: X1(AccountId32 {
                        network: Any,
                        id: [
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0, 0, 0,
                        ],
                    }),
                },
            },
        ]),
    }])
}
