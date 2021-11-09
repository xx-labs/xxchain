use super::*;
use mock::*;

use frame_support::{assert_noop, assert_ok};
use sp_runtime::traits::BadOrigin;

#[test]
fn approval_origin_must_be_root() {
    ExtBuilder::default()
        .build_and_execute(|| {
            // Signed origin fails
            assert_noop!(
				XXBetanetRewards::approve(Origin::signed(1)),
				BadOrigin
			);

            // Root origin works
            assert_ok!(
				XXBetanetRewards::approve(Origin::root())
			);

            // Check events
            assert_eq!(
                xx_betanet_rewards_events(),
                vec![RawEvent::ProgramApproved]
            );
        })
}

#[test]
fn set_option_origin_must_be_in_accounts() {
    ExtBuilder::default()
        .build_and_execute(|| {
            // Account 10 can select option
            assert_ok!(
				XXBetanetRewards::select_option(Origin::signed(10), RewardOption::NoVesting)
			);

            // Account 1 can't
            assert_noop!(
				XXBetanetRewards::select_option(Origin::signed(1), RewardOption::NoVesting),
				Error::<Test>::NoRewards
			);

            // Check events
            assert_eq!(
                xx_betanet_rewards_events(),
                vec![RawEvent::OptionSelected(10, RewardOption::NoVesting)]
            );
        })
}

#[test]
fn nothing_happens_if_program_not_accepted() {
    ExtBuilder::default()
        .build_and_execute(|| {
            // Account 10 can select option
            assert_ok!(
				XXBetanetRewards::select_option(Origin::signed(10), RewardOption::Vesting9Month)
			);

            // Check events
            assert_eq!(
                xx_betanet_rewards_events(),
                vec![RawEvent::OptionSelected(10, RewardOption::Vesting9Month)]
            );

            // Keep original issuance and claim total
            let original_issuance = Balances::total_issuance();
            let original_claim_total = <claims::Total<Test>>::get();

            // Go to enactment block
            run_to_block(BetanetStakingRewardsBlock::get());

            // Confirm total claims remains the same
            assert_eq!(
                <claims::Total<Test>>::get(),
                original_claim_total,
            );

            // Confirm rewards pool left untouched
            assert_eq!(
                mock::RewardMock::total(),
                Zero::zero(),
            );

            // Confirm total issuance remains the same
            assert_eq!(
                Balances::total_issuance(),
                original_issuance,
            );
        })
}

#[test]
fn all_functions_are_noop_after_enactment_block() {
    ExtBuilder::default()
        .with_claims()
        .build_and_execute(|| {
            // Go to enactment block
            run_to_block(BetanetStakingRewardsBlock::get());

            // Confirm accounts can't select options anymore
            assert_noop!(
				XXBetanetRewards::select_option(Origin::signed(10), RewardOption::NoVesting),
				Error::<Test>::EnactmentBlockHasPassed
			);

            // Confirm root can't approve program anymore
            assert_noop!(
				XXBetanetRewards::approve(Origin::root()),
				Error::<Test>::EnactmentBlockHasPassed
			);

            // Issue a claim that had potential rewards, and confirm account is not added to Accounts map
            assert_ok!(Claims::claim(Origin::none(), 70, sig::<Test>(&alice(), &70u64.encode(), &[][..])));
            assert_eq!(<Accounts<Test>>::contains_key(&70), false);
        })
}

#[test]
fn rewards_are_correct_for_genesis_accounts_no_vesting() {
    ExtBuilder::default()
        .build_and_execute(|| {
            // Account 10 selects option 1: No Vesting
            assert_ok!(
				XXBetanetRewards::select_option(Origin::signed(10), RewardOption::NoVesting)
			);

            // Account 20 selects option 2: 1 month vest
            assert_ok!(
				XXBetanetRewards::select_option(Origin::signed(20), RewardOption::Vesting1Month)
			);

            // Account 30 selects option 3: 3 month vest
            assert_ok!(
				XXBetanetRewards::select_option(Origin::signed(30), RewardOption::Vesting3Month)
			);

            // Account 40 selects option 4: 6 month vest
            assert_ok!(
				XXBetanetRewards::select_option(Origin::signed(40), RewardOption::Vesting6Month)
			);

            // Account 50 selects option 5: 9 month vest
            assert_ok!(
				XXBetanetRewards::select_option(Origin::signed(50), RewardOption::Vesting9Month)
			);

            // Account 60 doesn't select option, defaults to 4: 6 month vest

            // Approve program
            assert_ok!(
				XXBetanetRewards::approve(Origin::root())
			);

            // Keep original issuance
            let original_issuance = Balances::total_issuance();

            // Go to enactment block
            run_to_block(BetanetStakingRewardsBlock::get());

            // Check events
            assert_eq!(
                xx_betanet_rewards_events(),
                vec![
                    RawEvent::OptionSelected(10, RewardOption::NoVesting),
                    RawEvent::OptionSelected(20, RewardOption::Vesting1Month),
                    RawEvent::OptionSelected(30, RewardOption::Vesting3Month),
                    RawEvent::OptionSelected(40, RewardOption::Vesting6Month),
                    RawEvent::OptionSelected(50, RewardOption::Vesting9Month),
                    RawEvent::ProgramApproved,
                    RawEvent::ProgramEnacted
                ]
            );

            // Confirm rewards paid out correctly and vesting schedules added
            let mut rewards_paid = 0u128;
            rewards_paid += confirm_reward_result(&10, RewardOption::NoVesting, false);
            rewards_paid += confirm_reward_result(&20, RewardOption::Vesting1Month, false);
            rewards_paid += confirm_reward_result(&30, RewardOption::Vesting3Month, false);
            rewards_paid += confirm_reward_result(&40, RewardOption::Vesting6Month, false);
            rewards_paid += confirm_reward_result(&50, RewardOption::Vesting9Month, false);
            rewards_paid += confirm_reward_result(&60, RewardOption::Vesting6Month, false);

            // Confirm total rewards were paid from reward pool
            assert_eq!(
                mock::RewardMock::total(),
                rewards_paid,
            );

            // Confirm total issuance remained the same
            assert_eq!(
                Balances::total_issuance(),
                original_issuance,
            );
        })
}

#[test]
fn rewards_are_correct_for_genesis_accounts_with_vesting() {
    ExtBuilder::default()
        .with_vesting()
        .build_and_execute(|| {
            // Account 10 selects option 1: No Vesting
            assert_ok!(
				XXBetanetRewards::select_option(Origin::signed(10), RewardOption::NoVesting)
			);

            // Account 20 selects option 2: 1 month vest
            assert_ok!(
				XXBetanetRewards::select_option(Origin::signed(20), RewardOption::Vesting1Month)
			);

            // Account 30 selects option 3: 3 month vest
            assert_ok!(
				XXBetanetRewards::select_option(Origin::signed(30), RewardOption::Vesting3Month)
			);

            // Account 40 selects option 4: 6 month vest
            assert_ok!(
				XXBetanetRewards::select_option(Origin::signed(40), RewardOption::Vesting6Month)
			);

            // Account 50 selects option 5: 9 month vest
            assert_ok!(
				XXBetanetRewards::select_option(Origin::signed(50), RewardOption::Vesting9Month)
			);

            // Account 60 doesn't select option, defaults to 4: 6 month vest

            // Approve program
            assert_ok!(
				XXBetanetRewards::approve(Origin::root())
			);

            // Keep original issuance
            let original_issuance = Balances::total_issuance();

            // Go to enactment block
            run_to_block(BetanetStakingRewardsBlock::get());

            // Check events
            assert_eq!(
                xx_betanet_rewards_events(),
                vec![
                    RawEvent::OptionSelected(10, RewardOption::NoVesting),
                    RawEvent::OptionSelected(20, RewardOption::Vesting1Month),
                    RawEvent::OptionSelected(30, RewardOption::Vesting3Month),
                    RawEvent::OptionSelected(40, RewardOption::Vesting6Month),
                    RawEvent::OptionSelected(50, RewardOption::Vesting9Month),
                    RawEvent::ProgramApproved,
                    RawEvent::ProgramEnacted
                ]
            );

            // Confirm rewards paid out correctly and vesting schedules added
            let mut rewards_paid = 0u128;
            rewards_paid += confirm_reward_result(&10, RewardOption::NoVesting, false);
            rewards_paid += confirm_reward_result(&20, RewardOption::Vesting1Month, false);
            rewards_paid += confirm_reward_result(&30, RewardOption::Vesting3Month, false);
            rewards_paid += confirm_reward_result(&40, RewardOption::Vesting6Month, false);
            rewards_paid += confirm_reward_result(&50, RewardOption::Vesting9Month, false);
            rewards_paid += confirm_reward_result(&60, RewardOption::Vesting6Month, false);

            // Confirm total rewards were paid from reward pool
            assert_eq!(
                mock::RewardMock::total(),
                rewards_paid,
            );

            // Confirm total issuance remained the same
            assert_eq!(
                Balances::total_issuance(),
                original_issuance,
            );
        })
}

#[test]
fn rewards_are_correct_for_claims() {
    ExtBuilder::default()
        .with_claims()
        .build_and_execute(|| {
            // Alice claims coins into account 70
            assert_ok!(Claims::claim(Origin::none(), 70, sig::<Test>(&alice(), &70u64.encode(), &[][..])));

            // Confirm rewards added to betanet rewards pallet
            confirm_claim_rewards_added(&70);

            // Bob claims coins into account 80
            assert_ok!(Claims::claim(Origin::none(), 80, sig::<Test>(&bob(), &80u64.encode(), &[][..])));

            // Confirm rewards added to betanet rewards pallet
            confirm_claim_rewards_added(&80);

            // Eve claims coins into account 90
            assert_ok!(Claims::claim(Origin::none(), 90, sig::<Test>(&eve(), &90u64.encode(), &[][..])));

            // Confirm no rewards added to betanet rewards pallet
            assert_eq!(<Accounts<Test>>::contains_key(&90), false);

            // Confirm Alice and Bob can choose options
            assert_ok!(
				XXBetanetRewards::select_option(Origin::signed(70), RewardOption::Vesting1Month)
			);

            assert_ok!(
				XXBetanetRewards::select_option(Origin::signed(80), RewardOption::Vesting3Month)
			);

            // Accounts 10, 20, 30, 40, 50 and 60 don't select option, defaults to 4: 6 month vest

            // Approve program
            assert_ok!(
				XXBetanetRewards::approve(Origin::root())
			);

            // Keep original issuance and claim total
            let original_issuance = Balances::total_issuance();
            let original_claim_total = <claims::Total<Test>>::get();

            // Go to enactment block
            run_to_block(BetanetStakingRewardsBlock::get());

            // Check events
            assert_eq!(
                xx_betanet_rewards_events(),
                vec![
                    RawEvent::OptionSelected(70, RewardOption::Vesting1Month),
                    RawEvent::OptionSelected(80, RewardOption::Vesting3Month),
                    RawEvent::ProgramApproved,
                    RawEvent::ProgramEnacted
                ]
            );

            // Confirm rewards paid out correctly and vesting schedules added
            let mut rewards_paid = 0u128;
            rewards_paid += confirm_reward_result(&10, RewardOption::Vesting6Month, false);
            rewards_paid += confirm_reward_result(&20, RewardOption::Vesting6Month, false);
            rewards_paid += confirm_reward_result(&30, RewardOption::Vesting6Month, false);
            rewards_paid += confirm_reward_result(&40, RewardOption::Vesting6Month, false);
            rewards_paid += confirm_reward_result(&50, RewardOption::Vesting6Month, false);
            rewards_paid += confirm_reward_result(&60, RewardOption::Vesting6Month, false);
            rewards_paid += confirm_reward_result(&70, RewardOption::Vesting1Month, true);
            rewards_paid += confirm_reward_result(&80, RewardOption::Vesting3Month, true);

            // Confirm leftover claims got rewards
            let mut leftover_rewards_paid = 0u128;
            leftover_rewards_paid += confirm_leftover_claim_rewards_added(&eth(&charlie()), false);
            leftover_rewards_paid += confirm_leftover_claim_rewards_added(&eth(&dave()), true);

            // Confirm leftover claim without rewards remains unchanged
            confirm_leftover_claim_without_rewards(&eth(&frank()));

            // Confirm total claims amount increased correctly
            assert_eq!(
                <claims::Total<Test>>::get(),
                original_claim_total + leftover_rewards_paid.clone(),
            );

            // Confirm total rewards were paid from reward pool
            assert_eq!(
                mock::RewardMock::total(),
                rewards_paid + leftover_rewards_paid.clone(),
            );

            // Confirm total issuance decreased by the amount rewarded to leftover claims
            assert_eq!(
                Balances::total_issuance(),
                original_issuance - leftover_rewards_paid,
            );

            // Issue one of the leftover claims, and confirm account is not added to Accounts map
            assert_ok!(Claims::claim(Origin::none(), 100, sig::<Test>(&charlie(), &100u64.encode(), &[][..])));
            assert_eq!(<Accounts<Test>>::contains_key(&100), false);
        })
}
