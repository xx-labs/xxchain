#![cfg_attr(not(feature = "std"), no_std)]

pub mod rewards;
pub mod inflation;
pub mod weights;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;

use frame_support::traits::{Currency, OnUnbalanced, Get, EnsureOrigin};
use frame_support::{
    decl_event, decl_module, decl_storage,
    PalletId, dispatch::DispatchResult,
};
pub use weights::WeightInfo;
use sp_runtime::traits::{AccountIdConversion};
use frame_system::{ensure_root};


use sp_std::prelude::*;

pub type BalanceOf<T> =
<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

type PositiveImbalanceOf<T> = <<T as Config>::Currency as Currency<
    <T as frame_system::Config>::AccountId, >>::PositiveImbalance;

type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
    <T as frame_system::Config>::AccountId, >>::NegativeImbalance;

pub trait Config: frame_system::Config {

    /// The Event type.
    type RuntimeEvent: From<Event<Self>> + Into<<Self as frame_system::Config>::RuntimeEvent>;

    /// The currency mechanism.
    type Currency: Currency<Self::AccountId>;

    /// Handler with which to retrieve total token custody
    type CustodyHandler: pallet_staking::CustodyHandler<Self::AccountId, BalanceOf<Self>>;

    /// Handler to retrieve public accounts
    type PublicAccountsHandler: xx_public::PublicAccountsHandler<Self::AccountId>;

    //---------------- REWARDS POOL ----------------//

    /// The RewardsPool sub component id, used to derive its account ID.
    type RewardsPoolId: Get<PalletId>;

    /// The reward remainder handler (Treasury).
    type RewardRemainder: OnUnbalanced<NegativeImbalanceOf<Self>>;

    //----------------   INFLATION  ----------------//

    /// Era duration needed for ideal inflation computation.
    type EraDuration: Get<Self::BlockNumber>;

    /// The admin origin for the pallet (Tech Committee unanimity).
    type AdminOrigin: EnsureOrigin<Self::RuntimeOrigin>;

    /// Weight information for extrinsics in this pallet.
    type WeightInfo: WeightInfo;
}

decl_storage! {
    trait Store for Module<T: Config> as XXEconomics {
        //----------------  INFLATION   ----------------//

        /// Inflation fixed parameters: minimum inflation, ideal stake and curve falloff
        pub InflationParams get(fn inflation_params) config():
            inflation::InflationFixedParams;

        /// List of ideal interest points, defined as a tuple of block number and idea interest
        pub InterestPoints get(fn interest_points) config() build(|config: &GenesisConfig<T>| {
            // Sort points when building from genesis
            let mut points = config.interest_points.clone();
            points.sort_by(|a, b| a.block.cmp(&b.block));
            points
        }): Vec<inflation::IdealInterestPoint<T::BlockNumber>>;

        /// Ideal liquidity rewards staked amount
        pub IdealLiquidityStake get(fn ideal_stake_rewards) config(): BalanceOf<T>;

        /// Liquidity rewards balance
        pub LiquidityRewards get(fn liquidity_rewards) config(): BalanceOf<T>;

    }
	add_extra_genesis {
	    config(balance): BalanceOf<T>;
		build(|config| {
		    //---------------- REWARDS POOL ----------------//
			// Create Rewards pool account and set the balance from genesis
			let account_id = <Module<T>>::rewards_account_id();
            let _ = <T as Config>::Currency::make_free_balance_be(&account_id, config.balance);
		});
	}
}

decl_event! {
    pub enum Event<T> where
        Balance = BalanceOf<T>,
    {
        //---------------- REWARDS POOL ----------------//

        /// Rewards were given from the pool
        RewardFromPool(Balance),
        /// Rewards were minted
        RewardMinted(Balance),

        //----------------  INFLATION   ----------------//

        /// Inflation fixed parameters were changed
        InflationParamsChanged,
        /// Ideal interest points were changed
        InterestPointsChanged,
        /// Ideal liquidity rewards stake was changed
        IdealLiquidityStakeChanged,
        /// Liquidity rewards balance was changed
        LiquidityRewardsBalanceChanged,
    }
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::RuntimeOrigin {
	    //---------------- REWARDS POOL ----------------//

	    const RewardsPoolId: PalletId = T::RewardsPoolId::get();
	    const RewardsPoolAccount: T::AccountId = T::RewardsPoolId::get().into_account_truncating();

	    fn deposit_event() = default;

        //----------------    ADMIN     ----------------//

        /// Set inflation fixed parameters
        ///
        /// The dispatch origin must be AdminOrigin.
        ///
        #[weight = <T as Config>::WeightInfo::set_inflation_params()]
        pub fn set_inflation_params(origin, params: inflation::InflationFixedParams) {
            Self::ensure_admin(origin)?;
            <InflationParams>::put(params);
            Self::deposit_event(RawEvent::InflationParamsChanged);
        }

        /// Set ideal interest points
        ///
        /// Overwrites the full list of points. Doesn't check if points are ordered per block.
        /// It's up to the caller to ensure the ordering, otherwise leads to unexpected behavior.
        ///
        /// The dispatch origin must be AdminOrigin.
        ///
        #[weight = <T as Config>::WeightInfo::set_interest_points()]
        pub fn set_interest_points(origin, points: Vec<inflation::IdealInterestPoint<T::BlockNumber>>) {
            Self::ensure_admin(origin)?;
            // Insert sorted vector of points
            let mut sorted_points = points.clone();
            sorted_points.sort_by(|a, b| a.block.cmp(&b.block));
            <InterestPoints<T>>::put(sorted_points);
            Self::deposit_event(RawEvent::InterestPointsChanged);
        }

        /// Set ideal liquidity rewards stake amount
        ///
        /// The dispatch origin must be AdminOrigin.
        /// This can be used to adjust the ideal liquidity reward stake
        ///
        #[weight = <T as Config>::WeightInfo::set_liquidity_rewards_stake()]
        pub fn set_liquidity_rewards_stake(origin, #[compact] amount: BalanceOf<T>) {
            Self::ensure_admin(origin)?;
            <IdealLiquidityStake<T>>::put(amount);
            Self::deposit_event(RawEvent::IdealLiquidityStakeChanged);
        }

        /// Set balance of liquidity rewards
        ///
        /// The dispatch origin must be AdminOrigin.
        /// This should only be used to make corrections to liquidity rewards balance
        /// according to data from ETH chain
        ///
        #[weight = <T as Config>::WeightInfo::set_liquidity_rewards_balance()]
        pub fn set_liquidity_rewards_balance(origin, #[compact] amount: BalanceOf<T>) {
            Self::ensure_admin(origin)?;
            <LiquidityRewards<T>>::put(amount);
            Self::deposit_event(RawEvent::LiquidityRewardsBalanceChanged);
        }
	}
}

impl<T: Config> Module<T> {
    /// Check if origin is admin
    fn ensure_admin(o: T::RuntimeOrigin) -> DispatchResult {
        <T as Config>::AdminOrigin::try_origin(o)
            .map(|_| ())
            .or_else(ensure_root)?;
        Ok(())
    }
}

// Manual implementation of WhitelistedStorageKeys for runtime benchmarks
#[cfg(feature = "runtime-benchmarks")]
impl<T: Config> frame_support::traits::WhitelistedStorageKeys for Module<T> {
    fn whitelisted_storage_keys() -> frame_support::sp_std::vec::Vec<frame_benchmarking::TrackedStorageKey> {
        use frame_support::sp_std::vec;
        vec![]
    }
}
