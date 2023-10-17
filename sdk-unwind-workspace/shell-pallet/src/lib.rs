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

#[frame_support::pallet]
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
	pub trait Config: frame_system::Config {}
}
