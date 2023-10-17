//! # Super basic Pallet
//!
//! A pallet in `dev_mode` with minimal functionality to demonstrate basic patterns for writing a
//! FRAME pallet.
//!
//! These include:
//! - Different pallet parts such as: the `Config` trait, storage, genesis config, genesis build,
//!   and calls.
//! - Using a mock runtime to write unit tests for the callable logic a pallet exposes.

// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

// Used for safe math.
use sp_arithmetic::traits::Saturating;

/// ** TESTING MODULE **
#[cfg(test)]
mod tests;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
	// Import various useful types required by all FRAME pallets.
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	// The `Pallet` struct serves as a placeholder to implement traits, methods and dispatchables in
	// this pallet.
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// ** PALLET CONFIGURATION TRAIT **
	#[pallet::config]
	pub trait Config: pallet_balances::Config + frame_system::Config {}

	/// ** STORAGE **
	/// A single value storage item that stores some Balance.
	#[pallet::storage]
	#[pallet::getter(fn dummy)]
	pub(super) type Dummy<T: Config> = StorageValue<_, T::Balance>;

	/// ** GENESIS **
	// A way to configure the genesis state of your pallet.
	#[pallet::genesis_config]
	#[derive(frame_support::DefaultNoBound)]
	pub struct GenesisConfig<T: Config> {
		pub dummy: T::Balance,
	}

	// A way to build the genesis state of your pallet.
	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			Dummy::<T>::put(&self.dummy);
		}
	}

	/// ** PALLET CALL BLOCK **
	/// Your pallet's callable functions go here.
	/// Note: because we are using dev_mode, we don't need to care about weights or call indices.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		pub fn accumulate_dummy(origin: OriginFor<T>, increase_by: T::Balance) -> DispatchResult {
			// check that the caller is signed
			let _sender = ensure_signed(origin)?;

			// // demo of None value in storage, the default value for the storage item
			// let current_value_in_dummy = Self::dummy();
			// assert_eq!(current_value_in_dummy, None);

			Dummy::<T>::mutate(|dummy| {
				// Using `saturating_add` instead of a regular `+` to avoid overflowing
				let new_dummy = dummy.map_or(increase_by, |d| d.saturating_add(increase_by));
				*dummy = Some(new_dummy);
			});

			Ok(())
		}

		/// A function that deletes the storage of Dummy, that can only be called by some root
		/// origin.
		pub fn kill_dummy(origin: OriginFor<T>) -> DispatchResult {
			let _caller = ensure_root(origin)?;

			Dummy::<T>::kill();

			Ok(())
		}
	}
}
