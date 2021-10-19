#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::{Currency, Get, OnUnbalanced, VestingSchedule};
use frame_support::{
    decl_event, decl_error, decl_module, decl_storage, ensure, weights::{Weight, DispatchClass},
    StorageValue, StorageMap, IterableStorageMap,
};
use sp_runtime::{PerThing, Perbill, RuntimeDebug, traits::Zero};
use frame_system::{ensure_root, ensure_signed};

use sp_std::prelude::*;
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

pub trait Config: frame_system::Config + claims::Config + pallet_vesting::Config {

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
                let lock = (info.option.principal_lock() * info.principal) + reward_amount;
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

    /// Process a leftover claim
    fn process_leftover_claim(
        address: claims::EthereumAddress,
        reward: BalanceOf<T>)
    {
        // 1. Compute vesting parameters
        // If a vesting schedule exists, add the reward amount to the lock
        // and recompute per block amount
        let vesting = <claims::Vesting<T>>::take(&address);
        let (locked, per_block, start_block) = if let Some(vs) = vesting {
            // Add reward amount to locked
            let new_locked = vs.0.clone() + reward.clone();
            // Compute new per block amount
            // The lock duration stays the same, so:
            //   1. lock_time = orig_locked/orig_per_block
            //   2. lock_time = new_locked/new_per_block
            // 1 = 2 <=> new_per_block = orig_per_block * (new_locked/orig_locked)
            // = orig_per_block + orig_per_block * (rewards/orig_locked)
            let ratio = Perbill::from_rational(reward.clone(), vs.0);
            let new_per_block = (ratio * vs.1.clone()) + vs.1;
            (new_locked, new_per_block, vs.2)
        } else {
            // Compute vesting from default option
            let option: RewardOption = Default::default();
            let principal = <claims::Claims<T>>::get(&address).unwrap_or_default();
            let lock = (option.principal_lock() * principal) + reward.clone();
            let per_block = lock.clone() / option.vesting_period().into();
            (lock, per_block, T::EnactmentBlock::get())
        };

        // 2. "Payout" reward into claim
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
        <claims::Total<T>>::mutate(|t| *t += reward);
        // Explicitly drop credit in order to burn from issuance
        drop(credit);

        // 3. Add vesting schedule
        <claims::Vesting<T>>::insert(&address, (locked, per_block, start_block));
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
