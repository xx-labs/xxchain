// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::{Currency, EnsureOrigin, ExistenceRequirement::AllowDeath, Get};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure,
};
use frame_system::{self as system, ensure_root, ensure_signed};
use sp_core::U256;
use sp_runtime::traits::SaturatedConversion;
use sp_std::prelude::*;
pub use weights::WeightInfo;

pub mod weights;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;

type ResourceId = chainbridge::ResourceId;

type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub trait Config: system::Config + chainbridge::Config {
    /// The Event type
    type RuntimeEvent: From<Event<Self>> + Into<<Self as frame_system::Config>::RuntimeEvent>;

    /// Specifies the origin check provided by the bridge for calls that can only be called by the bridge pallet
    type BridgeOrigin: EnsureOrigin<Self::RuntimeOrigin, Success = Self::AccountId>;

    /// The currency mechanism.
    type Currency: Currency<Self::AccountId>;

    /// Native token ID
    type NativeTokenId: Get<ResourceId>;

    /// Origin used to change fee and destination
    type AdminOrigin: EnsureOrigin<Self::RuntimeOrigin>;

    /// Weight information for extrinsics in this pallet.
    type WeightInfo: WeightInfo;
}

decl_storage! {
    trait Store for Module<T: Config> as Swap {
        /// Swap service fee charged when moving native tokens out of the chain
        pub SwapFee get(fn swap_fee) config(): BalanceOf<T>;

        /// Account to which the fee is paid to
        pub FeeDestination get(fn fee_destination): Option<T::AccountId>;
    }

    add_extra_genesis {
        config(chains): Vec<u8>;
        config(relayers): Vec<T::AccountId>;
        config(resources): Vec<(ResourceId, Vec<u8>)>;
        config(threshold): u32;
        config(balance): BalanceOf<T>;
        config(fee_destination): Option<T::AccountId>;

        build(|config: &GenesisConfig<T>| {
            /*
            Initialize chains, relayers and resources
            Uses expect to panic in the case the values cannot be set. This is reasonable as in that
            case the chain is invalid and should not progress any further.
            */
            <Module<T>>::initialize(&config.chains, &config.relayers, &config.resources, &config.threshold)
                .expect("Could not set config on Chainbridge pallet");
            // Create chainbridge account and set the balance from genesis
            let account_id = <chainbridge::Module<T>>::account_id();
            T::Currency::make_free_balance_be(&account_id, config.balance);
            // Set fee destination
            if let Some(dest) = &config.fee_destination {
                <FeeDestination<T>>::put(dest);
            }
        });
    }
}

decl_event! {
    pub enum Event<T> where Balance = BalanceOf<T>, <T as frame_system::Config>::AccountId {
        /// Swap service fee was changed
        FeeChanged(Balance),
        /// Swap fee destination was changed
        FeeDestinationChanged(AccountId),
    }
}

decl_error! {
    pub enum Error for Module<T: Config> {
        DestinationNotWhitelisted,
        InsufficientBalance,
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::RuntimeOrigin {
        const NativeTokenId: ResourceId = T::NativeTokenId::get();

        fn deposit_event() = default;

        //
        // Initiation calls. These start a bridge transfer.
        //

        /// Transfers an amount of the native token to some recipient on a (whitelisted) destination chain.
        #[weight = <T as Config>::WeightInfo::transfer_native()]
        pub fn transfer_native(origin, amount: BalanceOf<T>, recipient: Vec<u8>, dest_id: chainbridge::ChainId) -> DispatchResult {
            let source = ensure_signed(origin)?;

            // Ensure destination chain is whitelisted
            ensure!(<chainbridge::Module<T>>::chain_whitelisted(dest_id), Error::<T>::DestinationNotWhitelisted);

            // Ensure account has enough balance to pay for both fee and transfer
            let fee = <SwapFee<T>>::get();
            let balance = T::Currency::free_balance(&source);
            ensure!(balance >= amount + fee, Error::<T>::InsufficientBalance);

            // Transfer fee to configured destination (if destination exists)
            if let Some(dest) = <FeeDestination<T>>::get() {
                T::Currency::transfer(&source, &dest, fee, AllowDeath)?;
            };

            // Transfer amount to bridge
            let bridge_id = <chainbridge::Module<T>>::account_id();
            T::Currency::transfer(&source, &bridge_id, amount, AllowDeath)?;

            let resource_id = T::NativeTokenId::get();
            <chainbridge::Module<T>>::transfer_fungible(dest_id, resource_id, recipient,
                U256::from(amount.saturated_into::<u128>()))?;
            Ok(())
        }

        //
        // Executable calls. These can be triggered by a bridge transfer initiated on another chain
        //

        /// Executes a currency transfer from the bridge account
        #[weight = <T as Config>::WeightInfo::transfer()]
        pub fn transfer(origin, to: T::AccountId, amount: BalanceOf<T>) -> DispatchResult {
            let source = T::BridgeOrigin::ensure_origin(origin)?;
            T::Currency::transfer(&source, &to, amount, AllowDeath)?;
            Ok(())
        }

        /// Set swap fee
        #[weight = <T as Config>::WeightInfo::set_swap_fee()]
        pub fn set_swap_fee(origin, #[compact] fee: BalanceOf<T>) -> DispatchResult {
            Self::ensure_admin(origin)?;
            <SwapFee<T>>::put(fee);
            Self::deposit_event(RawEvent::FeeChanged(fee));
            Ok(())
        }

        /// Set fee destination
        #[weight = <T as Config>::WeightInfo::set_fee_destination()]
        pub fn set_fee_destination(origin, dest: T::AccountId) -> DispatchResult {
            Self::ensure_admin(origin)?;
            <FeeDestination<T>>::put(dest.clone());
            Self::deposit_event(RawEvent::FeeDestinationChanged(dest));
            Ok(())
        }
    }
}

impl<T: Config> Module<T> {
    /// Initialize bridge configurations from genesis
    fn initialize(
        chains: &[u8],
        relayers: &[T::AccountId],
        resources: &[(ResourceId, Vec<u8>)],
        threshold: &u32,
    ) -> DispatchResult {
        for c in chains {
            <chainbridge::Module<T>>::whitelist(*c)?;
        }

        for rs in relayers {
            <chainbridge::Module<T>>::register_relayer(rs.clone())?;
        }

        for &(ref re, ref m) in resources.iter() {
            <chainbridge::Module<T>>::register_resource(*re, m.clone())?;
        }

        <chainbridge::Module<T>>::set_relayer_threshold(*threshold)
    }

    fn ensure_admin(o: T::RuntimeOrigin) -> DispatchResult {
        <T as Config>::AdminOrigin::try_origin(o)
            .map(|_| ())
            .or_else(ensure_root)?;
        Ok(())
    }
}
