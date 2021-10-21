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
fn rewards_works_for_genesis_accounts_no_vesting() {
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
                xx_economics::Module::<Test>::rewards_balance(),
                RewardsPoolBalance::get() - rewards_paid,
            );

            // Confirm total issuance remained the same
            assert_eq!(
                Balances::total_issuance(),
                original_issuance,
            );
        })
}
