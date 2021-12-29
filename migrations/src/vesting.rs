use frame_support::{
    BoundedVec,
    weights::Weight,
    traits::{Currency, OnRuntimeUpgrade, Get},
};

#[cfg(feature = "try-runtime")]
use frame_support::traits::OnRuntimeUpgradeHelpersExt;
#[cfg(feature = "try-runtime")]
use sp_std::prelude::*;

use pallet_vesting::{Vesting, VestingInfo, Config, MaxVestingSchedulesGet};

type BalanceOf<T> =
<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub struct SaleVestingFixMigration<T: Config>(sp_std::marker::PhantomData<T>);

type VestingData<T> = BoundedVec<VestingInfo<BalanceOf<T>, <T as frame_system::Config>::BlockNumber>, MaxVestingSchedulesGet<T>>;

impl<T: Config> OnRuntimeUpgrade for SaleVestingFixMigration<T> {
    fn on_runtime_upgrade() -> Weight {
        let mut reads_writes = 0;
        // Old starting block is 2 weeks = 201600
        let old_start_block = T::BlockNumber::from(201600u32);
        // New starting block is 2 weeks + 365 days = 5457600
        let new_start_block = T::BlockNumber::from(5457600u32);

        Vesting::<T>::translate_values::<VestingData<T>, _>
            (|vesting_list| {
                reads_writes += 1;

                vesting_list.try_mutate(|vests| {
                    vests.iter_mut().for_each(|sched| {
                        if sched.starting_block() == old_start_block {
                            *sched = VestingInfo::<BalanceOf<T>, T::BlockNumber>::new(
                                sched.locked(),
                                sched.locked(),
                                new_start_block,
                            )
                        }
                    })
                })
            },
        );

        T::DbWeight::get().reads_writes(reads_writes, reads_writes)
    }

    #[cfg(feature = "try-runtime")]
    fn pre_upgrade() -> Result<(), &'static str> {
        // Store snapshot of pre upgrade vesting data
        let snapshot: Vec<(T::AccountId, VestingData<T>)> = Vesting::<T>::iter().collect();
        let size = snapshot.len();
        Self::set_temp_storage(snapshot, "vesting_snapshot");
        log::debug!(
            target: "runtime::migrations::vesting",
            "Pre upgrade: saved vesting state snapshot with {} entries", size
        );
        Ok(())
    }

    #[cfg(feature = "try-runtime")]
    fn post_upgrade() -> Result<(), &'static str> {
        // Read snapshot of pre upgrade vesting data
        let snapshot = Self::get_temp_storage::<Vec<(T::AccountId, VestingData<T>)>>("vesting_snapshot").unwrap();
        let mut modified = 0;
        // Old starting block is 2 weeks = 201600
        let old_start_block = T::BlockNumber::from(201600u32);
        // New starting block is 2 weeks + 365 days = 5457600
        let new_start_block = T::BlockNumber::from(5457600u32);
        // Check state size is the same
        assert!(snapshot.len() == Vesting::<T>::iter_keys().collect::<Vec<T::AccountId>>().len(), "Size mismatch in vesting state after upgrade");
        // Check new vesting data matches snapshot with modified sale schedules
        snapshot.iter().for_each(|data| {
            let new_data = Vesting::<T>::get(&data.0).expect("Key in vesting snapshot not found after upgrade");
            data.1.iter().enumerate().for_each(|(idx, sched)| {
                let new_sched = new_data.get(idx).expect("Vesting schedules list size was modified after upgrade");
                // First check that locked value was not changed
                assert!(new_sched.locked() == sched.locked(), "Locked value of a schedule modified after upgrade");
                // Then check for correct per block and starting block for sale fixes (or not modified)
                if sched.starting_block() == old_start_block {
                    // Per block is equal to locked for fixed schedules
                    assert!(new_sched.per_block() == new_sched.locked(), "Per block of sale schedule is incorrect after upgrade");
                    // Starting block is equal to 5457600
                    assert!(new_sched.starting_block() == new_start_block, "Starting block of sale schedule is incorrect after upgrade");
                    modified += 1;
                } else {
                    assert!(new_sched.per_block() == sched.per_block(), "Per block of a schedule modified after upgrade");
                    assert!(new_sched.starting_block() == sched.starting_block(), "Starting block of a schedule modified after upgrade");
                }
            })
        });
        log::debug!(
            target: "runtime::migrations::vesting",
            "Post upgrade: checks completed, found {} fixed sale vesting schedules", modified
        );
        Ok(())
    }
}
