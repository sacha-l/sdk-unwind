// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

//! Mock runtime environment and tests for the Super Basic pallet.

use crate::*;
use frame_support::{assert_ok, traits::ConstU64};
use sp_core::H256;
// The testing primitives are very useful for avoiding having to work with signatures
// or public keys. `u64` is used as the `AccountId` and no `Signature`s are required.
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};
// Reexport crate as its pallet name for construct_runtime.
use crate as pallet_example_super_basic;

type Block = frame_system::mocking::MockBlock<Test>;

// For testing the pallet, we construct a mock runtime.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		Example: pallet_example_super_basic::{Pallet, Call, Storage, Config<T>},
	}
);

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type Nonce = u64;
	type Hash = H256;
	type RuntimeCall = RuntimeCall;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = u64;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ConstU64<1>;
	type AccountStore = System;
	type WeightInfo = ();
	type FreezeIdentifier = ();
	type MaxFreezes = ();
	type RuntimeHoldReason = ();
	type MaxHolds = ();
}

// Implement the pallet's configuration trait for the mock runtime.
impl Config for Test {}

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let t = RuntimeGenesisConfig {
		// We use default for brevity, but you can configure as desired if needed.
		system: Default::default(),
		balances: Default::default(),
		example: pallet_example_super_basic::GenesisConfig { dummy: 42 },
	}
	.build_storage()
	.unwrap();
	t.into()
}

#[test]
fn genesis_storage_works() {
	new_test_ext().execute_with(|| {
		// check that our genesis build worked
		assert_eq!(Example::dummy(), Some(42));
	});
}

#[test]
fn accumulate_dummy_works() {
	new_test_ext().execute_with(|| {
		let genesis_val = 42;
		let increment_by = 100;

		// check that accumulate works when we have Some value in Dummy already
		assert_ok!(Example::accumulate_dummy(RuntimeOrigin::signed(1), increment_by));
		assert_eq!(Example::dummy(), Some(genesis_val + increment_by));
	});
}

#[test]
fn kill_dummy_works() {
	new_test_ext().execute_with(|| {
		// we know there is the value Som(42) in storage
		assert_eq!(Example::dummy(), Some(42));
		// construct and dispatch the call with root origin
		assert_ok!(Example::kill_dummy(RuntimeOrigin::root()));
		assert_eq!(Example::dummy(), None);
	});
}
