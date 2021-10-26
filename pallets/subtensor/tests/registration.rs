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
// #[test]
// fn test_subscribe_ok_dispatch_info_ok() {
// 	new_test_ext().execute_with(|| {
// 		let email_hash: Vec<u8> = vec![0;32];
// 		let hotkey: u64 = 0;
// 		let coldkey: u64 = 0;
//         let call = Call::Subtensor(SubtensorCall::register( email_hash, hotkey, coldkey ));
// 		assert_eq!(call.get_dispatch_info(), DispatchInfo {
// 			weight: 0,
// 			class: DispatchClass::Normal,
// 			pays_fee: Pays::No
// 		});
// 	});
// }

// #[test]
// fn test_registration_ok() {
// 	new_test_ext().execute_with(|| {
// 		let registration_id = 0;
// 		let hotkey_account_id = 1;
// 		let coldkey_account_id = 667; // Neighbour of the beast, har har

// 		// Subscribe and check extrinsic output
// 		assert_ok!(Subtensor::register(<<Test as Config>::Origin>::signed(registration_id), hotkey_account_id, coldkey_account_id));
// 		let neuron = Subtensor::get_neuron_for_hotkey(&hotkey_account_id);

// 		// Check uid setting functionality
// 		assert_eq!(neuron.uid, 0);

// 		// Check if metadata is set correctly
// 		assert_eq!(neuron.ip, 0);
// 		assert_eq!(neuron.ip_type, 0);
// 		assert_eq!(neuron.port, 0);
// 		assert_eq!(neuron.coldkey, coldkey_account_id);

// 		// Check if this function works
// 		assert_eq!(Subtensor::is_uid_active(neuron.uid), true);

// 		// Check neuron count increment functionality
//         assert_eq!(Subtensor::get_neuron_count(), 1);

// 		// Check if weights are set correctly. Only self weight
// 		assert_eq!( Subtensor::get_weights_for_neuron(&neuron), vec![u32::MAX] );

// 		// Check if the neuron has a hotkey account
// 		assert_eq!(Subtensor::has_hotkey_account(&neuron.uid), true);

// 		// Check if the balance of this hotkey account == 0
// 		assert_eq!(Subtensor::get_stake_of_neuron_hotkey_account_by_uid(neuron.uid), 0);
// 	});
// }

// #[test]
// fn test_already_active_hotkey() {
// 	new_test_ext().execute_with(|| {

// 		let registration_id = 0;
// 		let hotkey_account_id = 1;
// 		let coldkey_account_id = 667; // Neighbour of the beast, har har
// 		let email_hash_1: Vec<u8> = vec![0;32]; // different emails
// 		let email_hash_2: Vec<u8> = vec![1;32];

// 		// This line links the hotkey to the coldkey on first subscription
// 		assert_ok!(Subtensor::set_registeration_key(<<Test as Config>::Origin>::root(), registration_id));
// 		let result = Subtensor::register(<<Test as Config>::Origin>::signed(registration_id), email_hash_1, hotkey_account_id, coldkey_account_id );
// 		assert_ok!( result );

// 		let result = Subtensor::register(<<Test as Config>::Origin>::signed(registration_id), email_hash_2, hotkey_account_id, coldkey_account_id );
// 		assert_eq!( result, Err(Error::<Test>::AlreadyRegistered.into()) );
// 	});
// }

// #[test]
// fn test_max_registrations_per_email_reached() {
// 	new_test_ext().execute_with(|| {
// 		let registration_id = 0;
// 		assert_ok!(Subtensor::set_registeration_key(<<Test as Config>::Origin>::root(), registration_id));
// 		for i in 0..Subtensor::get_max_registrations_per_email()+1 {
// 			let hotkey_account_id = i as u64;
// 			let coldkey_account_id = i as u64;
// 			let email_hash: Vec<u8> = vec![0;32];
// 			let result = Subtensor::register(<<Test as Config>::Origin>::signed(registration_id), email_hash, hotkey_account_id, coldkey_account_id );
// 			assert_ok!( result );
// 		}
// 		let email_hash: Vec<u8> = vec![0;32];
// 		let hotkey_account_id = (Subtensor::get_max_registrations_per_email() + 1) as u64;
// 		let coldkey_account_id = (Subtensor::get_max_registrations_per_email() + 1)as u64;
// 		let result = Subtensor::register(<<Test as Config>::Origin>::signed(registration_id), email_hash, hotkey_account_id, coldkey_account_id );
// 		assert_eq!( result, Err(Error::<Test>::MaxRegistrationsReached.into()) );
// 	});
// }

// #[test]
// fn test_register_failed_no_signature() {
// 	new_test_ext().execute_with(|| {
// 		let registration_id = 0;
// 		assert_ok!(Subtensor::set_registeration_key(<<Test as Config>::Origin>::root(), registration_id));

// 		let hotkey_account_id = 1;
// 		let coldkey_account_id = 667; // Neighbour of the beast, har har
// 		let email_hash: Vec<u8> = vec![0;32];

// 		// Subscribe and check extrinsic output
// 		let result = Subtensor::register(<<Test as Config>::Origin>::none(), email_hash, hotkey_account_id, coldkey_account_id);
// 		assert_eq!(result, Err(DispatchError::BadOrigin.into()));
// 	});
// }

// #[test]
// fn test_register_invalid_email_hash() {
// 	new_test_ext().execute_with(|| {
// 		let registration_id = 0;
// 		assert_ok!(Subtensor::set_registeration_key(<<Test as Config>::Origin>::root(), registration_id));

// 		let hotkey_account_id = 1;
// 		let coldkey_account_id = 667; // Neighbour of the beast, har har
// 		let email_hash: Vec<u8> = vec![0;33];
// 		// Subscribe and check extrinsic output
// 		let result = Subtensor::register(<<Test as Config>::Origin>::signed(registration_id), email_hash, hotkey_account_id, coldkey_account_id );
// 		assert_eq!(result, Err(Error::<Test>::InvalidEmailHash.into()));
// 	});
// }

// /********************************************
// 	subscribing::get_next_uid() tests
// *********************************************/
// #[test]
// fn test_get_next_uid() {
// 	new_test_ext().execute_with(|| {
//         assert_eq!(Subtensor::get_next_uid(), 0); // We start with id 0
// 		assert_eq!(Subtensor::get_next_uid(), 1); // One up
// 		assert_eq!(Subtensor::get_next_uid(), 2) // One more
// 	});
// }

