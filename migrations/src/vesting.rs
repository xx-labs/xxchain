use frame_support::{
    BoundedVec,
    weights::Weight,
    traits::{Currency, OnRuntimeUpgrade, Get},
};
use pallet_vesting::{Vesting, VestingInfo, Config, MaxVestingSchedulesGet};

type BalanceOf<T> =
<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub struct SaleVestingFixMigration<T: Config>(sp_std::marker::PhantomData<T>);

impl<T: Config> OnRuntimeUpgrade for SaleVestingFixMigration<T> {
    fn on_runtime_upgrade() -> Weight {
        let mut reads_writes = 0;
        // Old starting block is 2 weeks = 201600
        let old_start_block = T::BlockNumber::from(201600u32);
        // New starting block is 2 weeks + 365 days = 5457600
        let new_start_block = T::BlockNumber::from(5457600u32);

        Vesting::<T>::translate_values::<
            BoundedVec<
                VestingInfo<BalanceOf<T>, T::BlockNumber>,
                MaxVestingSchedulesGet<T>
            >
            , _>
            (|vesting_list| {
                reads_writes += 1;

                vesting_list.try_mutate(|vests| {
                    vests.iter_mut().for_each(|sched| {
                        if sched.starting_block() == old_start_block {
                            log::debug!(
                                target: "runtime::migrations::vesting",
                                "vesting migration: Modifying sale vesting schedule"
                            );
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
        Ok(())
    }

    #[cfg(feature = "try-runtime")]
    fn post_upgrade() -> Result<(), &'static str> {
        Ok(())
    }
}
