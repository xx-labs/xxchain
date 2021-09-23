// Added as part the code review and testing
// by ChainSafe Systems Aug 2021

use super::*;
use crate::Module as XXCustody;

use frame_benchmarking::{benchmarks, account, impl_benchmark_test_suite};
use frame_system::RawOrigin;
use sp_runtime::traits::Bounded;

const SEED: u32 = 0;

fn team_member<T: Config>() -> T::AccountId { <TeamAccounts<T>>::iter().next().expect("No team members set in genesis config").0 }

fn custodian<T: Config>() -> T::AccountId { <Custodians<T>>::iter().next().expect("No custodians set in genesis config").0 }

fn account_from_index<T: Config>(index: u32) -> T::AccountId {
	account("x", index, SEED)
}

benchmarks!{

	payout {
		// worst case: 
		// - custody period is over
		// - balance is bonded
		// - account has a governence proxy
		// This will result in a call to force_unstake, remove_proxy and do_payout
		let team = team_member::<T>();
		let custodian = custodian::<T>();
		let proxy = account_from_index::<T>(11);

		let info = XXCustody::<T>::team_accounts(team.clone());
		let amount = <<T as pallet_staking::Config>::Currency as Currency<T::AccountId>>::minimum_balance() * 10u32.into();

		// set up a bond
		XXCustody::<T>::custody_bond(
        RawOrigin::Signed(custodian.clone()).into(),
        info.custody.clone(),
        custodian.clone(),
        <<T as pallet_staking::Config>::Currency as Currency<T::AccountId>>::minimum_balance() * 10u32.into()
    ).expect("Failed to bond allocation");

		// set up a proxy
		XXCustody::<T>::custody_set_proxy(
		    RawOrigin::Signed(custodian).into(),
		    info.custody,
		    proxy,
		).expect("Failed to set proxy");

		// run to the end of the custody period
		frame_system::Pallet::<T>::set_block_number(T::CustodyDuration::get());

	}: _(RawOrigin::Signed(team.clone()), team.clone())


	custody_bond {
		let team = team_member::<T>();
		let custodian = custodian::<T>();
		let amount = <<T as pallet_staking::Config>::Currency as Currency<T::AccountId>>::minimum_balance() * 10u32.into();
		let info = XXCustody::<T>::team_accounts(team.clone());

	}: _(RawOrigin::Signed(custodian.clone()), info.custody, custodian.clone(), amount)


	custody_bond_extra {
		let team = team_member::<T>();
		let custodian = custodian::<T>();

		let info = XXCustody::<T>::team_accounts(team.clone());
		let amount = <<T as pallet_staking::Config>::Currency as Currency<T::AccountId>>::minimum_balance() * 10u32.into();

		// set up an initial bond
		XXCustody::<T>::custody_bond(
        RawOrigin::Signed(custodian.clone()).into(),
        info.custody.clone(),
        custodian.clone(),
        amount
    ).expect("Failed to bond allocation");
	}: _(RawOrigin::Signed(custodian.clone()), info.custody, amount)


	custody_set_controller {
		let team = team_member::<T>();
		let custodian = custodian::<T>();
		let new_controller = account_from_index::<T>(55);

		let info = XXCustody::<T>::team_accounts(team.clone());
		let amount = <<T as pallet_staking::Config>::Currency as Currency<T::AccountId>>::minimum_balance() * 10u32.into();

		// set up an initial bond with the custodian as the controller
		XXCustody::<T>::custody_bond(
        RawOrigin::Signed(custodian.clone()).into(),
        info.custody.clone(),
        custodian.clone(),
        amount
    ).expect("Failed to bond allocation");
	}: _(RawOrigin::Signed(custodian.clone()), info.custody, new_controller)


	custody_set_proxy {
		let team = team_member::<T>();
		let custodian = custodian::<T>();
		let proxy = account_from_index::<T>(11);

		let info = XXCustody::<T>::team_accounts(team.clone());

	}: _(RawOrigin::Signed(custodian.clone()), info.custody, proxy)
 	
 	team_custody_set_proxy {
		let team = team_member::<T>();
		let proxy = account_from_index::<T>(11);

		let info = XXCustody::<T>::team_accounts(team.clone());

		// allocate some balance to pay the lockup fee (this may not be required later)
		let balance = <<T as pallet_staking::Config>::Currency as Currency<T::AccountId>>::Balance::max_value();
		<<T as pallet_staking::Config>::Currency as Currency<T::AccountId>>::make_free_balance_be(&team, balance);

		// run to the end of the custody period
		frame_system::Pallet::<T>::set_block_number(T::CustodyDuration::get());

	}: _(RawOrigin::Signed(team.clone()), proxy)

  //----------------    ADMIN     ----------------//

  // these are all simple add/remove operations

 	add_custodian {

 	}: _(RawOrigin::Root, account_from_index::<T>(5))

 	remove_custodian {

 	}: _(RawOrigin::Root, custodian::<T>())

 	replace_team_member {

 	}: _(RawOrigin::Root, team_member::<T>(), account_from_index::<T>(6))
}




impl_benchmark_test_suite!(
  XXCustody,
  crate::mock::ExtBuilder::default()
  	.with_custodians(&[11])
  	.with_team_allocations(&[(22, 1000)])
  	.build(),
  crate::mock::Test,
);
