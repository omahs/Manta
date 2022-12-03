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

//! Asset Utilities
use crate::{
    constants::TEST_DEFAULT_ASSET_ED,
    nft::NonFungibleLedger,
    types::{Balance as MantaBalance, CalamariAssetId},
};
use alloc::vec::Vec;
use codec::{Decode, Encode};
use core::{borrow::Borrow, marker::PhantomData};
use frame_support::{
    dispatch::DispatchError,
    pallet_prelude::Get,
    traits::tokens::{
        currency::Currency,
        fungible,
        fungibles::{self, Mutate, Transfer},
        DepositConsequence, ExistenceRequirement, WithdrawReasons,
    },
    Parameter,
};
use frame_system::Config;
use scale_info::TypeInfo;
use xcm::{
    v1::{Junctions, MultiLocation},
    VersionedMultiLocation,
};
use xcm_executor::traits::Convert;

/// Asset Id
pub trait AssetIdType {
    /// Asset Id Type
    type AssetId;
}

/// Asset Id Type
pub type AssetId<T> = <T as AssetIdType>::AssetId;

/// Balance
pub trait BalanceType {
    /// Balance Type
    type Balance;
}

/// Balance Type
pub type Balance<T> = <T as BalanceType>::Balance;

/// Location
pub trait LocationType {
    /// Location Type
    type Location;
}

/// Location Type
pub type Location<T> = <T as LocationType>::Location;

/// Asset Metadata
pub trait AssetMetadata: BalanceType + AssetIdType {
    /// Returns the minimum balance to hold this asset.
    fn min_balance(&self) -> &Self::Balance;

    /// Returns a boolean value indicating whether this asset needs an existential deposit.
    fn is_sufficient(&self) -> bool;

    /// Returns AssetId for `AssetManager`
    fn asset_id(&self) -> &Self::AssetId;
}

/// Asset Registry
///
/// The registry trait: defines the interface of creating an asset in the asset implementation
/// layer. We may revisit this interface design (e.g. add change asset interface). However, change
/// in StorageMetadata should be rare.
pub trait AssetRegistry<C>: AssetIdType + BalanceType
where
    C: Config,
{
    /// Metadata Type
    type Metadata: IsFungible<Self::AssetId, Self::AssetId, Self::AssetId>;

    /// Error Type
    type Error;

    /// TODO: docs
    fn create_asset(
        origin: C::Origin,
        asset_id: Self::AssetId,
        admin: C::AccountId,
        metadata: Self::Metadata,
    ) -> Result<(), Self::Error>;

    /// Creates an new asset.
    ///
    /// * `asset_id`: the asset id to be created
    /// * `metadata`: the metadata that the implementation layer stores
    /// * `min_balance`: the minimum balance to hold this asset
    /// * `is_sufficient`: whether this asset can be used as reserve asset,
    ///     to the first approximation. More specifically, Whether a non-zero balance of this asset
    ///     is deposit of sufficient value to account for the state bloat associated with its
    ///     balance storage. If set to `true`, then non-zero balances may be stored without a
    ///     `consumer` reference (and thus an ED in the Balances pallet or whatever else is used to
    ///     control user-account state growth).
    fn force_create_asset(
        asset_id: Self::AssetId,
        metadata: Self::Metadata,
    ) -> Result<(), Self::Error>;

    /// Update asset metadata by `AssetId`.
    ///
    /// * `asset_id`: the asset id to be created.
    /// * `metadata`: the metadata that the implementation layer stores.
    fn force_update_metadata(
        asset_id: &Self::AssetId,
        metadata: Self::Metadata,
    ) -> Result<(), Self::Error>;

    /// docs
    fn get_metadata(asset_id: &Self::AssetId) -> Option<Self::Metadata>;
}

/// Asset Configuration
pub trait AssetConfig<C>: AssetIdType + BalanceType + LocationType
where
    C: Config,
{
    /// Metadata type that required in token storage: e.g. AssetMetadata in Pallet-Assets.
    type StorageMetadata: From<Self::AssetRegistryMetadata>
        + IsFungible<Self::AssetId, Self::AssetId, Self::AssetId>;

    /// The Asset Metadata type stored in this pallet.
    type AssetRegistryMetadata: Parameter + IsFungible<Self::AssetId, Self::AssetId, Self::AssetId>;

    /// The identifier of asset id in manta pay to id within pallet
    type IdentifierMapping: Parameter
        + IsFungible<Self::AssetId, Self::AssetId, Self::AssetId>
        + IdentifierMapping<Self::AssetId, Self::AssetId, Self::AssetId>
        + From<Self::AssetRegistryMetadata>;

    /// The AssetId that the non-native asset starts from.
    ///
    /// A typical configuration is 8, so that asset 0 - 7 is reserved.
    type StartNonNativeAssetId: Get<Self::AssetId>;

    /// The Native Asset Id, a typical configuration is 1.
    type NativeAssetId: Get<Self::AssetId>;

    /// Native Asset Location
    type NativeAssetLocation: Get<Self::Location>;

    /// Native Asset Metadata
    type NativeAssetMetadata: Get<Self::AssetRegistryMetadata>;

    /// Asset Registry
    ///
    /// The trait we use to register Assets and mint assets.
    type AssetRegistry: AssetRegistry<
        C,
        AssetId = Self::AssetId,
        Balance = Self::Balance,
        Metadata = Self::StorageMetadata,
        Error = DispatchError,
    >;

    /// Fungible Ledger
    type FungibleLedger: FungibleLedger<
        AccountId = C::AccountId,
        AssetId = Self::AssetId,
        Balance = Self::Balance,
    >;

    /// Fungible Ledger
    type NonFungibleLedger: NonFungibleLedger<
        AccountId = C::AccountId,
        AssetId = Self::AssetId,
        Balance = Self::Balance,
    >;
}

/// Fungible Asset Storage Metadata
#[derive(Clone, Debug, Decode, Encode, Eq, Hash, Ord, PartialEq, PartialOrd, TypeInfo)]
pub struct FungibleAssetStorageMetadata {
    /// Asset Name
    pub name: Vec<u8>,

    /// Asset Symbol
    pub symbol: Vec<u8>,

    /// Number of Decimals
    pub decimals: u8,

    /// Frozen Flag
    ///
    /// Whether or not transfers of the asset are allowed.
    pub is_frozen: bool,
}

/// Non Fungible Asset Storage Metadata
#[derive(Clone, Debug, Decode, Encode, Eq, Hash, Ord, PartialEq, PartialOrd, TypeInfo)]
pub struct NonFungibleAssetStorageMetadata<CollectionId, ItemId> {
    /// Asset Name
    pub name: Vec<u8>,

    /// Asset info
    pub info: Vec<u8>,

    /// Collection Id
    pub collection_id: CollectionId,

    /// Item Id
    pub item_id: ItemId,
}

/// If data is fungible
pub trait IsFungible<AssetId, CollectionId, ItemId> {
    /// Is fungible
    fn is_fungible(&self) -> bool;

    /// Asset Id of fungible
    fn get_fungible_id(&self) -> Option<&AssetId>;

    /// Collection and Item Id of NonFungible
    fn get_non_fungible_id(&self) -> Option<(&CollectionId, &ItemId)>;
}

/// Identifiew in the `pallet-assets` and `pallet-uniques`
pub trait IdentifierMapping<AssetId, CollectionId, ItemId> {
    /// new fungible asset
    fn new_fungible(asset_id: AssetId) -> Self;

    /// new non-fungible asset
    fn new_non_fungible(collection_id: CollectionId, item_id: ItemId) -> Self;
}

/// Asset Storage Metadata
#[derive(Clone, Debug, Decode, Encode, Eq, Hash, Ord, PartialEq, PartialOrd, TypeInfo)]
pub enum AssetStorageMetadata<Balance, AssetId, CollectionId, ItemId> {
    /// Metadata for Fungible Assets
    Fungible(AssetRegistryMetadata<Balance, AssetId>),
    /// Metadata for NonFungible Assets
    NonFungible(NonFungibleAssetStorageMetadata<CollectionId, ItemId>),
}

/// Asset Id in `pallet-assets` or `pallets-uniques`
#[derive(Clone, Debug, Decode, Encode, Eq, Hash, Ord, PartialEq, PartialOrd, TypeInfo)]
pub enum AssetIdMapping<AssetId, CollectionId, ItemId> {
    /// Metadata for Fungible Assets
    Fungible(AssetId),
    /// Metadata for NonFungible Assets
    NonFungible((CollectionId, ItemId)),
}

impl<B, A, C, I> From<AssetStorageMetadata<B, A, C, I>> for AssetIdMapping<A, C, I> {
    fn from(asset_metadata: AssetStorageMetadata<B, A, C, I>) -> Self {
        match asset_metadata {
            AssetStorageMetadata::<B, A, C, I>::Fungible(meta) => Self::Fungible(meta.asset_id),
            AssetStorageMetadata::<B, A, C, I>::NonFungible(meta) => {
                Self::NonFungible((meta.collection_id, meta.item_id))
            }
        }
    }
}

impl<A, C, I> IdentifierMapping<A, C, I> for AssetIdMapping<A, C, I> {
    fn new_fungible(asset_id: A) -> Self {
        Self::Fungible(asset_id)
    }

    fn new_non_fungible(collection_id: C, item_id: I) -> Self {
        Self::NonFungible((collection_id, item_id))
    }
}

impl<A, C, I> IsFungible<A, C, I> for AssetIdMapping<A, C, I> {
    fn is_fungible(&self) -> bool {
        match self {
            Self::Fungible(_) => true,
            Self::NonFungible(_) => true,
        }
    }

    fn get_fungible_id(&self) -> Option<&A> {
        match self {
            Self::Fungible(asset_id) => Some(asset_id),
            Self::NonFungible(_) => None,
        }
    }

    fn get_non_fungible_id(&self) -> Option<(&C, &I)> {
        match self {
            Self::Fungible(_) => None,
            Self::NonFungible((collection_id, item_id)) => Some((&collection_id, &item_id)),
        }
    }
}

impl<B, A, C, I> IsFungible<A, C, I> for AssetStorageMetadata<B, A, C, I> {
    fn is_fungible(&self) -> bool {
        match self {
            Self::Fungible(_) => true,
            Self::NonFungible(_) => false,
        }
    }

    fn get_fungible_id(&self) -> Option<&A> {
        match self {
            Self::Fungible(meta) => Some(meta.asset_id()),
            Self::NonFungible(_) => None,
        }
    }

    fn get_non_fungible_id(&self) -> Option<(&C, &I)> {
        match self {
            Self::Fungible(_) => None,
            Self::NonFungible(meta) => Some((&meta.collection_id, &meta.item_id)),
        }
    }
}

impl<A, B, C, I> From<AssetRegistryMetadata<B, A>> for AssetStorageMetadata<B, A, C, I> {
    #[inline]
    fn from(source: AssetRegistryMetadata<B, A>) -> Self {
        Self::Fungible(source)
    }
}

/// Asset Registry Metadata for Fungibles
#[derive(Clone, Debug, Decode, Encode, Eq, Hash, Ord, PartialEq, PartialOrd, TypeInfo)]
pub struct AssetRegistryMetadata<B, A> {
    /// Asset Storage Metadata for fungible assets
    pub metadata: FungibleAssetStorageMetadata,

    /// Asset Id
    pub asset_id: A,

    /// Minimum Balance
    pub min_balance: B,

    /// Sufficiency Flag
    ///
    /// This flag should be set to `true` whenever a non-zero balance of this asset is deposit of
    /// sufficient value to account for the state bloat associated with its balance storage. If set
    /// to `true`, then non-zero balances may be stored without a `consumer` reference (and thus an
    /// existential deposit in the balances pallet or whatever else is used to control user-account
    /// state growth).
    ///
    /// If this flag is set to `false`, a fresh account cannot receive XCM tokens.
    pub is_sufficient: bool,
}

/// Because there is no obvious defaults for an asset's metadata, we explicitly
/// name a trait to carry this logic for testing and benchmarking
pub trait TestingDefault {
    /// Returns some default asset metadata
    fn testing_default() -> Self;
}

impl TestingDefault for AssetRegistryMetadata<MantaBalance, CalamariAssetId> {
    fn testing_default() -> Self {
        Self {
            metadata: FungibleAssetStorageMetadata {
                name: b"Default".to_vec(),
                symbol: b"DEF".to_vec(),
                decimals: 12,
                is_frozen: false,
            },
            asset_id: 0,
            min_balance: TEST_DEFAULT_ASSET_ED,
            is_sufficient: true,
        }
    }
}

impl<B, A> BalanceType for AssetRegistryMetadata<B, A> {
    type Balance = B;
}

impl<B, A> AssetIdType for AssetRegistryMetadata<B, A> {
    type AssetId = A;
}

impl<B, A> AssetMetadata for AssetRegistryMetadata<B, A> {
    #[inline]
    fn min_balance(&self) -> &B {
        &self.min_balance
    }

    #[inline]
    fn is_sufficient(&self) -> bool {
        self.is_sufficient
    }

    #[inline]
    fn asset_id(&self) -> &A {
        &self.asset_id
    }
}

/// Asset Location
#[derive(Clone, Debug, Decode, Encode, Eq, PartialEq, TypeInfo)]
pub struct AssetLocation(pub VersionedMultiLocation);

impl Default for AssetLocation {
    #[inline]
    fn default() -> Self {
        Self(VersionedMultiLocation::V1(MultiLocation {
            parents: 0,
            interior: Junctions::Here,
        }))
    }
}

impl From<MultiLocation> for AssetLocation {
    /// Converts a [`MultiLocation`] into an [`AssetLocation`].
    ///
    /// # Safety
    ///
    /// This method does not guarantee that the output [`AssetLocation`] is registered, i.e. has a
    /// valid [`AssetId`].
    #[inline]
    fn from(location: MultiLocation) -> Self {
        AssetLocation(VersionedMultiLocation::V1(location))
    }
}

impl From<AssetLocation> for Option<MultiLocation> {
    /// Converts an [`AssetLocation`] into an optional [`MultiLocation`], returning `None` if it
    /// represents a native asset.
    #[inline]
    fn from(location: AssetLocation) -> Self {
        match location {
            AssetLocation(VersionedMultiLocation::V1(location)) => Some(location),
            _ => None,
        }
    }
}

///
pub trait AssetIdLocationMap: AssetIdType + LocationType {
    /// Returns the [`Location`](LocationType::Location) of `asset_id`.
    fn location(asset_id: &Self::AssetId) -> Option<Self::Location>;

    /// Returns the [`AssetId`](AssetIdType::AssetId) located at `location`.
    fn asset_id(location: &Self::Location) -> Option<Self::AssetId>;
}

/// Defines the units per second charged given an `AssetId`.
pub trait UnitsPerSecond: AssetIdType {
    /// Returns the units per second for `asset_id`.
    fn units_per_second(asset_id: &Self::AssetId) -> Option<u128>;
}

/// Converter struct implementing `Convert`. MultiLocation to AssetId and the reverse.
pub struct AssetIdLocationConvert<M>(PhantomData<M>);

impl<M> Convert<MultiLocation, M::AssetId> for AssetIdLocationConvert<M>
where
    M: AssetIdLocationMap,
    M::AssetId: Clone,
    M::Location: Clone + From<MultiLocation> + Into<Option<MultiLocation>>,
{
    #[inline]
    fn convert_ref(location: impl Borrow<MultiLocation>) -> Result<M::AssetId, ()> {
        M::asset_id(&location.borrow().clone().into()).ok_or(())
    }

    #[inline]
    fn reverse_ref(asset_id: impl Borrow<M::AssetId>) -> Result<MultiLocation, ()> {
        M::location(asset_id.borrow())
            .and_then(Into::into)
            .ok_or(())
    }
}

/// Fungible Ledger Error
#[derive(Clone, Copy, Debug, Decode, Encode, Eq, PartialEq, TypeInfo)]
pub enum FungibleLedgerError<I, B> {
    /// Invalid Asset Id
    InvalidAssetId(I),

    /// Deposit couldn't happen due to the amount being too low. This is usually because the account
    /// doesn't yet exist and the deposit wouldn't bring it to at least the minimum needed for
    /// existence.
    BelowMinimum,

    /// Deposit cannot happen since the account cannot be created (usually because it's a consumer
    /// and there exists no provider reference).
    CannotCreate,

    /// The asset is unknown. Usually because an `AssetId` has been presented which doesn't exist
    /// on the system.
    UnknownAsset,

    /// An overflow would occur. This is practically unexpected, but could happen in test systems
    /// with extremely small balance types or balances that approach the max value of the balance
    /// type.
    Overflow,

    /// Cannot withdraw more than the specified amount
    CannotWithdrawMoreThan(B),

    /// Unable to Mint an Asset
    InvalidMint(DispatchError),

    /// Unable to Burn an Asset
    InvalidBurn(DispatchError),

    /// Unable to Transfer an Asset
    InvalidTransfer(DispatchError),
}

impl<I, B> FungibleLedgerError<I, B> {
    /// Converts a deposit `consequence` into a [`FungibleLedgerError`] or into `Ok(())` when the
    /// value of `consequence` is [`Success`](DepositConsequence::Success).
    #[inline]
    pub fn from_deposit(consequence: DepositConsequence) -> Result<(), Self> {
        Err(match consequence {
            DepositConsequence::BelowMinimum => Self::BelowMinimum,
            DepositConsequence::CannotCreate => Self::CannotCreate,
            DepositConsequence::Overflow => Self::Overflow,
            DepositConsequence::UnknownAsset => Self::UnknownAsset,
            DepositConsequence::Success => return Ok(()),
        })
    }
}

impl<I, B> AssetIdType for FungibleLedgerError<I, B> {
    type AssetId = I;
}

impl<I, B> BalanceType for FungibleLedgerError<I, B> {
    type Balance = B;
}

/// Unified Interface for Fungible Assets
///
/// This trait unifies the interface for the [`fungible`] and [`fungibles`] modules.
///
/// It is assumed that the supply of native asset cannot be changed, while the supply of non-native
/// assets can increase or decrease.
pub trait FungibleLedger: AssetIdType + BalanceType {
    /// Account Id Type
    type AccountId;

    /// Checks if an asset id is valid and returning and [`Error`](FungibleLedgerError) otherwise.
    fn ensure_valid(
        asset_id: Self::AssetId,
    ) -> Result<Self::AssetId, FungibleLedgerError<Self::AssetId, Self::Balance>>;

    /// Check whether `account` can increase its balance by `amount` in the given `asset_id`.
    fn can_deposit(
        asset_id: Self::AssetId,
        account: &Self::AccountId,
        amount: Self::Balance,
        can_increase_total_supply: bool,
    ) -> Result<Self::AssetId, FungibleLedgerError<Self::AssetId, Self::Balance>>;

    /// Deposit `amount` of an asset with the given `asset_id` to `account`.
    fn deposit_minting(
        asset_id: Self::AssetId,
        account: &Self::AccountId,
        amount: Self::Balance,
    ) -> Result<(), FungibleLedgerError<Self::AssetId, Self::Balance>>;

    /// Checks if `amount` of `asset_id` can be deposited into `account` and then deposits it if
    /// that is successful.
    fn deposit_minting_with_check(
        asset_id: Self::AssetId,
        account: &Self::AccountId,
        amount: Self::Balance,
        can_increase_total_supply: bool,
    ) -> Result<(), FungibleLedgerError<Self::AssetId, Self::Balance>>;

    /// Performs a transfer from `source` to `destination` of
    fn transfer(
        asset_id: Self::AssetId,
        source: &Self::AccountId,
        destination: &Self::AccountId,
        amount: Self::Balance,
        existence_requirement: ExistenceRequirement,
    ) -> Result<(), FungibleLedgerError<Self::AssetId, Self::Balance>>;

    /// Check whether `account` can decrease its balance by `amount` in the given `asset_id`.
    fn can_withdraw(
        asset_id: Self::AssetId,
        account: &Self::AccountId,
        amount: &Self::Balance,
        existence_requirement: ExistenceRequirement,
    ) -> Result<Self::AssetId, FungibleLedgerError<Self::AssetId, Self::Balance>>;

    /// Performs a withdraw from `who` for `amount` of `asset_id`
    fn withdraw_burning(
        asset_id: Self::AssetId,
        who: &Self::AccountId,
        amount: Self::Balance,
        existence_requirement: ExistenceRequirement,
    ) -> Result<(), FungibleLedgerError<Self::AssetId, Self::Balance>>;
}

/// Fungible Ledger Implementation for Native and NonNative Assets
///
/// The `Native` assets are defined by an implementation of a [`fungible`] asset and the `NonNative`
/// by a set of fungible assets using [`fungibles`].
pub struct NativeAndNonNative<C, A, Native, NonNative> {
    ///  Type Parameter Marker
    __: PhantomData<(C, A, Native, NonNative)>,
}

impl<C, A, Native, NonNative> AssetIdType for NativeAndNonNative<C, A, Native, NonNative>
where
    C: Config,
    A: AssetConfig<C>,
{
    type AssetId = A::AssetId;
}

impl<C, A, Native, NonNative> BalanceType for NativeAndNonNative<C, A, Native, NonNative>
where
    C: Config,
    A: AssetConfig<C>,
{
    type Balance = A::Balance;
}

impl<C, A, Native, NonNative> FungibleLedger for NativeAndNonNative<C, A, Native, NonNative>
where
    C: Config,
    A: AssetConfig<C>,
    A::AssetId: Clone + PartialOrd,
    A::Balance: Clone + PartialOrd,
    Native: fungible::Inspect<C::AccountId, Balance = A::Balance>
        + Currency<C::AccountId, Balance = A::Balance>,
    NonNative: fungibles::Inspect<C::AccountId, AssetId = A::AssetId, Balance = A::Balance>
        + Mutate<C::AccountId>
        + Transfer<C::AccountId>,
{
    type AccountId = C::AccountId;

    #[inline]
    fn ensure_valid(
        asset_id: Self::AssetId,
    ) -> Result<Self::AssetId, FungibleLedgerError<Self::AssetId, Self::Balance>> {
        if asset_id >= A::StartNonNativeAssetId::get() || asset_id == A::NativeAssetId::get() {
            Ok(asset_id)
        } else {
            Err(FungibleLedgerError::InvalidAssetId(asset_id))
        }
    }

    /// Non-native assets will use the `can_increase_total_supply` flag, while native assets will
    /// not.
    #[inline]
    fn can_deposit(
        asset_id: Self::AssetId,
        account: &C::AccountId,
        amount: Self::Balance,
        can_increase_total_supply: bool,
    ) -> Result<Self::AssetId, FungibleLedgerError<Self::AssetId, Self::Balance>> {
        let asset_id = Self::ensure_valid(asset_id)?;
        FungibleLedgerError::from_deposit(if asset_id == A::NativeAssetId::get() {
            Native::can_deposit(account, amount, false)
        } else {
            NonNative::can_deposit(asset_id.clone(), account, amount, can_increase_total_supply)
        })
        .map(|_| asset_id)
    }

    /// Will mint and increase the total supply of non-native assets.
    #[inline]
    fn deposit_minting(
        asset_id: Self::AssetId,
        account: &C::AccountId,
        amount: Self::Balance,
    ) -> Result<(), FungibleLedgerError<Self::AssetId, Self::Balance>> {
        let asset_id = Self::ensure_valid(asset_id)?;
        if asset_id == A::NativeAssetId::get() {
            Native::deposit_creating(account, amount);
        } else {
            NonNative::mint_into(asset_id, account, amount)
                .map_err(FungibleLedgerError::InvalidMint)?;
        }
        Ok(())
    }

    #[inline]
    fn deposit_minting_with_check(
        asset_id: Self::AssetId,
        account: &Self::AccountId,
        amount: Self::Balance,
        can_increase_total_supply: bool,
    ) -> Result<(), FungibleLedgerError<Self::AssetId, Self::Balance>> {
        let asset_id = Self::ensure_valid(asset_id)?;
        if asset_id == A::NativeAssetId::get() {
            FungibleLedgerError::from_deposit(Native::can_deposit(account, amount.clone(), false))?;
            Native::deposit_creating(account, amount);
        } else {
            FungibleLedgerError::from_deposit(NonNative::can_deposit(
                asset_id.clone(),
                account,
                amount.clone(),
                can_increase_total_supply,
            ))?;
            NonNative::mint_into(asset_id, account, amount)
                .map_err(FungibleLedgerError::InvalidMint)?;
        }
        Ok(())
    }

    #[inline]
    fn transfer(
        asset_id: Self::AssetId,
        source: &C::AccountId,
        destination: &C::AccountId,
        amount: Self::Balance,
        existence_requirement: ExistenceRequirement,
    ) -> Result<(), FungibleLedgerError<Self::AssetId, Self::Balance>> {
        let asset_id = Self::ensure_valid(asset_id)?;
        if asset_id == A::NativeAssetId::get() {
            Native::transfer(source, destination, amount, existence_requirement)
        } else {
            NonNative::transfer(
                asset_id,
                source,
                destination,
                amount,
                match existence_requirement {
                    ExistenceRequirement::KeepAlive => true,
                    ExistenceRequirement::AllowDeath => false,
                },
            )
            .map(|_| ())
        }
        .map_err(FungibleLedgerError::InvalidTransfer)
    }

    #[inline]
    fn can_withdraw(
        asset_id: Self::AssetId,
        account: &C::AccountId,
        amount: &Self::Balance,
        existence_requirement: ExistenceRequirement,
    ) -> Result<Self::AssetId, FungibleLedgerError<Self::AssetId, Self::Balance>> {
        let asset_id = Self::ensure_valid(asset_id)?;
        let keep_alive = match existence_requirement {
            ExistenceRequirement::KeepAlive => true,
            ExistenceRequirement::AllowDeath => false,
        };
        let reducible_amount = if asset_id == A::NativeAssetId::get() {
            Native::reducible_balance(account, keep_alive)
        } else {
            NonNative::reducible_balance(asset_id.clone(), account, keep_alive)
        };
        if reducible_amount >= *amount {
            return Ok(asset_id);
        }
        Err(FungibleLedgerError::CannotWithdrawMoreThan(
            reducible_amount,
        ))
    }

    /// Will burn and decrease total supply of non-native assets.
    #[inline]
    fn withdraw_burning(
        asset_id: Self::AssetId,
        who: &C::AccountId,
        amount: Self::Balance,
        existence_requirement: ExistenceRequirement,
    ) -> Result<(), FungibleLedgerError<Self::AssetId, Self::Balance>> {
        let asset_id = Self::can_withdraw(
            Self::ensure_valid(asset_id)?,
            who,
            &amount,
            existence_requirement,
        )?;
        if asset_id == A::NativeAssetId::get() {
            Native::withdraw(
                who,
                amount,
                WithdrawReasons::TRANSFER,
                existence_requirement,
            )
            .map_err(FungibleLedgerError::InvalidBurn)?;
        } else {
            // NOTE: The `existence_requirement` is used in the `can_reduce_by_amount` checks,
            //       so it doesn't matter that `burn_from` uses `allow_death` by default in this
            //       implementation.
            NonNative::burn_from(asset_id, who, amount)
                .map_err(FungibleLedgerError::InvalidBurn)?;
        }
        Ok(())
    }
}
