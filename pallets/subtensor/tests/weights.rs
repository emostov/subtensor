// mod mock;
// use mock::*;
// use frame_system::{Config};
// use pallet_subtensor::{Error};
// use frame_support::weights::{GetDispatchInfo, DispatchInfo, DispatchClass, Pays};
// use frame_support::{assert_ok};
// use sp_runtime::DispatchError;


// /***************************
//   pub fn set_weights() tests
// *****************************/

// // This does not produce the expected result
// #[test]
// fn test_set_weights_dispatch_info_ok() {
// 	new_test_ext().execute_with(|| {
// 		let w_uids = vec![1, 1];
// 		let w_vals = vec![1, 1];

// 		let call = Call::Subtensor(SubtensorCall::set_weights(w_uids, w_vals));

// 		assert_eq!(call.get_dispatch_info(), DispatchInfo {
// 			weight: 0,
// 			class: DispatchClass::Normal,
// 			pays_fee: Pays::No
// 		});
// 	});
// }


// /**
// * This test the situation where user tries to set weights, but the vecs are empty.
// * After setting the weights, the wi
// */
// #[test]
// fn set_weights_ok_no_weights() {
// 	new_test_ext().execute_with(|| {

// 		// == Intial values ==
// 		let hotkey_account_id:u64 = 55; // Arbitrary number
// 		let initial_stake = 10000;

// 		let weights_keys : Vec<u32> = vec![];
// 		let weight_values : Vec<u32> = vec![];

// 		// == Expectations ==
// 		let expect_stake:u64 = 10000; // The stake for the neuron should remain the same
// 		let expect_total_stake:u64 = 10000; // The total stake should remain the same

// 		// Let's subscribe a new neuron to the chain
// 		assert_ok!(Subtensor::set_registeration_key(<<Test as Config>::Origin>::root(), 0));
// 		let neuron = register_ok_neuron(0, 0, hotkey_account_id, 66);

// 		// Let's give it some stake.
// 		Subtensor::add_stake_to_neuron_hotkey_account(neuron.uid, initial_stake);

// 		// Dispatch a signed extrinsic, setting weights.
// 		assert_ok!(Subtensor::set_weights(Origin::signed(hotkey_account_id), weights_keys, weight_values));
// 		assert_eq!(Subtensor::get_weights_for_neuron(&neuron), vec![u32::max_value()]);
// 		assert_eq!(Subtensor::get_stake_of_neuron_hotkey_account_by_uid(neuron.uid), expect_stake);
// 		assert_eq!(Subtensor::get_total_stake(), expect_total_stake);
// 	});
// }

// #[test]
// fn test_weights_err_weights_vec_not_equal_size() {
// 	new_test_ext().execute_with(|| {
// 		assert_ok!(Subtensor::set_registeration_key(<<Test as Config>::Origin>::root(), 0));
//     	let _neuron = register_ok_neuron(0, 0, 666, 77);

// 		let weights_keys: Vec<u32> = vec![1, 2, 3, 4, 5, 6];
// 		let weight_values: Vec<u32> = vec![1, 2, 3, 4, 5]; // Uneven sizes

// 		let result = Subtensor::set_weights(Origin::signed(666), weights_keys, weight_values);

// 		assert_eq!(result, Err(Error::<Test>::WeightVecNotEqualSize.into()));
// 	});
// }

// #[test]
// fn test_weights_err_has_duplicate_ids() {
// 	new_test_ext().execute_with(|| {
// 		assert_ok!(Subtensor::set_registeration_key(<<Test as Config>::Origin>::root(), 0));
//     	let _neuron = register_ok_neuron(0, 0, 666, 77);
// 		let weights_keys: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 6, 6]; // Contains duplicates
// 		let weight_values: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8];

// 		let result = Subtensor::set_weights(Origin::signed(666), weights_keys, weight_values);

// 		assert_eq!(result, Err(Error::<Test>::DuplicateUids.into()));
// 	});
// }

// #[test]
// fn test_no_signature() {
// 	new_test_ext().execute_with(|| {
// 		let weights_keys: Vec<u32> = vec![];
// 		let weight_values: Vec<u32> = vec![];

// 		let result = Subtensor::set_weights(Origin::none(), weights_keys, weight_values);
// 		assert_eq!(result, Err(DispatchError::BadOrigin.into()));
// 	});
// }

// #[test]
// fn test_set_weights_err_not_active() {
// 	new_test_ext().execute_with(|| {
// 		let weights_keys: Vec<u32> = vec![1, 2, 3, 4, 5, 6];
// 		let weight_values: Vec<u32> = vec![1, 2, 3, 4, 5, 6];

// 		let result = Subtensor::set_weights(Origin::signed(1), weights_keys, weight_values);

// 		assert_eq!(result, Err(Error::<Test>::NotRegistered.into()));
// 	});
// }


// #[test]
// fn test_set_weights_err_invalid_uid() {
// 	new_test_ext().execute_with(|| {
// 		assert_ok!(Subtensor::set_registeration_key(<<Test as Config>::Origin>::root(), 0));

//         let _neuron = register_ok_neuron(0, 0, 55, 66);
// 		let weight_keys : Vec<u32> = vec![99999]; // Does not exist
// 		let weight_values : Vec<u32> = vec![88]; // random value

// 		let result = Subtensor::set_weights(Origin::signed(55), weight_keys, weight_values);

// 		assert_eq!(result, Err(Error::<Test>::InvalidUid.into()));

// 	});
// }



