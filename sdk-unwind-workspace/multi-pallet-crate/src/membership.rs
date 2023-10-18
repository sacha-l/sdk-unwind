// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

//! # Shell Pallet
//!
//! A bare bones module that contains the minimum requirements for a FRAME pallet to compile.
//!
//! Jump into the
//! [`pallet` attribute macro docs](https://paritytech.github.io/polkadot-sdk/master/frame_support/attr.pallet.html)
//! to learn about the additional attributes available.
//!
//! Note: this pallet is for learning purposes only and is not meant to be used in production.

// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

use frame_support::pallet_prelude::DispatchResult;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
	// Import various useful types required by all FRAME pallets.
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	// The `Pallet` struct serves as a placeholder to implement traits, methods and calls this
	// pallet exposes.
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	// The pallet's configuration trait.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// This is the max amount of members allowed.
		type MaxMembers: Get<u32>;
	}

	#[pallet::storage]
	pub type Members<T: Config> =
		StorageValue<_, BoundedVec<(T::AccountId, u32, u8), T::MaxMembers>>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Adds a member to the vec in storage.
		pub fn add_member(
			origin: OriginFor<T>,
			member: T::AccountId,
			class_of: u32,
			emoji: u8,
		) -> DispatchResult {
			// check the origin
			ensure_signed(origin)?;

			// write to Members storage
			let _ = Self::do_update_members(member, class_of, emoji);

			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	/// Writes new Member to storage
	pub fn do_update_members(member: T::AccountId, class: u32, emoji: u8) -> DispatchResult {
		let member = (member, class, emoji);

		// write storage update here
		Members::<T>::try_append(member).map_err(|_| "max members exceeded")?;

		Ok(())
	}
}
