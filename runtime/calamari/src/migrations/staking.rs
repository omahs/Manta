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

use crate::{
    sp_api_hidden_includes_construct_runtime::hidden_include::traits::{Currency, OriginTrait},
    Balance, Get, Vec, Weight,
};
use core::marker::PhantomData;
use frame_support::traits::OnRuntimeUpgrade;
#[cfg(feature = "try-runtime")]
use frame_support::traits::OnRuntimeUpgradeHelpersExt;

/// Migration to move old invulnerables to the staking set on upgrade
/// [DelegationScheduledRequests] storage item.
/// Additionally [DelegatorState] is migrated from [OldDelegator] to [Delegator].
pub struct MigrateInvulnerables<T>(PhantomData<T>);

#[allow(deprecated)]
impl<T> OnRuntimeUpgrade for MigrateInvulnerables<T>
where
    T: frame_system::Config
        + pallet_parachain_staking::Config
        + pallet_session::Config
        + manta_collator_selection::Config,
    <<T as frame_system::Config>::Origin as OriginTrait>::AccountId:
        From<<T as frame_system::Config>::AccountId>,
    pallet_parachain_staking::BalanceOf<T>: Into<Balance> + From<Balance>,
{
    fn on_runtime_upgrade() -> Weight
    where
        <<T as frame_system::Config>::Origin as OriginTrait>::AccountId:
            From<<T as frame_system::Config>::AccountId>,
        pallet_parachain_staking::BalanceOf<T>: Into<Balance> + From<Balance>,
    {
        log::info!(
            target: "MigrateInvulnerables",
            "Migrating invulnerables from manta_collator_selection to
            pallet_parachain_staking"
        );

        // let mut reads: Weight = 0;
        // let mut writes: Weight = 0;

        // 1. Find current invulnerables
        let invulnerables = manta_collator_selection::Pallet::<T>::invulnerables();

        // 2. Clear the invulnerables list
        let _ = manta_collator_selection::Pallet::<T>::set_invulnerables(
            <T as frame_system::Config>::Origin::root(),
            Vec::new(),
        );

        // 3. Register invulnerables to whitelist
        // i.e. onboard with manta_collator_selection::registerCandidate
        for invuln in invulnerables.clone() {
            log::info!(
                "Migrating account {:?} with initial free_balance {:?}",
                invuln.clone(),
                <T as pallet_parachain_staking::Config>::Currency::free_balance(&invuln)
            );
            let _ = manta_collator_selection::Pallet::<T>::register_candidate(
                <T as frame_system::Config>::Origin::root(),
                invuln.clone(),
            );
            log::info!(
                "Migrating account {:?} with free_balance after collator_selection::candidates {:?}",
                invuln.clone(),
                <T as pallet_parachain_staking::Config>::Currency::free_balance(&invuln)
            );
        }

        // 4. Initialize parachain staking pallet to genesis-equivalent state
        //    and onboard invulnerables to staking with 400k lock

        // +1 because migrations execute before this is updated in the on_initialize hook
        let current_block = frame_system::Pallet::<T>::block_number() + 1u32.into();
        pallet_parachain_staking::Pallet::<T>::initialize_pallet(
            current_block,
            invulnerables,
            crate::currency::inflation_config::<T>(),
        );
        // Setting total_selected will take effect at the beginning of the next round, so for the first 6 hours
        // our invulnerables will be the only collators
        let _ = pallet_parachain_staking::Pallet::<T>::set_total_selected(
            <T as frame_system::Config>::Origin::root(),
            63u32,
        );

        // TODO: Get correct weight from the extrinsics
        // T::DbWeight::get().reads_writes(reads, writes)
        T::BlockWeights::get().max_block // only migration, we can just use the whole block
    }

    #[cfg(feature = "try-runtime")]
    fn pre_upgrade() -> Result<(), &'static str> {
        use crate::sp_api_hidden_includes_construct_runtime::hidden_include::traits::ReservableCurrency;

        // Ensure they each have 400k KMA reserved by collator_selection
        let invulnerables = manta_collator_selection::Pallet::<T>::invulnerables();
        for invulnerable in invulnerables.clone() {
            assert!(
                <T as pallet_parachain_staking::Config>::Currency::reserved_balance(&invulnerable)
                    >= T::MinWhitelistCandidateStk::get()
            );
        }

        Self::set_temp_storage(invulnerables, "invulnerables");

        Ok(())
    }

    #[cfg(feature = "try-runtime")]
    fn post_upgrade() -> Result<(), &'static str> {
        // Invulnerables were migrated correctly
        let invulnerables: Vec<T::AccountId> =
            Self::get_temp_storage("invulnerables").expect("must exist");
        for invuln in invulnerables {
            assert!(
                !manta_collator_selection::Pallet::<T>::candidates()
                    .iter()
                    .any(|x| x.who == invuln),
                "invulnerables must no longer be candidates with collator_selection",
            );
            let bond = pallet_parachain_staking::Bond {
                owner: invuln.clone(),
                amount: T::MinWhitelistCandidateStk::get(),
            };
            assert!(
                pallet_parachain_staking::Pallet::<T>::candidate_pool().contains(&bond),
                "invulnerables must now be candidates",
            );
            assert!(
                pallet_parachain_staking::Pallet::<T>::selected_candidates().contains(&invuln),
                "invulnerables must be active collators for the first round",
            );
        }
        // Other pallet parameters are set
        // Inflation is set to 3%
        assert_eq!(
            pallet_parachain_staking::Pallet::<T>::inflation_config()
                .annual
                .ideal,
            sp_arithmetic::Perbill::from_percent(3)
        );

        // TotalSelected is 63
        assert_eq!(
            pallet_parachain_staking::Pallet::<T>::total_selected(),
            63u32
        );

        // Round is 1
        let current_block = frame_system::Pallet::<T>::block_number() + 1u32.into();
        let round_info = pallet_parachain_staking::Pallet::<T>::round();
        assert_eq!(round_info.current, 1u32);
        assert_eq!(round_info.first, current_block);

        Ok(())
    }
}
