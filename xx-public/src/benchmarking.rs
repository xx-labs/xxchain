use super::*;
use crate::Module as XXPublic;

use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, account};
use frame_system::RawOrigin;
use frame_support::traits::OriginTrait;

const SEED: u32 = 0;

fn account_from_index<T: Config>(index: u32) -> T::AccountId {
    account("x", index, SEED)
}

fn set_testnet_manager<T: Config>(account: T::AccountId) {
    XXPublic::<T>::set_testnet_manager_account(T::Origin::root(), account).ok();
}

fn set_sale_manager<T: Config>(account: T::AccountId) {
    XXPublic::<T>::set_sale_manager_account(T::Origin::root(), account).ok();
}

const MAX_DISTRIBUTIONS: u32 = 100;
const EXPECTED_SCHEDULES: u32 = 4;

benchmarks!{
    set_testnet_manager_account {

    }: _(RawOrigin::Root, account_from_index::<T>(42))

    set_sale_manager_account {

    }: _(RawOrigin::Root, account_from_index::<T>(43))

    // Real case all distributions might have up to 4 vesting schedules
    testnet_distribute {
        let n in 1 .. MAX_DISTRIBUTIONS;
        let amount = <CurrencyOf<T> as Currency<<T as frame_system::Config>::AccountId>>::minimum_balance() * 25u32.into();
        let vest = <CurrencyOf<T> as Currency<<T as frame_system::Config>::AccountId>>::minimum_balance() * 1u32.into();
        let block = T::BlockNumber::zero();
        let mut scheds = Vec::<(BalanceOf<T>, BalanceOf<T>, T::BlockNumber)>::new();
        for i in 1 .. EXPECTED_SCHEDULES {
            scheds.push((vest.clone(), vest.clone(), block.clone()))
        }

        let manager =  account_from_index::<T>(42);
        set_testnet_manager::<T>(manager.clone());

        let mut distribution = Vec::<TransferData<T::AccountId, BalanceOf<T>, T::BlockNumber>>::new();
        for i in 0 .. n {
            let data = TransferData::<T::AccountId, BalanceOf<T>, T::BlockNumber> {
                destination: account_from_index::<T>(i),
                amount: amount.clone(),
                schedules: Some(scheds.clone()),
            };
            distribution.push(data)
        }

    }: _(RawOrigin::Signed(manager), distribution)

    sale_distribute {
        let n in 1 .. MAX_DISTRIBUTIONS;
        let amount = <CurrencyOf<T> as Currency<<T as frame_system::Config>::AccountId>>::minimum_balance() * 25u32.into();
        let vest = <CurrencyOf<T> as Currency<<T as frame_system::Config>::AccountId>>::minimum_balance() * 1u32.into();
        let block = T::BlockNumber::zero();
        let mut scheds = Vec::<(BalanceOf<T>, BalanceOf<T>, T::BlockNumber)>::new();
        for i in 1 .. EXPECTED_SCHEDULES {
            scheds.push((vest.clone(), vest.clone(), block.clone()))
        }

        let manager =  account_from_index::<T>(42);
        set_sale_manager::<T>(manager.clone());

        let mut distribution = Vec::<TransferData<T::AccountId, BalanceOf<T>, T::BlockNumber>>::new();
        for i in 0 .. n {
            let data = TransferData::<T::AccountId, BalanceOf<T>, T::BlockNumber> {
                destination: account_from_index::<T>(i),
                amount: amount.clone(),
                schedules: Some(scheds.clone()),
            };
            distribution.push(data)
        }

    }: _(RawOrigin::Signed(manager), distribution)
}

impl_benchmark_test_suite!(
  XXPublic,
  crate::mock::ExtBuilder::default()
  	.build(),
  crate::mock::Test,
);
