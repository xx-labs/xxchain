#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::{Currency, Get, EnsureOrigin, fungible::Inspect};
use frame_support::{
    decl_event, decl_error, decl_module, decl_storage, dispatch::DispatchResult, ensure,
};
use sp_runtime::traits::{Convert};
use frame_system::{ensure_root, ensure_signed};
use weights::WeightInfo;
use sp_std::prelude::*;

pub mod custody;
pub mod weights;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;

pub type BalanceOf<T> =
<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub trait Config: frame_system::Config + pallet_proxy::Config + pallet_staking::Config {

    /// The Event type.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;

    /// The currency mechanism.
    type Currency: Currency<Self::AccountId> + Inspect<Self::AccountId>;

    //----------------   CUSTODY    ----------------//

    /// The payout frequency of vested coins under custody.
    type PayoutFrequency: Get<Self::BlockNumber>;

    /// The custody duration.
    type CustodyDuration: Get<Self::BlockNumber>;

    /// The governance custody duration.
    type GovernanceCustodyDuration: Get<Self::BlockNumber>;

    /// The getter for the proxy type to use for custody accounts
    type CustodyProxy: Get<<Self as pallet_proxy::Config>::ProxyType>;

    /// Convert the block number into a balance.
    type BlockNumberToBalance: Convert<Self::BlockNumber, BalanceOf<Self>>;

    //----------------    ADMIN     ----------------//

    /// The admin origin for the pallet (Tech Committee unanimity).
    type AdminOrigin: EnsureOrigin<Self::Origin>;

    /// Weight information for extrinsics in this pallet.
    type WeightInfo: WeightInfo;
}

decl_storage! {
    trait Store for Module<T: Config> as XXCustody {
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
    }
	add_extra_genesis {
	    config(team_allocations): Vec<(T::AccountId, BalanceOf<T>)>;
		build(|config| {
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
    {

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
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {

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
        #[weight = <T as Config>::WeightInfo::payout()]
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
        #[weight = <T as Config>::WeightInfo::custody_bond()]
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
        #[weight = <T as Config>::WeightInfo::custody_bond_extra()]
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
        #[weight = <T as Config>::WeightInfo::custody_set_controller()]
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
        #[weight = <T as Config>::WeightInfo::custody_set_proxy()]
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
        #[weight = <T as Config>::WeightInfo::team_custody_set_proxy()]
        pub fn team_custody_set_proxy(origin, proxy: T::AccountId) {
            let who = ensure_signed(origin)?;
            ensure!(Self::is_team_member(&who), Error::<T>::InvalidTeamMember);
            Self::try_team_custody_set_proxy(who, proxy)?;
        }

        //----------------    ADMIN     ----------------//

        /// Add a custodian account
        ///
        /// The dispatch origin must be AdminOrigin.
        ///
        #[weight = <T as Config>::WeightInfo::add_custodian()]
        pub fn add_custodian(origin, custodian: T::AccountId) {
            Self::ensure_admin(origin)?;
            <Custodians<T>>::insert(&custodian, ());
            Self::deposit_event(RawEvent::CustodianAdded(custodian));
        }

        /// Remove a custodian account
        ///
        /// The dispatch origin must be AdminOrigin.
        ///
        #[weight = <T as Config>::WeightInfo::remove_custodian()]
        pub fn remove_custodian(origin, custodian: T::AccountId) {
            Self::ensure_admin(origin)?;
            <Custodians<T>>::remove(&custodian);
            Self::deposit_event(RawEvent::CustodianRemoved(custodian));
        }

        /// Replace an existing team member account with a new account
        ///
        /// The dispatch origin must be AdminOrigin.
        ///
        #[weight = <T as Config>::WeightInfo::replace_team_member()]
        pub fn replace_team_member(origin, who: T::AccountId, new: T::AccountId) {
            Self::ensure_admin(origin)?;
            ensure!(Self::is_team_member(&who), Error::<T>::InvalidTeamMember);
            ensure!(!Self::is_team_member(&new), Error::<T>::TeamMemberExists);
            Self::update_team_member(who.clone(), new.clone());
            Self::deposit_event(RawEvent::TeamMemberUpdated(who, new));
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

}

/// Implement CustodyHandler trait
impl<T: Config> pallet_staking::CustodyHandler<T::AccountId, BalanceOf<T>> for Module<T> {
    fn is_custody_account(who: &T::AccountId) -> bool {
        Self::is_custody(who)
    }

    fn total_custody() -> BalanceOf<T> {
        Self::total_custody()
    }
}
