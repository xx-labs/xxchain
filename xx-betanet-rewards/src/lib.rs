#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

use frame_support::traits::{Currency, Get, OnUnbalanced, VestingSchedule};
use frame_support::{
    decl_event, decl_error, decl_module, decl_storage, ensure, weights::{Weight, DispatchClass},
    StorageValue, StorageMap, IterableStorageMap,
};
use sp_runtime::{PerThing, Perbill, RuntimeDebug, traits::{Saturating, Zero, SaturatedConversion}};
use frame_system::{ensure_root, ensure_signed};

use sp_std::prelude::*;
use sp_std::convert::TryFrom;
use codec::{Encode, Decode};
use claims::CurrencyOf;
use claims::BalanceOf;

type PositiveImbalanceOf<T> = <CurrencyOf<T> as Currency<<T as frame_system::Config>::AccountId>>::PositiveImbalance;

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub enum RewardOption {
    /// No Vesting, pays 2% rewards
    NoVesting,
    /// 1 month vest for 100% principal, pays 12% rewards
    Vesting1Month,
    /// 3 month vest for 90% principal, pays 45% rewards
    Vesting3Month,
    /// 6 month vest for 80% principal, pays 100% rewards (Default)
    Vesting6Month,
    /// 9 month vest for 100% principal, pays 120% rewards
    Vesting9Month,
}

impl Default for RewardOption {
    fn default() -> Self {
        RewardOption::Vesting6Month
    }
}

impl RewardOption {
    /// Return the vesting period of the given reward option
    /// Assumes 6s blocks
    pub fn vesting_period(&self) -> u32 {
        match self {
            RewardOption::NoVesting => 0,
            RewardOption::Vesting1Month => 432000,
            RewardOption::Vesting3Month => 1296000,
            RewardOption::Vesting6Month => 2592000,
            RewardOption::Vesting9Month => 3888000,
        }
    }

    /// Return the percentage of principal that is locked
    pub fn principal_lock(&self) -> Perbill {
        match self {
            RewardOption::NoVesting => PerThing::zero(),
            RewardOption::Vesting1Month => Perbill::from_rational(1u32, 1u32),
            RewardOption::Vesting3Month => Perbill::from_rational(9u32, 10u32),
            RewardOption::Vesting6Month => Perbill::from_rational(8u32, 10u32),
            RewardOption::Vesting9Month => Perbill::from_rational(1u32, 1u32),
        }
    }

    /// Compute the percentage of rewards that are paid
    pub fn rewards(&self) -> Perbill {
        match self {
            RewardOption::NoVesting => Perbill::from_rational(2u32, 100u32),
            RewardOption::Vesting1Month => Perbill::from_rational(12u32, 100u32),
            RewardOption::Vesting3Month => Perbill::from_rational(45u32, 100u32),
            RewardOption::Vesting6Month => Perbill::from_rational(1u32, 1u32),
            RewardOption::Vesting9Month => Perbill::from_rational(1u32, 1u32),
        }
    }

    /// Return the extra percentage rewards that are paid (only for 9 month vest)
    pub fn extra_rewards(&self) -> Perbill {
        match self {
            RewardOption::NoVesting => PerThing::zero(),
            RewardOption::Vesting1Month => PerThing::zero(),
            RewardOption::Vesting3Month => PerThing::zero(),
            RewardOption::Vesting6Month => PerThing::zero(),
            RewardOption::Vesting9Month => Perbill::from_rational(2u32, 10u32),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct UserInfo<Balance> {
    /// Principal value for this user
    pub principal: Balance,
    /// Reward amount for this user
    pub reward: Balance,
    /// Option selected by the user
    pub option: RewardOption,
}

impl<Balance: Zero> Default for UserInfo<Balance> {
    fn default() -> Self {
        UserInfo {
            principal: Zero::zero(),
            reward: Zero::zero(),
            option: Default::default(),
        }
    }
}

pub trait Config: frame_system::Config + claims::Config {

    /// The Event type.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;

    /// The enactment block to payout betanet staking rewards if approved
    type EnactmentBlock: Get<Self::BlockNumber>;

    /// The reward handler for paying out betanet rewards
    type Reward: OnUnbalanced<PositiveImbalanceOf<Self>>;
}

decl_storage! {
    trait Store for Module<T: Config> as XXBetanetRewards {

        /// Store user info
        pub Accounts get(fn accounts) config(): map hasher(twox_64_concat)
            T::AccountId => UserInfo<BalanceOf<T>>;

        /// Store if the rewards program is approved
        pub Approved get (fn approved): bool;
    }
}

decl_event! {
    pub enum Event<T> where
        AccountId = <T as frame_system::Config>::AccountId
    {
        /// Reward option has been selected
        OptionSelected(AccountId, RewardOption),
        /// BetaNet Staking Program has been approved
        ProgramApproved,
        /// BetaNet Staking Program has been enacted
        ProgramEnacted,
    }
}

decl_error! {
	pub enum Error for Module<T: Config> {
        /// Account doesn't have rewards
        NoRewards,
        /// Enactment block has passed
        EnactmentBlockHasPassed,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
        type Error = Error<T>;

	    fn deposit_event() = default;

	    const EnactmentBlock: T::BlockNumber = T::EnactmentBlock::get();

	    /// Select desired option for BetaNet Staking Rewards
        ///
        /// Only callable by accounts that have rewards and no vesting period
        ///
        /// # <weight>
        /// - TODO: Calculate correct weight
        /// # </weight>
        #[weight = 150_000_000]
        pub fn select_option(origin, option: RewardOption) {
            let who = ensure_signed(origin)?;
            ensure!(<Accounts<T>>::contains_key(&who), Error::<T>::NoRewards);
            let block = <frame_system::Pallet<T>>::block_number();
            ensure!(block < T::EnactmentBlock::get(), Error::<T>::EnactmentBlockHasPassed);
            <Accounts<T>>::mutate(&who, |info| {
                let curr_opt = &mut info.option;
                *curr_opt = option.clone();
            });
            Self::deposit_event(RawEvent::OptionSelected(who, option))
        }

        /// Approve the BetaNet Staking Rewards program
        ///
        /// The dispatch origin must be Root.
        ///
        /// # <weight>
        /// - TODO: Calculate correct weight
        /// # </weight>
        #[weight = (
			90_000_000,
			DispatchClass::Operational

		)]
        pub fn approve(origin) {
            ensure_root(origin)?;
            let block = <frame_system::Pallet<T>>::block_number();
            ensure!(block < T::EnactmentBlock::get(), Error::<T>::EnactmentBlockHasPassed);
            <Approved>::put(true);
            Self::deposit_event(RawEvent::ProgramApproved)
        }

        /// When enactment block is reached
        fn on_initialize(n: T::BlockNumber) -> Weight {
            if n == T::EnactmentBlock::get() {
                let approved = Approved::get();
                if approved {
                    Self::enact_program();
                    Self::deposit_event(RawEvent::ProgramEnacted);
                    T::BlockWeights::get().max_block
                } else {
                    0
                }
            } else {
                0
            }
        }
    }
}

/// Implement reward handler
impl<T: Config> claims::RewardHandler<T::AccountId, BalanceOf<T>> for Module<T> {
    /// Add a claimed account to rewards
    fn add_claimed(dest: T::AccountId, claim: BalanceOf<T>, reward: BalanceOf<T>) {
        let block = <frame_system::Pallet<T>>::block_number();
        if block >= T::EnactmentBlock::get() {
            return
        }
        let info = UserInfo {
            principal: claim,
            reward,
            option: Default::default(),
        };
        <Accounts<T>>::insert(&dest, info);
    }
}

impl<T: Config> Module<T> {
    /// Enact program if approved
    fn enact_program() {
        // 1. Process rewards
        Self::process_rewards();
        // 2. Process leftover claims
        Self::process_claims()
    }

    /// Process a single account
    fn process_account(
        account: T::AccountId,
        info: UserInfo<BalanceOf<T>>)
    {
        // 1. Compute reward amount based on option
        let extra = info.option.extra_rewards();
        let reward_amount = if extra.is_zero() {
            info.option.rewards() * info.reward.clone()
        } else {
            (info.option.rewards() * info.reward.clone()) + (extra * info.reward)
        };

        // 2. Payout reward
        // Deposit rewards in account, creating positive imbalance
        let imbalance = <CurrencyOf<T>>::deposit_creating(&account, reward_amount.clone());
        // Debit the positive imbalance from T::Reward
        T::Reward::on_unbalanced(imbalance);

        // 3. Add vesting schedule
        match info.option {
            RewardOption::NoVesting => (),
            _ => {
                // Compute vesting parameters based on option
                let desired_lock = (info.option.principal_lock() * info.principal) + reward_amount;
                // If account has any vesting schedule, adjust lock
                let current_locked = <T as claims::Config>::VestingSchedule::vesting_balance(&account);
                let lock = if let Some(l) = current_locked {
                    desired_lock.saturating_sub(l)
                } else {
                    desired_lock
                };
                if !lock.is_zero() {
                    let per_block = lock.clone() / info.option.vesting_period().into();
                    let _ = <T as claims::Config>::VestingSchedule::add_vesting_schedule(
                        &account,
                        lock,
                        per_block,
                        T::EnactmentBlock::get(),
                    );
                }
            }
        }
    }

    /// Compute claims vesting lock based on desired value and existing schedules
    fn compute_claims_vesting_lock(who: &claims::EthereumAddress, desired: BalanceOf<T>)
        -> BalanceOf<T> {
        let block = T::EnactmentBlock::get();
        let mut lock = desired;
        // Get existing vesting schedules from claims pallet
        if let Some(schedules) = <claims::Vesting<T>>::get(who) {
            // For each schedule, get how much is locked at the enactment block
            // and subtract it from desired lock
            schedules.iter().for_each( |vs| {
                let blocks_as_balance = <BalanceOf<T>>::try_from(
                    block.saturating_sub(vs.2).saturated_into::<u128>()
                ).ok().unwrap_or(Zero::zero());
                lock = lock.saturating_sub(vs.0.saturating_sub(vs.1*blocks_as_balance))
            });
        }
        lock
    }

    /// Process a leftover claim
    fn process_leftover_claim(
        address: claims::EthereumAddress,
        reward: BalanceOf<T>)
    {
        // For leftover claims, the default option is applied

        // 1. "Payout" reward into claim
        // Create a pair of imbalances
        let (debit, credit) = <CurrencyOf<T>>::pair(reward.clone());
        // Debit the positive imbalance from T::Reward
        T::Reward::on_unbalanced(debit);
        // Add rewards amount to claim
        <claims::Claims<T>>::mutate(&address, |val| {
            if let Some(claim) = val {
                *claim += reward.clone()
            }
        });
        <claims::Total<T>>::mutate(|t| *t += reward.clone());
        // Explicitly drop credit in order to burn from issuance
        drop(credit);

        // 2. Add vesting schedule (if needed)
        // Compute vesting parameters
        let option: RewardOption = Default::default();
        let principal = <claims::Claims<T>>::get(&address).unwrap_or_default();
        let desired_lock = (option.principal_lock() * principal) + reward;
        // If account has any vesting schedule, adjust lock
        let lock = Self::compute_claims_vesting_lock(
            &address,
            desired_lock,
        );

        if !lock.is_zero() {
            let per_block = lock.clone() / option.vesting_period().into();
            <claims::Vesting<T>>::append(&address, (lock, per_block, T::EnactmentBlock::get()));
        }
    }

    /// Process rewards
    fn process_rewards() {
        <Accounts<T>>::drain().for_each(|(account, info)| {
            Self::process_account(account, info)
        })
    }

    /// Process claims
    fn process_claims() {
        <claims::Rewards<T>>::drain().for_each(|(address, reward)| {
            Self::process_leftover_claim(address, reward)
        })
    }
}
