#![cfg_attr(not(feature = "std"), no_std)]

pub mod rewards;
pub mod inflation;
pub mod custody;
pub mod cmix;

use frame_support::traits::{Currency, OnUnbalanced, Get, EnsureOrigin, fungible::Inspect};
use frame_support::{
    decl_event, decl_error, decl_module, decl_storage,
    PalletId, dispatch::DispatchResult, ensure,
    weights::{DispatchClass, Pays},
};
use sp_runtime::traits::{AccountIdConversion, Convert};
use frame_system::{ensure_root, ensure_signed};
use pallet_staking::XXNetworkHandler;

use sp_std::prelude::*;

pub type BalanceOf<T> =
<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

type PositiveImbalanceOf<T> = <<T as Config>::Currency as Currency<
    <T as frame_system::Config>::AccountId, >>::PositiveImbalance;

type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
    <T as frame_system::Config>::AccountId, >>::NegativeImbalance;

pub trait Config: frame_system::Config + pallet_proxy::Config + pallet_staking::Config {

    /// The Event type.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;

    /// The currency mechanism.
    type Currency: Currency<Self::AccountId>;

    /// Balance inspection mechanism.
    type Inspect: Inspect<Self::AccountId>;

    //---------------- REWARDS POOL ----------------//

    /// The RewardsPool sub component id, used to derive its account ID.
    type RewardsPoolId: Get<PalletId>;

    /// The reward remainder handler (Treasury).
    type RewardRemainder: OnUnbalanced<NegativeImbalanceOf<Self>>;

    //----------------   INFLATION  ----------------//

    /// Era duration needed for ideal inflation computation.
    type EraDuration: Get<Self::BlockNumber>;

    //----------------   CUSTODY    ----------------//

    /// The payout frequency of vested coins under custody.
    type PayoutFrequency: Get<Self::BlockNumber>;

    /// The custody duration.
    type CustodyDuration: Get<Self::BlockNumber>;

    /// The governance custody duration.
    type GovernanceCustodyDuration: Get<Self::BlockNumber>;

    /// The getter for the governance proxy type
    type GovernanceProxy: Get<<Self as pallet_proxy::Config>::ProxyType>;

    /// Convert the block number into a balance.
    type BlockNumberToBalance: Convert<Self::BlockNumber, BalanceOf<Self>>;

    //----------------    CMIX      ----------------//

    /// The origin that is allowed to modify cmix variables.
    type CmixVariablesOrigin: EnsureOrigin<Self::Origin>;

    //----------------    ADMIN     ----------------//

    /// The admin origin for the pallet (Tech Committee unanimity).
    type AdminOrigin: EnsureOrigin<Self::Origin>;
}

decl_storage! {
    trait Store for Module<T: Config> as XXNetwork {
        //----------------  INFLATION   ----------------//

        /// Inflation fixed parameters: minimum inflation, ideal stake and curve falloff
        pub InflationParams get(fn inflation_params) config():
            inflation::InflationFixedParams;

        /// List of ideal interest points, defined as a tuple of block number and idea interest
        pub InterestPoints get(fn interest_points) config():
            Vec<inflation::IdealInterestPoint<T::BlockNumber>>;

        /// Ideal liquidity rewards staked amount
        pub IdealLiquidityStake get(fn ideal_stake_rewards) config(): BalanceOf<T>;

        /// Liquidity rewards balance
        pub LiquidityRewards get(fn liquidity_rewards) config(): BalanceOf<T>;

        //----------------   CUSTODY    ----------------//

        /// Keep track of team members'accounts custody info
        pub TeamAccounts get(fn team_accounts): map hasher(twox_64_concat)
            T::AccountId => custody::CustodyInfo<T::AccountId, BalanceOf<T>>;

        /// Keep track of custody accounts
        pub CustodyAccounts get(fn custody_accounts): map hasher(twox_64_concat)
            T::AccountId => ();

        /// Keep track of custodians
        pub Custodians get(fn custodians) config(): map hasher(twox_64_concat)
            T::AccountId => ();

        /// Total amount under custody
        pub TotalCustody get(fn total_custody): BalanceOf<T>;

        //----------------    CMIX      ----------------//

        /// Cmix software hashes
        pub CmixHashes get(fn cmix_hashes) config(): cmix::SoftwareHashes<T::Hash>;

        /// Highest block number that AdminOrigin is allowed to change cmix hashes
        pub AdminPermission get(fn admin_permission) config(): T::BlockNumber;

        /// Scheduling server account
        pub SchedulingAccount get(fn scheduling_account) config(): T::AccountId;

        /// Cmix user ephemeral reception IDs address space size in bits
        pub CmixAddressSpace get(fn cmix_address_space) config(): u8;

        /// Next cmix variables
        pub NextCmixVariables get(fn next_cmix_variables): Option<cmix::Variables>;

        /// Current cmix variables
        pub CmixVariables get(fn cmix_variables) config(): cmix::Variables;
    }
	add_extra_genesis {
	    config(balance): BalanceOf<T>;
	    config(team_allocations): Vec<(T::AccountId, BalanceOf<T>)>;
		build(|config| {
		    //---------------- REWARDS POOL ----------------//
			// Create Rewards pool account and set the balance from genesis
			let account_id = <Module<T>>::rewards_account_id();
            let _ = <T as Config>::Currency::make_free_balance_be(&account_id, config.balance);

            //----------------   CUSTODY    ----------------//
            for &(ref who, balance) in &config.team_allocations {
                // Initialized custody for this member
                <Module<T>>::initialize_custody(who, balance);
            }
		});
	}
}

decl_event! {
    pub enum Event<T> where
        Balance = BalanceOf<T>,
        <T as frame_system::Config>::AccountId,
        <T as frame_system::Config>::BlockNumber,
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

        //----------------   CUSTODY    ----------------//

        /// Team payout was given from custody
        PayoutFromCustody(AccountId, Balance),
        /// Team payout was given from reserve
        PayoutFromReserve(AccountId, Balance),
        /// Custody finished for the given team account
        CustodyDone(AccountId),

        //----------------    ADMIN     ----------------//

        /// Custodian added
        CustodianAdded(AccountId),
        /// Custodian removed
        CustodianRemoved(AccountId),
        /// Team member updated
        TeamMemberUpdated(AccountId, AccountId),

        //----------------    CMIX      ----------------//

        /// Cmix hashes updated
        CmixHashesUpdated,
        /// Admin permission updated
        AdminPermissionUpdated(BlockNumber),
        /// Scheduling server account updated
        SchedulingAccountUpdated,
        /// Cmix variables updated
        CmixVariablesUpdated,
        /// Cmix address space size updated
        CmixAddressSpaceUpdated,
        /// Cmix points data submitted to chain
        CmixPointsAdded,
        /// Cmix points deduction data submitted to chain
        CmixPointsDeducted,
    }
}

decl_error! {
	pub enum Error for Module<T: Config> {
		/// Invalid team member account
        InvalidTeamMember,
        /// Invalid custody account
        InvalidCustodyAccount,
        /// Must be custodian to call this function
        MustBeCustodian,
        /// Payout not available yet
        PayoutNotAvailable,
        /// Payout failed due to insufficient custody + reserve funds
        PayoutFailedInsufficientFunds,
        /// Custody period ended, custodian can't call this function anymore
        CustodyPeriodEnded,
        /// Governance custody ongoing, team member can't call this function yet
        GovernanceCustodyActive,
        /// Governance custody period ended, custodian can't call this function anymore
        GovernanceCustodyPeriodEnded,
        /// This team member account already exists
        TeamMemberExists,
        /// AdminOrigin is not allowed to modify cmix hashes
        AdminPermissionExpired,
        /// Must be scheduling server account to call this function
        MustBeScheduling,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
	    //---------------- REWARDS POOL ----------------//

	    const RewardsPoolId: PalletId = T::RewardsPoolId::get();
	    const RewardsPoolAccount: T::AccountId = T::RewardsPoolId::get().into_account();

        type Error = Error<T>;

	    fn deposit_event() = default;

        //----------------   CUSTODY    ----------------//

	    const PayoutFrequency: T::BlockNumber = T::PayoutFrequency::get();
	    const CustodyDuration: T::BlockNumber = T::CustodyDuration::get();
	    const GovernanceCustodyDuration: T::BlockNumber = T::GovernanceCustodyDuration::get();

	    /// Payout the amount already vested to the given team member account
        ///
        /// Anyone can call this function since it is deterministic
        ///
        /// # <weight>
        /// - TODO: Calculate correct weight
        /// # </weight>
        #[weight = 150_000_000]
        pub fn payout(origin, who: T::AccountId) {
            ensure_signed(origin)?;
            ensure!(Self::is_team_member(&who), Error::<T>::InvalidTeamMember);
            Self::try_payout(who)?;
        }

        /// Bond the given amount from the given custody account, with the specified controller
        ///
        /// During the Custody period, the function is callable by Custodians only. After the
        /// Custody ends, the function is not callable anymore.
        ///
        /// # <weight>
        /// - TODO: Calculate correct weight
        /// # </weight>
        #[weight = 150_000_000]
        pub fn custody_bond(origin,
            custody: T::AccountId,
            controller: T::AccountId,
            #[compact] value: pallet_staking::BalanceOf<T>,
        ) {
            let who = ensure_signed(origin)?;
            ensure!(Self::is_custodian(&who), Error::<T>::MustBeCustodian);
            ensure!(Self::is_custody(&custody), Error::<T>::InvalidCustodyAccount);
            Self::try_custody_bond(custody, controller, value)?;
        }

        /// Bond extra amount from the given custody account
        ///
        /// During the Custody period, the function is callable by Custodians only. After the
        /// Custody ends, the function is not callable anymore.
        ///
        /// # <weight>
        /// - TODO: Calculate correct weight
        /// # </weight>
        #[weight = 150_000_000]
        pub fn custody_bond_extra(origin,
            custody: T::AccountId,
            #[compact] value: pallet_staking::BalanceOf<T>,
        ) {
            let who = ensure_signed(origin)?;
            ensure!(Self::is_custodian(&who), Error::<T>::MustBeCustodian);
            ensure!(Self::is_custody(&custody), Error::<T>::InvalidCustodyAccount);
            Self::try_custody_bond_extra(custody, value)?;
        }

        /// Set the controller of a given custody account
        ///
        /// During the Custody period, the function is callable by Custodians only. After the
        /// Custody ends, the function is not callable anymore.
        ///
        /// # <weight>
        /// - TODO: Calculate correct weight
        /// # </weight>
        #[weight = 150_000_000]
        pub fn custody_set_controller(origin,
            custody: T::AccountId,
            controller: T::AccountId,
        ) {
            let who = ensure_signed(origin)?;
            ensure!(Self::is_custodian(&who), Error::<T>::MustBeCustodian);
            ensure!(Self::is_custody(&custody), Error::<T>::InvalidCustodyAccount);
            Self::try_custody_set_controller(custody, controller)?;
        }

        /// Set the governance proxy of a given custody account
        ///
        /// Only one proxy account is allowed per custody account, so this function
        /// removes any proxies first, and then adds the new proxy
        ///
        /// During the Governance Custody period, the function is callable by Custodians only.
        /// After the Governance Custody ends, the function is not callable anymore.
        ///
        /// # <weight>
        /// - TODO: Calculate correct weight
        /// # </weight>
        #[weight = 150_000_000]
        pub fn custody_set_proxy(origin,
            custody: T::AccountId,
            proxy: T::AccountId,
        ) {
            let who = ensure_signed(origin)?;
            ensure!(Self::is_custodian(&who), Error::<T>::MustBeCustodian);
            ensure!(Self::is_custody(&custody), Error::<T>::InvalidCustodyAccount);
            Self::try_custody_set_proxy(custody, proxy)?;
        }

        /// Allow the team member to set a governance proxy of their own custody account
        ///
        /// During the Governance Custody period, the function is not callable.
        /// After the Governance Custody ends, the function is callable by team members only.
        ///
        /// # <weight>
        /// - TODO: Calculate correct weight
        /// # </weight>
        #[weight = 150_000_000]
        pub fn team_custody_set_proxy(origin, proxy: T::AccountId) {
            let who = ensure_signed(origin)?;
            ensure!(Self::is_team_member(&who), Error::<T>::InvalidTeamMember);
            Self::try_team_custody_set_proxy(who, proxy)?;
        }

        //----------------    ADMIN     ----------------//

        /// Set inflation fixed parameters
        ///
        /// The dispatch origin must be AdminOrigin.
        ///
        /// # <weight>
        /// - O(1) insert
        /// # </weight>
        #[weight = 90_000_000]
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
        /// # <weight>
        /// - O(1) insert
        /// # </weight>
        #[weight = 90_000_000]
        pub fn set_interest_points(origin, points: Vec<inflation::IdealInterestPoint<T::BlockNumber>>) {
            Self::ensure_admin(origin)?;
            <InterestPoints<T>>::put(points);
            Self::deposit_event(RawEvent::InterestPointsChanged);
        }

        /// Set ideal liquidity rewards stake amount
        ///
        /// The dispatch origin must be AdminOrigin.
        /// This can be used to adjust the ideal liquidity reward stake
        ///
        /// # <weight>
        /// - O(1) insert
        /// # </weight>
        #[weight = 90_000_000]
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
        /// # <weight>
        /// - O(1) insert
        /// # </weight>
        #[weight = 90_000_000]
        pub fn set_liquidity_rewards_balance(origin, #[compact] amount: BalanceOf<T>) {
            Self::ensure_admin(origin)?;
            <LiquidityRewards<T>>::put(amount);
            Self::deposit_event(RawEvent::LiquidityRewardsBalanceChanged);
        }

        /// Add a custodian account
        ///
        /// The dispatch origin must be AdminOrigin.
        ///
        /// # <weight>
        /// - TODO: Calculate correct weight
        /// # </weight>
        #[weight = 150_000_000]
        pub fn add_custodian(origin, custodian: T::AccountId) {
            Self::ensure_admin(origin)?;
            <Custodians<T>>::insert(&custodian, ());
            Self::deposit_event(RawEvent::CustodianAdded(custodian));
        }

        /// Remove a custodian account
        ///
        /// The dispatch origin must be AdminOrigin.
        ///
        /// # <weight>
        /// - TODO: Calculate correct weight
        /// # </weight>
        #[weight = 150_000_000]
        pub fn remove_custodian(origin, custodian: T::AccountId) {
            Self::ensure_admin(origin)?;
            <Custodians<T>>::remove(&custodian);
            Self::deposit_event(RawEvent::CustodianRemoved(custodian));
        }

        /// Replace an existing team member account with a new account
        ///
        /// The dispatch origin must be AdminOrigin.
        ///
        /// # <weight>
        /// - TODO: Calculate correct weight
        /// # </weight>
        #[weight = 150_000_000]
        pub fn replace_team_member(origin, who: T::AccountId, new: T::AccountId) {
            Self::ensure_admin(origin)?;
            ensure!(Self::is_team_member(&who), Error::<T>::InvalidTeamMember);
            ensure!(!Self::is_team_member(&new), Error::<T>::TeamMemberExists);
            Self::update_team_member(who.clone(), new.clone());
            Self::deposit_event(RawEvent::TeamMemberUpdated(who, new));
        }

        //----------------    CMIX      ----------------//

        /// Set cmix software hashes
        ///
        /// The dispatch origin must be AdminOrigin.
        /// Furthermore, this call is only allowed if current block is lower than `AdminPermission`.
        ///
        /// # <weight>
        /// - O(1) insert
        /// # </weight>
        #[weight = 90_000_000]
        pub fn set_cmix_hashes(origin, hashes: cmix::SoftwareHashes<T::Hash>) {
            Self::ensure_admin(origin)?;
            Self::ensure_admin_allowed_cmix_hashes()?;
            <CmixHashes<T>>::put(hashes);
            Self::deposit_event(RawEvent::CmixHashesUpdated);
        }

        /// Set scheduling server account
        ///
        /// The dispatch origin must be AdminOrigin.
        ///
        /// # <weight>
        /// - O(1) insert
        /// # </weight>
        #[weight = 90_000_000]
        pub fn set_scheduling_account(origin, who: T::AccountId) {
            Self::ensure_admin(origin)?;
            <SchedulingAccount<T>>::put(who);
            Self::deposit_event(RawEvent::SchedulingAccountUpdated);
        }

        /// Set next cmix variables
        ///
        /// The dispatch origin must be `CmixVariablesOrigin`.
        /// The new variables will be stored in `NextCmixVariables`.
        /// Then, at the beginning of the next era, `NextCmixVariables` is emptied and the value
        /// is written to `CmixVariables`.
        ///
        /// # <weight>
        /// - O(1) insert
        /// # </weight>
        #[weight = 90_000_000]
        pub fn set_next_cmix_variables(origin, variables: cmix::Variables) {
            Self::ensure_cmix_variables(origin)?;
            NextCmixVariables::put(variables);
        }

        /// Submit cmix performance points
        ///
        /// `data` is a vector of tuples of (account, points)
        /// The dispatch origin must be `SchedulingAccount`
        ///
        /// # <weight>
        /// - TODO: Calculate correct weight
        /// # </weight>
        #[weight = (
			300_000_000,
			DispatchClass::Operational,
			Pays::No
		)]
        pub fn submit_cmix_points(origin, data: Vec<(T::AccountId, u32)>) {
            let who = ensure_signed(origin)?;
            ensure!(Self::is_scheduling(who), Error::<T>::MustBeScheduling);
            Self::reward_cmix_points(data);
            Self::deposit_event(RawEvent::CmixPointsAdded);
        }

        /// Submit cmix performance points deductions
        ///
        /// `data` is a vector of tuples of (account, points)
        /// The dispatch origin must be `SchedulingAccount`
        ///
        /// # <weight>
        /// - TODO: Calculate correct weight
        /// # </weight>
        #[weight = (
			300_000_000,
			DispatchClass::Operational,
			Pays::No
		)]
        pub fn submit_cmix_deductions(origin, data: Vec<(T::AccountId, u32)>) {
            let who = ensure_signed(origin)?;
            ensure!(Self::is_scheduling(who), Error::<T>::MustBeScheduling);
            Self::deduct_cmix_points(data);
            Self::deposit_event(RawEvent::CmixPointsDeducted);
        }

        /// Set cmix address space size
        ///
        /// The dispatch origin must be `SchedulingAccount`
        ///
        /// # <weight>
        /// - O(1) insert
        /// # </weight>
        #[weight = (
			90_000_000,
			DispatchClass::Operational,
			Pays::No
		)]
        pub fn set_cmix_address_space(origin, size: u8) {
            let who = ensure_signed(origin)?;
            ensure!(Self::is_scheduling(who), Error::<T>::MustBeScheduling);
            CmixAddressSpace::put(size);
            Self::deposit_event(RawEvent::CmixAddressSpaceUpdated);
        }

        //----------------  GOVERNANCE  ----------------//

        /// Set admin permission
        ///
        /// `permission` is the block number up to which the AdminOrigin
        /// will be allowed to call the `set_cmix_hashes` function.
        /// It is expected that `permission` will be modified by Democracy
        /// in 6-month periods.
        ///
        /// # <weight>
        /// - O(1) insert
        /// # </weight>
        #[weight = 90_000_000]
        pub fn set_admin_permission(origin, permission: T::BlockNumber) {
            ensure_root(origin)?;
            <AdminPermission<T>>::put(permission);
            Self::deposit_event(RawEvent::AdminPermissionUpdated(permission));
        }
	}
}

impl<T: Config> Module<T> {
    /// Check if given account is a team member
    fn is_team_member(who: &T::AccountId) -> bool {
        <TeamAccounts<T>>::contains_key(who)
    }

    /// Check if given account is a custody account
    fn is_custody(who: &T::AccountId) -> bool {
        <CustodyAccounts<T>>::contains_key(who)
    }

    /// Check if given account is a custodian
    fn is_custodian(who: &T::AccountId) -> bool {
        <Custodians<T>>::contains_key(who)
    }

    /// Check if origin is admin
    fn ensure_admin(o: T::Origin) -> DispatchResult {
        <T as Config>::AdminOrigin::try_origin(o)
            .map(|_| ())
            .or_else(ensure_root)?;
        Ok(())
    }

    /// Checks if admin is allowed to modify cmix hashes
    fn ensure_admin_allowed_cmix_hashes() -> DispatchResult {
        let block = <frame_system::Pallet<T>>::block_number();
        let permission = <AdminPermission<T>>::get();
        ensure!(permission >= block, Error::<T>::AdminPermissionExpired);
        Ok(())
    }

    /// Check if given account is scheduling server
    fn is_scheduling(who: T::AccountId) -> bool {
        who == <SchedulingAccount<T>>::get()
    }

    /// Check if origin is cmix variables
    fn ensure_cmix_variables(o: T::Origin) -> DispatchResult {
        T::CmixVariablesOrigin::try_origin(o)
            .map(|_| ())
            .or_else(ensure_root)?;
        Ok(())
    }

    //----------------    CMIX      ----------------//

    /// Add cmix points to staking era rewards
    pub fn reward_cmix_points(data: Vec<(T::AccountId, u32)>) {
        <pallet_staking::Module<T>>::reward_by_ids(data)
    }

    /// Deduct cmix points from staking era rewards
    pub fn deduct_cmix_points(data: Vec<(T::AccountId, u32)>) {
        <pallet_staking::Module<T>>::deduct_by_ids(data)
    }
}

/// Implement XXNetworkHandler trait
impl<T: Config> XXNetworkHandler<T::AccountId> for Module<T> {
    fn is_custody_account(who: &T::AccountId) -> bool {
        Self::is_custody(who)
    }
    fn get_block_points() -> u32 {
        let variables = CmixVariables::get();
        variables.get_block_points()
    }
    fn end_era() {
        // Update cmix variables if next ones are set
        if let Some(next) = NextCmixVariables::take() {
            CmixVariables::put(next);
            Self::deposit_event(RawEvent::CmixVariablesUpdated);
        }
    }
}
