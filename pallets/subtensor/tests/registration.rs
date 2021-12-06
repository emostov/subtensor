use pallet_subtensor::{Error};
use frame_support::{assert_ok};
use frame_system::Config;
mod mock;
use mock::*;
use frame_support::sp_runtime::DispatchError;
use frame_support::dispatch::{GetDispatchInfo, DispatchInfo};
use frame_support::weights::{DispatchClass, Pays};

/********************************************
	subscribing::subscribe() tests
*********************************************/
#[test]
fn test_subscribe_ok_dispatch_info_ok() {
	new_test_ext().execute_with(|| {
		let block_number: u64 = 0;
		let nonce: u64 = 0;
		let work: Vec<u8> = vec![0;32];
		let hotkey: u64 = 0;
		let coldkey: u64 = 0;
        let call = Call::Subtensor(SubtensorCall::register( block_number, nonce, work, hotkey, coldkey ));
		assert_eq!(call.get_dispatch_info(), DispatchInfo {
			weight: 0,
			class: DispatchClass::Normal,
			pays_fee: Pays::No
		});
	});
}

#[test]
fn test_difficulty() {
	new_test_ext().execute_with(|| {
		assert_eq!( Subtensor::get_difficulty().as_u64(), 10000 );
	});

}

#[test]
fn test_registration_ok() {
	new_test_ext().execute_with(|| {
		let block_number: u64 = 0;
		let (nonce, work): (u64, Vec<u8>) = Subtensor::create_work_for_block_number( block_number );
		let hotkey_account_id = 1;
		let coldkey_account_id = 667; // Neighbour of the beast, har har

		// Subscribe and check extrinsic output
		assert_ok!(Subtensor::register(<<Test as Config>::Origin>::signed(hotkey_account_id), block_number, nonce, work, hotkey_account_id, coldkey_account_id));
		let neuron = Subtensor::get_neuron_for_hotkey(&hotkey_account_id);

		// Check uid setting functionality
		assert_eq!(neuron.uid, 0);

		// Check if metadata is set correctly
		assert_eq!(neuron.ip, 0);
		assert_eq!(neuron.ip_type, 0);
		assert_eq!(neuron.port, 0);
		assert_eq!(neuron.coldkey, coldkey_account_id);

		// Check if this function works
		assert_eq!(Subtensor::is_uid_active(neuron.uid), true);

		// Check neuron count increment functionality
        assert_eq!(Subtensor::get_neuron_count(), 1);

		// Check if weights are set correctly. Only self weight
		assert_eq!( Subtensor::get_weights_for_neuron(&neuron), vec![u32::MAX] );

		// Check if the neuron has a hotkey account
		assert_eq!(Subtensor::has_hotkey_account(&neuron.uid), true);

		// Check if the balance of this hotkey account == 0
		assert_eq!(Subtensor::get_stake_of_neuron_hotkey_account_by_uid(neuron.uid), 0);
	});
}

#[test]
fn test_too_many_registrations_per_block() {
	new_test_ext().execute_with(|| {
		
		Subtensor::set_max_registratations_per_block( 10 );

		let block_number: u64 = 0;
		let (nonce0, work0): (u64, Vec<u8>) = Subtensor::create_work_for_block_number( block_number );
		let (nonce1, work1): (u64, Vec<u8>) = Subtensor::create_work_for_block_number( block_number );
		let (nonce2, work2): (u64, Vec<u8>) = Subtensor::create_work_for_block_number( block_number );
		let (nonce3, work3): (u64, Vec<u8>) = Subtensor::create_work_for_block_number( block_number );
		let (nonce4, work4): (u64, Vec<u8>) = Subtensor::create_work_for_block_number( block_number );
		let (nonce5, work5): (u64, Vec<u8>) = Subtensor::create_work_for_block_number( block_number );
		let (nonce6, work6): (u64, Vec<u8>) = Subtensor::create_work_for_block_number( block_number );
		let (nonce7, work7): (u64, Vec<u8>) = Subtensor::create_work_for_block_number( block_number );
		let (nonce8, work8): (u64, Vec<u8>) = Subtensor::create_work_for_block_number( block_number );
		let (nonce9, work9): (u64, Vec<u8>) = Subtensor::create_work_for_block_number( block_number );
		let (nonce10, work10): (u64, Vec<u8>) = Subtensor::create_work_for_block_number( block_number );
		assert_eq!( Subtensor::get_difficulty_as_u64(), 10000 );

		// Subscribe and check extrinsic output
		assert_ok!(Subtensor::register(<<Test as Config>::Origin>::signed(0), block_number, nonce0, work0, 0, 0));
		assert_ok!(Subtensor::register(<<Test as Config>::Origin>::signed(1),  block_number, nonce1, work1, 1, 1));
		assert_ok!(Subtensor::register(<<Test as Config>::Origin>::signed(2),  block_number, nonce2, work2, 2, 2));
		assert_ok!(Subtensor::register(<<Test as Config>::Origin>::signed(3),  block_number, nonce3, work3, 3, 3));
		assert_ok!(Subtensor::register(<<Test as Config>::Origin>::signed(4),  block_number, nonce4, work4, 4, 4));
		assert_ok!(Subtensor::register(<<Test as Config>::Origin>::signed(5),  block_number, nonce5, work5, 5, 5));
		assert_ok!(Subtensor::register(<<Test as Config>::Origin>::signed(6),  block_number, nonce6, work6, 6, 6));
		assert_ok!(Subtensor::register(<<Test as Config>::Origin>::signed(7),  block_number, nonce7, work7, 7, 7));
		assert_ok!(Subtensor::register(<<Test as Config>::Origin>::signed(8),  block_number, nonce8, work8, 8, 8));
		assert_ok!(Subtensor::register(<<Test as Config>::Origin>::signed(9),  block_number, nonce9, work9, 9, 9));
		let result = Subtensor::register(<<Test as Config>::Origin>::signed(10), block_number, nonce10, work10, 10, 10);
		assert_eq!( result, Err(Error::<Test>::ToManyRegistrationsThisBlock.into()) );
	});
}

#[test]
fn test_defaults() {
	new_test_ext().execute_with(|| {
		assert_eq!( Subtensor::get_difficulty_as_u64(), 10000 );
		assert_eq!( Subtensor::get_target_registrations_per_interval(), 2 );
		assert_eq!( Subtensor::get_adjustment_interval(), 100 );
		assert_eq!( Subtensor::get_max_registratations_per_block(), 2 );
		step_block ( 1 );
		assert_eq!( Subtensor::get_difficulty_as_u64(), 10000 );
		assert_eq!( Subtensor::get_target_registrations_per_interval(), 2 );
		assert_eq!( Subtensor::get_adjustment_interval(), 100 );
		assert_eq!( Subtensor::get_max_registratations_per_block(), 2 );
		Subtensor::set_adjustment_interval( 2 );
		Subtensor::set_target_registrations_per_interval( 2 );
		Subtensor::set_difficulty_from_u64( 2 );
		Subtensor::set_max_registratations_per_block( 2 );
		assert_eq!( Subtensor::get_difficulty_as_u64(), 2 );
		assert_eq!( Subtensor::get_target_registrations_per_interval(), 2 );
		assert_eq!( Subtensor::get_adjustment_interval(), 2 );
		assert_eq!( Subtensor::get_max_registratations_per_block(), 2 );
	});
}

#[test]
fn test_difficulty_adjustment() {
	new_test_ext().execute_with(|| {
		Subtensor::set_adjustment_interval( 1 );
		Subtensor::set_target_registrations_per_interval( 1 );
		Subtensor::set_difficulty_from_u64( 1 );
		assert_eq!( Subtensor::get_difficulty_as_u64(), 1 );
		assert_eq!( Subtensor::get_target_registrations_per_interval(), 1 );
		assert_eq!( Subtensor::get_adjustment_interval(), 1 );
		assert_eq!( Subtensor::get_max_registratations_per_block(), 2 );

		let (nonce0, work0): (u64, Vec<u8>) = Subtensor::create_work_for_block_number( 0 );
		let (nonce1, work1): (u64, Vec<u8>) = Subtensor::create_work_for_block_number( 0 );
		assert_ok!(Subtensor::register(<<Test as Config>::Origin>::signed(0), 0, nonce0, work0, 0, 0));
		assert_ok!(Subtensor::register(<<Test as Config>::Origin>::signed(1), 0, nonce1, work1, 1, 1));
		assert_eq!( Subtensor::get_registrations_this_interval(), 2 );
		assert_eq!( Subtensor::get_registrations_this_block(), 2 );

		step_block ( 1 );
		assert_eq!( Subtensor::get_difficulty_as_u64(), 2 );
		step_block ( 1 );
		assert_eq!( Subtensor::get_difficulty_as_u64(), 10000 );
		let (nonce2, work2): (u64, Vec<u8>) = Subtensor::create_work_for_block_number( 2 );
		let (nonce3, work3): (u64, Vec<u8>) = Subtensor::create_work_for_block_number( 2 );
		assert_ok!(Subtensor::register(<<Test as Config>::Origin>::signed(2), 2, nonce2, work2, 2, 2));
		assert_ok!(Subtensor::register(<<Test as Config>::Origin>::signed(3), 2, nonce3, work3, 3, 3));
		step_block ( 1 );
		assert_eq!( Subtensor::get_difficulty_as_u64(), 20000 );
		let (nonce4, work4): (u64, Vec<u8>) = Subtensor::create_work_for_block_number( 3 );
		let (nonce5, work5): (u64, Vec<u8>) = Subtensor::create_work_for_block_number( 3 );
		assert_ok!(Subtensor::register(<<Test as Config>::Origin>::signed(4), 3, nonce4, work4, 4, 4));
		assert_ok!(Subtensor::register(<<Test as Config>::Origin>::signed(5), 3, nonce5, work5, 5, 5));
		step_block ( 1 );
		assert_eq!( Subtensor::get_difficulty_as_u64(), 40000 );
		step_block ( 1 );
		assert_eq!( Subtensor::get_difficulty_as_u64(), 20000 );
		step_block ( 1 );
		assert_eq!( Subtensor::get_difficulty_as_u64(), 10000 );
		step_block ( 1 );
		assert_eq!( Subtensor::get_difficulty_as_u64(), 10000 );
		step_block ( 1 );
		assert_eq!( Subtensor::get_difficulty_as_u64(), 10000 );

	});
}

#[test]
fn test_immunity_period() {
	new_test_ext().execute_with(|| {
		Subtensor::set_max_allowed_uids ( 2 );
		Subtensor::set_immunity_period ( 2 );
		assert_eq!( Subtensor::get_max_allowed_uids(), 2 );
		assert_eq!( Subtensor::get_immunity_period(), 2 );

		// Register two neurons into the first two slots.
		let neuron0 = register_ok_neuron( 0, 0 );
		assert_eq!( neuron0.uid, 0 );
		let neuron1 = register_ok_neuron( 1, 1 );
		assert_eq!( neuron1.uid, 1 );
		assert!( !Subtensor::will_be_prunned(0) );
		assert!( !Subtensor::will_be_prunned(1) );

		// Step to the next block.
		step_block ( 1 );

		// Register the next neuron, this causes the overflow over top of the max allowed.
		// Because both previous are immune, we will take the first uid to be prunned.
		let neuron2 = register_ok_neuron( 2, 2 );
		assert_eq!( neuron2.uid, 0 );

		// Register the next neuron, this causes the overflow over top of the max allowed.
		// Because uid0 is owned by a uid with a larger registration block number the uid to
		// prune is now 0. All uids are immune at this stage.
		let neuron3 = register_ok_neuron( 3, 3 );
		assert_eq!( neuron3.uid, 1 );
		assert!( Subtensor::will_be_prunned(0) );
		assert!( Subtensor::will_be_prunned(1) );

		// Step to the next block.
		Subtensor::set_stake_from_vector( vec![ 1, 0 ] );
		assert_eq!( Subtensor::get_stake(), vec![ 1, 0 ] );
		step_block ( 1 );

		// Register the next neuron, the previous neurons have immunity however the first has stake.
		let neuron4 = register_ok_neuron( 4, 4 );
		assert_eq!( neuron4.uid, 1 );

		// Register the next neuron, the first neuron still has stake but he was registed a block earlier. 
		// than neuron4, we go into slot 0
		let neuron5 = register_ok_neuron( 5, 5 );
		assert_eq!( neuron5.uid, 0 );
		assert!( Subtensor::will_be_prunned(0) );
		assert!( Subtensor::will_be_prunned(1) );

		Subtensor::set_stake_from_vector( vec![ 1, 0 ] );
		step_block ( 1 );
		step_block ( 1 );
		step_block ( 1 );

		// Register the next neuron, the first slot has stake go into slot 1
		let neuron6 = register_ok_neuron( 6, 6 );
		assert_eq!( neuron6.uid, 1 );
		assert!( !Subtensor::will_be_prunned(0) );
		assert!( Subtensor::will_be_prunned(1) );

		step_block ( 1 );
		// Prunned set is dropped.
		assert!( !Subtensor::will_be_prunned(0) );
		assert!( !Subtensor::will_be_prunned(1) );
		step_block ( 1 );
		step_block ( 1 );

		// Register the next neuron, the first slot has stake and both are no longer immune
		// so this goes into slot 1 again.
		let neuron7 = register_ok_neuron( 7, 7 );
		assert_eq!( neuron7.uid, 1 );
		assert!( !Subtensor::will_be_prunned(0) );
		assert!( Subtensor::will_be_prunned(1) );

	});
}

#[test]
fn test_already_active_hotkey() {
	new_test_ext().execute_with(|| {

		let block_number: u64 = 0;
		let (nonce, work): (u64, Vec<u8>) = Subtensor::create_work_for_block_number( block_number );
		let hotkey_account_id = 1;
		let coldkey_account_id = 667;

		assert_ok!(Subtensor::register(<<Test as Config>::Origin>::signed(hotkey_account_id), block_number, nonce, work, hotkey_account_id, coldkey_account_id));

		let block_number: u64 = 0;
		let (nonce, work): (u64, Vec<u8>) = Subtensor::create_work_for_block_number( block_number );
		let hotkey_account_id = 1;
		let coldkey_account_id = 667;
		let result = Subtensor::register(<<Test as Config>::Origin>::signed(hotkey_account_id), block_number, nonce, work, hotkey_account_id, coldkey_account_id);
		assert_eq!( result, Err(Error::<Test>::AlreadyRegistered.into()) );
	});
}


#[test]
fn test_invalid_seal() {
	new_test_ext().execute_with(|| {
		let block_number: u64 = 0;
		let (nonce, work): (u64, Vec<u8>) = Subtensor::create_work_for_block_number( 1 );
		let hotkey_account_id = 1;
		let coldkey_account_id = 667;
		let result = Subtensor::register(<<Test as Config>::Origin>::signed(hotkey_account_id), block_number, nonce, work, hotkey_account_id, coldkey_account_id);
		assert_eq!( result, Err(Error::<Test>::InvalidSeal.into()) );
	});
}

#[test]
fn test_invalid_block_number() {
	new_test_ext().execute_with(|| {
		let block_number: u64 = 1;
		let (nonce, work): (u64, Vec<u8>) = Subtensor::create_work_for_block_number( block_number );
		let hotkey_account_id = 1;
		let coldkey_account_id = 667;
		let result = Subtensor::register(<<Test as Config>::Origin>::signed(hotkey_account_id), block_number, nonce, work, hotkey_account_id, coldkey_account_id);
		assert_eq!( result, Err(Error::<Test>::InvalidWorkBlock.into()) );
	});
}

#[test]
fn test_invalid_difficulty() {
	new_test_ext().execute_with(|| {
		let block_number: u64 = 0;
		let (nonce, work): (u64, Vec<u8>) = Subtensor::create_work_for_block_number( block_number );
		let hotkey_account_id = 1;
		let coldkey_account_id = 667;
		Subtensor::set_difficulty_from_u64( 18_446_744_073_709_551_615u64 );
		let result = Subtensor::register(<<Test as Config>::Origin>::signed(hotkey_account_id), block_number, nonce, work, hotkey_account_id, coldkey_account_id);
		assert_eq!( result, Err(Error::<Test>::InvalidDifficulty.into()) );
	});
}

#[test]
fn test_register_failed_no_signature() {
	new_test_ext().execute_with(|| {

		let block_number: u64 = 1;
		let (nonce, work): (u64, Vec<u8>) = Subtensor::create_work_for_block_number( block_number );
		let hotkey_account_id = 1;
		let coldkey_account_id = 667; // Neighbour of the beast, har har

		// Subscribe and check extrinsic output
		let result = Subtensor::register(<<Test as Config>::Origin>::none(), block_number, nonce, work, hotkey_account_id, coldkey_account_id);
		assert_eq!(result, Err(DispatchError::BadOrigin.into()));
	});
}

/********************************************
	subscribing::get_next_uid() tests
*********************************************/
#[test]
fn test_get_next_uid() {
	new_test_ext().execute_with(|| {
        assert_eq!(Subtensor::get_next_uid(), 0); // We start with id 0
		assert_eq!(Subtensor::get_next_uid(), 1); // One up
		assert_eq!(Subtensor::get_next_uid(), 2) // One more
	});
}

