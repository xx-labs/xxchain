#![cfg_attr(not(feature = "std"), no_std)]

pub mod cmix;
pub mod weights;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;

use frame_support::traits::{EnsureOrigin};
use frame_support::{
    decl_event, decl_error, decl_module, decl_storage, dispatch::DispatchResult, ensure,
    weights::{DispatchClass, Pays},
};

use frame_system::{ensure_root, ensure_signed};
use weights::WeightInfo;
use sp_std::prelude::*;

pub trait Config: frame_system::Config + pallet_staking::Config {
    /// The Event type.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;

    /// The origin that is allowed to modify cmix variables.
    type CmixVariablesOrigin: EnsureOrigin<Self::Origin>;

    /// The admin origin for the pallet (Tech Committee unanimity).
    type AdminOrigin: EnsureOrigin<Self::Origin>;

    /// Weight information for extrinsics in this pallet.
    type WeightInfo: WeightInfo;
}

decl_storage! {
    trait Store for Module<T: Config> as XXCmix {

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
}

decl_event! {
    pub enum Event<T> where
        <T as frame_system::Config>::BlockNumber,
    {

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
        /// AdminOrigin is not allowed to modify cmix hashes
        AdminPermissionExpired,
        /// Must be scheduling server account to call this function
        MustBeScheduling,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {

        type Error = Error<T>;

        fn deposit_event() = default;

        /// Set cmix software hashes
        ///
        /// The dispatch origin must be AdminOrigin.
        /// Furthermore, this call is only allowed if current block is lower than `AdminPermission`.
        ///
        /// # <weight>
        /// - O(1) insert
        /// # </weight>
        #[weight = <T as Config>::WeightInfo::set_cmix_hashes()]
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
        #[weight = <T as Config>::WeightInfo::set_scheduling_account()]
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
        #[weight = <T as Config>::WeightInfo::set_next_cmix_variables()]
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
        /// - DB Weight: n reads and n writes where n is the length of the data vector
        /// # </weight>
        #[weight = (
			<T as Config>::WeightInfo::submit_cmix_points(data.len() as u32),
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
        /// - DB Weight: n reads and n writes where n is the length of the data vector
        /// # </weight>
        #[weight = (
            <T as Config>::WeightInfo::submit_cmix_deductions(data.len() as u32),
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
            <T as Config>::WeightInfo::set_cmix_address_space(),
			DispatchClass::Operational,
			Pays::No
		)]
        pub fn set_cmix_address_space(origin, size: u8) {
            let who = ensure_signed(origin)?;
            ensure!(Self::is_scheduling(who), Error::<T>::MustBeScheduling);
            CmixAddressSpace::put(size);
            Self::deposit_event(RawEvent::CmixAddressSpaceUpdated);
        }

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
        #[weight = <T as Config>::WeightInfo::set_admin_permission()]
        pub fn set_admin_permission(origin, permission: T::BlockNumber) {
            ensure_root(origin)?;
            <AdminPermission<T>>::put(permission);
            Self::deposit_event(RawEvent::AdminPermissionUpdated(permission));
        }
	}
}

impl<T: Config> Module<T> {

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

    /// Add cmix points to staking era rewards
    pub fn reward_cmix_points(data: Vec<(T::AccountId, u32)>) {
        <pallet_staking::Pallet<T>>::reward_by_ids(data)
    }

    /// Deduct cmix points from staking era rewards
    pub fn deduct_cmix_points(data: Vec<(T::AccountId, u32)>) {
        <pallet_staking::Pallet<T>>::deduct_by_ids(data)
    }
}

/// Implement EndEraHandler trait
impl<T: Config> pallet_staking::CmixHandler for Module<T> {

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
