use super::*;

use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
use frame_system::RawOrigin;

fn account<T: Config>() -> T::AccountId { <Accounts<T>>::iter().next().expect("No accounts set in genesis config").0 }

benchmarks!{
    select_option {
        let account = account::<T>();
    }: _(RawOrigin::Signed(account), RewardOption::Vesting9Month)

    approve {

    }: _(RawOrigin::Root)
}

impl_benchmark_test_suite!(
  XXBetanetRewards,
  crate::mock::ExtBuilder::default()
  	.build(),
  crate::mock::Test,
);
