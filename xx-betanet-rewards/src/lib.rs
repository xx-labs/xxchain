#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::{Currency, Get, OnUnbalanced, VestingSchedule};
use frame_support::{
    decl_event, decl_error, decl_module, decl_storage, ensure, weights::{Weight, DispatchClass},
    StorageValue, StorageMap, IterableStorageMap,
};
use sp_runtime::{PerThing, Perbill, RuntimeDebug, traits::{Zero, SaturatedConversion}};
use frame_system::{ensure_root, ensure_signed};

use sp_std::prelude::*;
use sp_std::convert::TryFrom;
use codec::{Encode, Decode};
use claims::CurrencyOf;
use claims::BalanceOf;

type PositiveImbalanceOf<T> = <CurrencyOf<T> as Currency<<T as frame_system::Config>::AccountId>>::PositiveImbalance;

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
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

        /// Store rewards for each user
        pub Rewards get(fn rewards) config(): map hasher(twox_64_concat)
            T::AccountId => BalanceOf<T>;

        /// Store principal for each user
        pub Principal get(fn principal) config(): map hasher(twox_64_concat)
            T::AccountId => BalanceOf<T>;

        /// Store option selected by each user
        pub Options get(fn reward_options): map hasher(twox_64_concat)
            T::AccountId => RewardOption;

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
        /// Account has vesting schedule, can't select option
        CantSelectOption,
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
            ensure!(<Rewards<T>>::contains_key(&who), Error::<T>::NoRewards);
            ensure!(!<pallet_vesting::Vesting<T>>::contains_key(&who), Error::<T>::CantSelectOption);
            <Options<T>>::insert(&who, option.clone());
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
    fn add_claimed(dest: T::AccountId, vesting: bool, claim: BalanceOf<T>, reward: BalanceOf<T>) {
        let block = <frame_system::Pallet<T>>::block_number();
        if block >= T::EnactmentBlock::get() {
            return
        }
        <Rewards<T>>::insert(&dest, reward);
        if !vesting {
            <Principal<T>>::insert(&dest, claim)
        }
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

    /// Process a single account or leftover claim
    fn process_account(
        claim: bool,
        account: T::AccountId,
        address: claims::EthereumAddress,
        reward: BalanceOf<T>)
    {
        // 1. Take existing vesting schedule from vesting pallet or claims
        let (has_vest, vs_lock, vs_per_block, vs_start_block) = if claim {
            let vesting = <claims::Vesting<T>>::take(&address);
            if let Some(vs) = vesting {
                (true, vs.0, vs.1, vs.2)
            } else {
                (false, Zero::zero(), Zero::zero(), Zero::zero())
            }
        } else {
            let vesting = <pallet_vesting::Vesting<T>>::take(&account);
            if let Some(vs) = vesting {
                let locked = <BalanceOf<T>>::try_from(vs.locked.saturated_into::<u128>()).ok()
                    .unwrap_or(Zero::zero());
                let per_block = <BalanceOf<T>>::try_from(vs.per_block.saturated_into::<u128>()).ok()
                    .unwrap_or(Zero::zero());
                (true, locked, per_block, vs.starting_block)
            } else {
                (false, Zero::zero(), Zero::zero(), Zero::zero())
            }
        };

        // 2. Take reward option or default
        let reward_option = if has_vest {
            RewardOption::Vesting9Month
        } else if claim {
            Default::default()
        } else {
            <Options<T>>::take(&account)
        };

        // 3. Compute reward based on option
        let extra = reward_option.extra_rewards();
        let reward_amount = if extra.is_zero() {
            reward_option.rewards() * reward.clone()
        } else {
            (reward_option.rewards() * reward.clone()) + (extra * reward)
        };

        // 4. Compute new vesting
        let (locked, per_block, start_block) = if has_vest {
            // Add reward amount to locked
            let new_locked = vs_lock.clone() + reward_amount.clone();

            // Compute new per block amount
            // The lock duration stays the same, so:
            //   1. lock_time = orig_locked/orig_per_block
            //   2. lock_time = new_locked/new_per_block
            // 1 = 2 <=> new_per_block = orig_per_block * (new_locked/orig_locked)
            // = orig_per_block + orig_per_block * (rewards/orig_locked)
            let ratio = Perbill::from_rational(reward_amount.clone(), vs_lock);
            let new_per_block = (ratio * vs_per_block.clone()) + vs_per_block;

            (new_locked, new_per_block, vs_start_block)
        } else {
            match reward_option {
                RewardOption::NoVesting => {
                    // Clear principal when no vesting
                    <Principal<T>>::remove(&account);
                    (Zero::zero(), Zero::zero(), Zero::zero())
                },
                _ => {
                    // Compute amount to lock
                    // Get principal from storage or claim
                    let principal = if claim {
                        <claims::Claims<T>>::get(&address).unwrap_or_default()
                    } else {
                        <Principal<T>>::take(&account)
                    };

                    // lock = principal_lock * principal + rewards
                    let lock = (reward_option.principal_lock() * principal) + reward_amount.clone();

                    // Compute amount per block
                    // per_block = lock / vesting_period
                    let per_block = lock.clone() / reward_option.vesting_period().into();

                    (lock, per_block, Zero::zero())
                }
            }
        };

        // 5. If processing account, deposit reward and handle imbalance
        // Otherwise, add reward value to claim and burn equal amount from T::Reward
        if !claim {
            // Deposit rewards in account, creating positive imbalance
            let imbalance = <CurrencyOf<T>>::deposit_creating(&account, reward_amount);
            // Debit the positive imbalance from T::Reward
            T::Reward::on_unbalanced(imbalance);
        } else {
            // Create a pair of imbalances
            let (debit, credit) = <CurrencyOf<T>>::pair(reward_amount.clone());
            // Debit the positive imbalance from T::Reward
            T::Reward::on_unbalanced(debit);
            // Add rewards amount to claim
            <claims::Claims<T>>::mutate(&address, |val| {
                if let Some(claim) = val {
                    *claim += reward_amount.clone()
                }
            });
            <claims::Total<T>>::mutate(|t| *t += reward_amount);
            // Explicitly drop credit in order to burn from issuance
            drop(credit);
        }

        // 6. Add vesting schedule
        if claim {
            <claims::Vesting<T>>::insert(&address, (locked, per_block, start_block));
        } else {
            match reward_option {
                RewardOption::NoVesting => (),
                _ => {
                    <T as claims::Config>::VestingSchedule::add_vesting_schedule(
                        &account,
                        locked,
                        per_block,
                        start_block,
                    ).expect("No vesting schedule exists, as checked above; qed");
                }
            }
        }
    }

    /// Process rewards
    fn process_rewards() {
        <Rewards<T>>::drain().for_each(|(account, reward)| {
            Self::process_account(false, account, Default::default(), reward)
        })
    }

    /// Process claims
    fn process_claims() {
        <claims::Rewards<T>>::drain().for_each(|(address, reward)| {
            Self::process_account(true, Default::default(), address, reward)
        })
    }
}
