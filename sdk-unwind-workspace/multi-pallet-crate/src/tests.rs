use super::*;
use crate as multi_pallet_crate;

	//! Mock runtime environment and tests for multi-pallet-crate.
	//! 
	//! TODO: We should alter the pallets such that they work together so it makes
	//! sense to have both in the same Mock runtime environment.
	use crate::{
        dummy_pallet::pallet::{self as pallet_dummy},
        membership::pallet::{self as pallet_membership}
    };
	use frame_support::{assert_ok, traits::ConstU64};
	use sp_core::H256;
    use sp_core::ConstU32;

	// The testing primitives are very useful for avoiding having to work with signatures
	// or public keys. `u64` is used as the `AccountId` and no `Signature`s are required.
	use sp_runtime::{
		traits::{BlakeTwo256, IdentityLookup},
		BuildStorage,
	};

	type Block = frame_system::mocking::MockBlock<Runtime>;

	// For testing the pallet, we construct a mock runtime.
	frame_support::construct_runtime!(
		pub enum Runtime
		{
			System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
            Dummy: pallet_dummy,
            Members: pallet_membership,
		}
	);

	impl frame_system::Config for Runtime {
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
		type AccountData = ();
		type OnNewAccount = ();
		type OnKilledAccount = ();
		type SystemWeightInfo = ();
		type SS58Prefix = ();
		type OnSetCode = ();
		type MaxConsumers = frame_support::traits::ConstU32<16>;
	}

	// Implement the pallet's configuration trait for the mock runtime.
	impl pallet_dummy::Config for Runtime {}
    
    impl pallet_membership::Config for Runtime {
        type MaxMembers = ConstU32<100>;
    }

	// TODO: write testing set-up