// This file is part of XX-Network.

// Added as part the code review and testing
// by ChainSafe Systems Aug 2021

//! Tests for modifications made to the staking module to support the XX network specific functionality

use super::*;
use frame_election_provider_support::Support;
use frame_support::{
    assert_noop, assert_ok,
    traits::{Currency},
    StorageMap,
};
use mock::*;

use sp_runtime::{
    traits::{BadOrigin},
};



/////////////////////////////////////////
//            Minimum Stake            //
/////////////////////////////////////////

#[test]
fn min_bond_set_from_genesis_config() {
    let val = 555;
    ExtBuilder::default().min_bond(val).build_and_execute(|| {
        assert_eq!(Staking::validator_min_bond(), val);
    });
}

#[test]
fn calling_validate_with_insufficient_bond_errors() {
    let min_bond = 555;
    let stash_value = min_bond - 1;
    ExtBuilder::default()
        .has_stakers(false)
        .min_bond(min_bond)
        .build_and_execute(|| {
            // accounts 10 has a balance of stash_value
            let _ = Balances::make_free_balance_be(&10, stash_value);

            // bond 10 as stash, 11 as controller account with stash_value balance
            assert_ok!(Staking::bond(Origin::signed(10), 11, stash_value));

            // controller (11) calls validate but stash balance is only stash_value (<=min_bond) so error
            assert_noop!(
                Staking::validate(Origin::signed(11), ValidatorPrefs::default()),
                Error::<Test>::ValidatorInsufficientBond,
            );

            assert_eq!(staking_events(), vec![RawEvent::Bonded(10, stash_value)],);
        });
}

#[test]
fn calling_validate_with_min_bond_succeeds() {
    let min_bond = 555;
    let stash_value = min_bond;
    ExtBuilder::default()
        .has_stakers(false)
        .min_bond(min_bond)
        .build_and_execute(|| {
            let _ = Balances::make_free_balance_be(&10, stash_value);

            assert_ok!(Staking::bond(Origin::signed(10), 11, stash_value));

            assert_ok!(Staking::validate(
                Origin::signed(11),
                ValidatorPrefs::default()
            ),);

            assert_eq!(staking_events(), vec![RawEvent::Bonded(10, stash_value)],);
        });
}

#[test]
fn can_unbond_below_minimum_if_not_validator() {
    let min_bond = 555;
    let stash_value = min_bond;
    let delta = 100; // how much to unbond
    ExtBuilder::default()
        .has_stakers(false)
        .min_bond(min_bond)
        .build_and_execute(|| {
            let _ = Balances::make_free_balance_be(&10, stash_value);

            // bond 10 as stash, 11 as controller account with stash_value balance
            assert_ok!(Staking::bond(Origin::signed(10), 11, stash_value));

            assert_eq!(
                Staking::ledger(&11),
                Some(StakingLedger {
                    stash: 10,
                    total: stash_value,
                    active: stash_value,
                    unlocking: vec![],
                    claimed_rewards: vec![],
                })
            );

            assert_ok!(Staking::unbond(Origin::signed(11), delta),);

            assert_eq!(
                Staking::ledger(&11),
                Some(StakingLedger {
                    stash: 10,
                    total: stash_value,
                    active: stash_value - delta,
                    unlocking: vec![UnlockChunk {
                        value: delta,
                        era: 3
                    }],
                    claimed_rewards: vec![],
                })
            );

            assert_eq!(
                staking_events(),
                vec![RawEvent::Bonded(10, 555), RawEvent::Unbonded(10, 100)],
            );
        });
}

#[test]
fn cannot_unbond_below_minimum_if_validator() {
    let min_bond = 555;
    let stash_value = min_bond;
    let delta = 100; // how much to unbond
    ExtBuilder::default()
        .has_stakers(false)
        .min_bond(min_bond)
        .build_and_execute(|| {
            let _ = Balances::make_free_balance_be(&10, stash_value);
            // bond 10 as stash, 11 as controller account with stash_value balance
            assert_ok!(Staking::bond(Origin::signed(10), 11, stash_value));
            // make 10/11 a validator
            assert_ok!(Staking::validate(
                Origin::signed(11),
                ValidatorPrefs::default()
            ),);
            assert_noop!(
                Staking::unbond(Origin::signed(11), delta),
                Error::<Test>::ValidatorInsufficientBond,
            );

            assert_eq!(staking_events(), vec![RawEvent::Bonded(10, 555)],);
        });
}

#[test]
fn only_admin_origin_can_set_minimum_bond() {
    let val = 100;
    ExtBuilder::default().build_and_execute(|| {
        // non-admin account call fails
        assert_noop!(
            Staking::set_validator_min_bond(Origin::signed(10), val),
            BadOrigin,
        );
        // admin account call succeeds
        assert_ok!(Staking::set_validator_min_bond(Origin::root(), val));
        // value was updated by call
        assert_eq!(Staking::validator_min_bond(), val);
    })
}

#[test]
fn admin_can_decrease_minimum_bond_and_validators_with_less_than_minimum_can_be_chilled() {
    let min_bond = 555;
    let stash_value = min_bond;
    let delta = 100; // how much to unbond
    let prefs = ValidatorPrefs {
        cmix_root: CmixHash::repeat_byte(0xff),
        ..Default::default()
    };
    ExtBuilder::default()
        .has_stakers(false)
        .min_bond(min_bond)
        .build_and_execute(|| {
            // This validator will be chilled after min bond increase
            let _ = Balances::make_free_balance_be(&10, stash_value);
            // bond 10 as stash, 11 as controller account with stash_value balance
            assert_ok!(Staking::bond(Origin::signed(10), 11, stash_value));
            // make 10/11 a validator
            assert_ok!(Staking::validate(
                Origin::signed(11),
                ValidatorPrefs::default()
            ),);

            // This validator will remain
            let _ = Balances::make_free_balance_be(&12, stash_value + delta);
            // bond 12 as stash, 13 as controller account with stash_value balance
            assert_ok!(Staking::bond(Origin::signed(12), 13, stash_value + delta));
            // make 12/13 a validator
            assert_ok!(Staking::validate(
                Origin::signed(13),
                prefs.clone()
            ),);

            // now increase the minimum above what 10/11 has staked
            assert_ok!(Staking::set_validator_min_bond(
                Origin::root(),
                min_bond + delta
            ));

            // both validators are still present despite change to minimum bond
            assert_eq!(Validators::<Test>::get(10), ValidatorPrefs::default());
            assert_eq!(Validators::<Test>::get(12), prefs);

            // validator 10/11 can be removed by any caller, using chill_other
            assert_ok!(Staking::chill_other(
                Origin::signed(1337),
                11
            ));

            // confirm validator 10/11 was removed
            assert!(Validators::<Test>::contains_key(10) == false);

            // validator 12/13 cannot be removed
            assert_noop!(
                Staking::chill_other(
                    Origin::signed(1337),
                    13
                ),
                Error::<Test>::CannotChillOther,
            );
        });

    // Is this actually the desired behaviour? An alternative would be
    // to drop/chill validators who do not meet the new minimum bond at the start of the next era.
    // This may be risky though as setting a high threshold
    // may reduce the number of validators enough for the network to be attacked.
}

/////////////////////////////////////
//            CMIX Root            //
/////////////////////////////////////

#[test]
fn calling_validate_with_existing_cmix_root_fails() {
    let stash_value = 100;
    ExtBuilder::default()
        .has_stakers(false)
        .build_and_execute(|| {
            let _ = Balances::make_free_balance_be(&10, stash_value);
            let _ = Balances::make_free_balance_be(&20, stash_value);

            assert_ok!(Staking::bond(Origin::signed(10), 11, stash_value));
            assert_ok!(Staking::bond(Origin::signed(20), 21, stash_value));

            // ok to add the first time
            assert_ok!(Staking::validate(
                Origin::signed(11),
                ValidatorPrefs {
                    cmix_root: CmixHash::repeat_byte(0xff),
                    ..Default::default()
                }
            ),);

            // if different account tries to validate, fails with ValidatorCmixIdNotUnique
            assert_noop!(
                Staking::validate(
                    Origin::signed(21),
                    ValidatorPrefs {
                        cmix_root: CmixHash::repeat_byte(0xff),
                        ..Default::default()
                    }
                ),
                Error::<Test>::ValidatorCmixIdNotUnique,
            );
        })
}

#[test]
fn calling_validate_with_existing_cmix_root_works_for_existing_validator() {
    let stash_value = 100;
    let new_commission = Perbill::from_rational(2u32,100u32);
    ExtBuilder::default()
        .has_stakers(false)
        .build_and_execute(|| {
            let _ = Balances::make_free_balance_be(&10, stash_value);
            let _ = Balances::make_free_balance_be(&20, stash_value);

            assert_ok!(Staking::bond(Origin::signed(10), 11, stash_value));
            assert_ok!(Staking::bond(Origin::signed(20), 21, stash_value));

            // ok to add the first time
            assert_ok!(Staking::validate(
                Origin::signed(11),
                ValidatorPrefs {
                    cmix_root: CmixHash::repeat_byte(0xff),
                    ..Default::default()
                }
            ),);

            // check validator commission is 0%
            assert_eq!(Validators::<Test>::get(10).commission, Default::default());

            // if different account tries to validate, fails with ValidatorCmixIdNotUnique
            assert_noop!(
                Staking::validate(
                    Origin::signed(21),
                    ValidatorPrefs {
                        cmix_root: CmixHash::repeat_byte(0xff),
                        ..Default::default()
                    }
                ),
                Error::<Test>::ValidatorCmixIdNotUnique,
            );

            // ok if original account tries to validate with same cmix ID,
            // for example, changing commission
            assert_ok!(Staking::validate(
                Origin::signed(11),
                ValidatorPrefs {
                    commission: new_commission.clone(),
                    cmix_root: CmixHash::repeat_byte(0xff),
                    ..Default::default()
                }
            ),);

            // check validator commission is now 2% instead of 0%
            assert_eq!(Validators::<Test>::get(10).commission, new_commission);

            // add second validator with different cmix ID
            assert_ok!(Staking::validate(
                Origin::signed(21),
                ValidatorPrefs {
                    cmix_root: CmixHash::repeat_byte(0xde),
                    ..Default::default()
                }
            ),);

            // check validator cmix ID is 0xde.....de
            assert_eq!(Validators::<Test>::get(20).cmix_root, CmixHash::repeat_byte(0xde));

            // if existing validator tries to change cmix ID to another existing one
            // fails with ValidatorCmixIdNotUnique
            assert_noop!(
                Staking::validate(
                    Origin::signed(11),
                    ValidatorPrefs {
                        cmix_root: CmixHash::repeat_byte(0xde),
                        ..Default::default()
                    }
                ),
                Error::<Test>::ValidatorCmixIdNotUnique,
            );

            // ok if existing validator tries to change cmix ID to a different non existing one
            assert_ok!(Staking::validate(
                Origin::signed(21),
                ValidatorPrefs {
                    cmix_root: CmixHash::repeat_byte(0xab),
                    ..Default::default()
                }
            ),);

            // check validator cmix ID has changed to 0xab.....ab
            assert_eq!(Validators::<Test>::get(20).cmix_root, CmixHash::repeat_byte(0xab));
        })
}

////////////////////////////////////////
//         Rewards Destination        //
////////////////////////////////////////

#[test]
fn rewards_paid_to_stash() {
    ExtBuilder::default()
        .nominate(true)
        .session_per_era(3)
        .build_and_execute(|| {
            // Un the default ExtBuilder initial conditions we have
            // A, B and C (unnominated) as validators with N as a nominator, nominating A and B
            let a = Staker {
                stash: 11,
                ctrl: 10,
            };
            let b = Staker {
                stash: 21,
                ctrl: 20,
            };
            let n = Staker {
                stash: 101,
                ctrl: 100,
            };

            let init_a_stash = Balances::total_balance(&a.stash);
            let init_a_stake = 1000;
            let init_a_ctrl = Balances::total_balance(&a.ctrl);

            let init_b_stash = Balances::total_balance(&b.stash);
            let init_b_stake = 1000;
            let init_b_ctrl = Balances::total_balance(&b.ctrl);

            let init_n_stash = Balances::total_balance(&n.stash);
            let init_n_stake = 500;
            let init_n_ctrl = Balances::total_balance(&n.ctrl);

            // set some reward points for validators
            // they all get a bonus 1 for being initialized
            <Module<Test>>::reward_by_ids(vec![(a.stash, 80)]);
            <Module<Test>>::reward_by_ids(vec![(b.stash, 20)]);

            let a_points = 80 + 1;
            let b_points = 20 + 1;

            assert_eq!(
                Staking::eras_reward_points(Staking::active_era().unwrap().index),
                EraRewardPoints {
                    total: a_points + b_points,
                    individual: vec![(a.stash, a_points), (b.stash, b_points)]
                        .into_iter()
                        .collect(),
                }
            );

            let total_payout_0 = current_total_payout_for_duration(reward_time_per_era());

            // advance to the next era and compute rewards for previous
            // there are 3 sessions per era in this test config so we are now in era 2
            start_session(3);

            // compute how much of the total era reward is allocated to each validator
            let a_part = Perbill::from_rational(a_points, a_points + b_points);
            let b_part = Perbill::from_rational(b_points, a_points + b_points);

            // Calculate how each validators share of total reward is allocated
            // between themselves and their nominators
            // Exposures for validators are their initial stash balances
            // Exposures for the nominator is their initial stash split between each nomination
            // (the vote allocation from nominator in this case is: 1/4 to A, 3/4 to B)

            let n_vote_to_a = init_n_stake / 4;
            let n_vote_to_b = init_n_stake * 3 / 4;

            let a_exp_part =
                Perbill::from_rational::<u32>(init_a_stake, init_a_stake + n_vote_to_a);
            let b_exp_part =
                Perbill::from_rational::<u32>(init_b_stake, init_b_stake + n_vote_to_b);
            let n_from_a_part =
                Perbill::from_rational::<u32>(n_vote_to_a, init_a_stake + n_vote_to_a);
            let n_from_b_part =
                Perbill::from_rational::<u32>(n_vote_to_b, init_b_stake + n_vote_to_b);
            // now we have everything we need to compute rewards
            let a_rewards = a_part * a_exp_part * total_payout_0;
            let b_rewards = b_part * b_exp_part * total_payout_0;
            let n_rewards =
                a_part * n_from_a_part * total_payout_0 + b_part * n_from_b_part * total_payout_0;

            make_all_reward_payment(0);

            // Rewards paid to stash accounts
            assert_eq!(Balances::total_balance(&a.stash), init_a_stash + a_rewards,);
            assert_eq!(Balances::total_balance(&b.stash), init_b_stash + b_rewards,);
            assert_eq!(Balances::total_balance(&n.stash), init_n_stash + n_rewards,);

            // Controller accounts remain the same
            assert_eq!(Balances::total_balance(&a.ctrl), init_a_ctrl,);
            assert_eq!(Balances::total_balance(&b.ctrl), init_b_ctrl,);
            assert_eq!(Balances::total_balance(&n.ctrl), init_n_ctrl,);

            //////////////////// second era /////////////////////////

            // We expect that identical rewards will be paid out again
            // if the same reward points are used

            <Module<Test>>::reward_by_ids(vec![(a.stash, 80)]);
            <Module<Test>>::reward_by_ids(vec![(b.stash, 20)]);

            start_session(6);

            make_all_reward_payment(1);

            assert_eq!(
                Balances::total_balance(&a.stash),
                init_a_stash + a_rewards + a_rewards,
            );
            assert_eq!(
                Balances::total_balance(&b.stash),
                init_b_stash + b_rewards + b_rewards,
            );
            assert_eq!(
                Balances::total_balance(&n.stash),
                init_n_stash + n_rewards + n_rewards,
            );
        })
}

////////////////////////////////////////
//           Custody Accounts         //
////////////////////////////////////////

#[test]
fn can_set_custody_accounts_in_builder() {
    let custody_account_id = 10;
    ExtBuilder::default()
        .custody_accounts(&[custody_account_id])
        .build_and_execute(|| {
            assert!(<Test as Config>::CustodianHandler::is_custody_account(
                &custody_account_id
            ));
            assert!(
                <Test as Config>::CustodianHandler::is_custody_account(&(custody_account_id + 1))
                    == false
            );
        })
}

#[test]
fn exposure_not_counted_for_custody_accounts() {
    let a = 10;
    let b = 20;

    let supports = vec![
        (
            a,
            Support {
                total: 100,
                voters: vec![(a, 100), (b, 20)],
            },
        ), // custody account self-assigns 100, b votes 20
        (
            b,
            Support {
                total: 150,
                voters: vec![(a, 100), (b, 50)],
            },
        ), // custody account votes 100 for B, B 50 for itself
    ];

    ExtBuilder::default()
        .has_stakers(false)
        .custody_accounts(&[a]) // A is a custody account
        .build_and_execute(|| {
            assert_eq!(
                Staking::collect_exposures(supports),
                vec![
                    (
                        a, // A has no exposure from itself, only contributions from B
                        Exposure {
                            total: 20,
                            own: 0,
                            others: vec![IndividualExposure { who: b, value: 20 }]
                        }
                    ),
                    (
                        b, // B has only exposure from itself, votes from A not counted
                        Exposure {
                            total: 50,
                            own: 50,
                            others: vec![]
                        }
                    )
                ]
            );
        })
}

#[test]
fn custody_accounts_cannot_be_slashed() {
    let a = 11;
    let b = 21;
    let c = 101;

    ExtBuilder::default()
        .has_stakers(true) // sets a, b to be stakers
        .nominate(true) // c nominates a and b
        .custody_accounts(&[c]) // c is a custody account
        .build_and_execute(|| {

            assert_eq!(Balances::free_balance(a), 1000);
            assert_eq!(Balances::free_balance(b), 2000);
            assert_eq!(Balances::free_balance(c), 2000);

            let initial_balance = Balances::free_balance(c);

            on_offence_now(
                &[
                    OffenceDetails {
                        offender: (
                            a,
                            Staking::eras_stakers(Staking::active_era().unwrap().index, a),
                        ),
                        reporters: vec![],
                    },
                    OffenceDetails {
                        offender: (
                            b,
                            Staking::eras_stakers(Staking::active_era().unwrap().index, b),
                        ),
                        reporters: vec![],
                    },
                ],
                &[Perbill::from_percent(50), Perbill::from_percent(20)],
            );

            assert_eq!(
                Balances::free_balance(&c),
                initial_balance,
            );

        })
}

#[test]
fn non_custody_accounts_can_be_slashed() {
    let a = 11;
    let b = 21;
    let c = 101;

    ExtBuilder::default()
        .has_stakers(true) // sets a, b to be stakers
        .nominate(true) // c nominates a and b
        // no custody accounts
        .build_and_execute(|| {

            assert_eq!(Balances::free_balance(a), 1000);
            assert_eq!(Balances::free_balance(b), 2000);
            assert_eq!(Balances::free_balance(c), 2000);

            on_offence_now(
                &[
                    OffenceDetails {
                        offender: (
                            a,
                            Staking::eras_stakers(Staking::active_era().unwrap().index, a),
                        ),
                        reporters: vec![],
                    },
                    OffenceDetails {
                        offender: (
                            b,
                            Staking::eras_stakers(Staking::active_era().unwrap().index, b),
                        ),
                        reporters: vec![],
                    },
                ],
                &[Perbill::from_percent(50), Perbill::from_percent(20)],
            );

            assert_eq!(
                Balances::free_balance(&c),
                1863, // new slashed balance
            );

        })
}

//////////////////////////////////////////
//         Rewards Points System        //
//////////////////////////////////////////

#[test]
fn can_set_block_points_in_builder() {
    let points = 7;
    ExtBuilder::default()
        .block_points(points)
        .build_and_execute(|| {
            assert_eq!(
                <Test as Config>::CmixHandler::get_block_points(),
                points
            );
        })
}

#[test]
fn validators_are_always_initialized_with_one_point() {
    let a = 10;
    ExtBuilder::default().build_and_execute(|| {
        // assigned 0 points a few times
        Staking::reward_by_ids(vec![(a, 0), (a, 0)]);
        // results in 1 point from the first initialization
        assert_eq!(
            Staking::eras_reward_points(Staking::active_era().unwrap().index),
            EraRewardPoints {
                total: 1,
                individual: vec![(a, 1)].into_iter().collect()
            }
        )
    })
}

#[test]
fn can_deduct_points() {
    let a = 10;
    let b = 20;
    ExtBuilder::default().build_and_execute(|| {
        // add points first
        Staking::reward_by_ids(vec![(a, 5), (b, 7)]);
        assert_eq!(
            Staking::eras_reward_points(Staking::active_era().unwrap().index),
            EraRewardPoints {
                total: 14,
                individual: vec![(a, 5 + 1), (b, 7 + 1)].into_iter().collect()
            }
        );

        // deduct some from each
        Staking::deduct_by_ids(vec![(a, 2), (b, 3)]);
        assert_eq!(
            Staking::eras_reward_points(Staking::active_era().unwrap().index),
            EraRewardPoints {
                total: 9,
                individual: vec![(a, 3 + 1), (b, 4 + 1)].into_iter().collect()
            }
        );
    })
}

#[test]
fn cannot_deduct_below_one() {
    let a = 10;
    ExtBuilder::default().build_and_execute(|| {
        // add points first
        Staking::reward_by_ids(vec![(a, 5)]);
        assert_eq!(
            Staking::eras_reward_points(Staking::active_era().unwrap().index),
            EraRewardPoints {
                total: 6,
                individual: vec![(a, 6)].into_iter().collect()
            }
        );

        // deduct more points than a has
        Staking::deduct_by_ids(vec![(a, 10)]);
        assert_eq!(
            Staking::eras_reward_points(Staking::active_era().unwrap().index),
            EraRewardPoints {
                // it keeps one
                total: 1,
                individual: vec![(a, 1)].into_iter().collect()
            }
        );
    })
}

//////////////////////////////////////////
//             Rewards Pool             //
//////////////////////////////////////////

#[test]
fn reward_handler_called_on_do_payout_stakers() {
   ExtBuilder::default()
        .nominate(true)
        .session_per_era(3)
        .build_and_execute(|| {
            let a = Staker {
                stash: 11,
                ctrl: 10,
            };
            let init_a_stash = Balances::total_balance(&a.stash);

            // give a some points
            <Module<Test>>::reward_by_ids(vec![(a.stash, 80)]);

            let total_payout_0 = current_total_payout_for_duration(reward_time_per_era());

            // there are 3 sessions per era in this test config so we are now in era 2
            start_session(3);
            make_all_reward_payment(0);

            assert!(Balances::total_balance(&a.stash) > init_a_stash);
            assert_eq!(mock::RewardMock::total(), total_payout_0);
        })
}
