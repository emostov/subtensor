// use pallet_subtensor::{ChargeTransactionPayment, Error, CallType};
// use frame_support::{assert_ok};

// mod mock;

// use mock::*;
// use frame_system::Config;
// use frame_support::weights::{DispatchInfo, Pays};
// use frame_support::weights::PostDispatchInfo;
// use sp_std::marker::PhantomData;
// use sp_runtime::traits::SignedExtension;
// use sp_runtime::transaction_validity::{InvalidTransaction, ValidTransaction};
// use frame_support::dispatch::GetDispatchInfo;

// #[test]
// fn fee_from_emission_works() {
//     new_test_ext().execute_with(|| {
//         let call = SubtensorCall::set_weights(vec![0], vec![0]).into();
//         let info = DispatchInfo::default();
//         let len = 10;
//         assert!(ChargeTransactionPayment::<Test>(PhantomData).validate(&1, &call, &info, len).is_ok());
//     });
// }

// #[test]
// fn fee_from_emission_priority_no_neuron() {
//     new_test_ext().execute_with(|| {
//         let call = SubtensorCall::set_weights(vec![0], vec![0]).into();
//         let info = DispatchInfo::default();
//         let len = 10;
//         assert_eq!(ChargeTransactionPayment::<Test>(PhantomData).validate(&1, &call, &info, len).unwrap().priority, 0);
//     });
// }

// #[test]
// fn fee_from_emission_priority_with_neuron() {
//     new_test_ext().execute_with(|| {

//         // Register a neuron
//         let registration_id = 0;
// 		let hotkey_account_id = 1;
// 		let coldkey_account_id = 667; // Neighbour of the beast, har har
// 		let email_hash: Vec<u8> = vec![0;32];
// 		assert_ok!(Subtensor::set_registeration_key(<<Test as Config>::Origin>::root(), registration_id));
// 		assert_ok!(Subtensor::register(<<Test as Config>::Origin>::signed(registration_id), email_hash, hotkey_account_id, coldkey_account_id));

//         // Registered neuron has zero priority because they have no stake.
//         let call = SubtensorCall::set_weights(vec![0], vec![0]).into();
//         let info = DispatchInfo::default();
//         let len = 10;
//         assert_eq!(ChargeTransactionPayment::<Test>(PhantomData).validate(&hotkey_account_id, &call, &info, len).unwrap().priority, 0);

//         step_block (1);

//         // Priority has not accumulates based on self-emission. But has no stake, thus still zero.
//         let call = SubtensorCall::set_weights(vec![0], vec![0]).into();
//         let info = DispatchInfo::default();
//         let len = 10;
//         assert_eq!(ChargeTransactionPayment::<Test>(PhantomData).validate(&hotkey_account_id, &call, &info, len).unwrap().priority, 0);
//     });
// }

// #[test]
// fn fee_from_emission_priority_with_neuron_and_weights_and_stake() {
//     new_test_ext().execute_with(|| {

//         // Register a neuron
//         let registration_id = 0;
// 		let hotkey_account_id = 1;
// 		let coldkey_account_id = 667; // Neighbour of the beast, har har
// 		let email_hash: Vec<u8> = vec![0;32];
// 		assert_ok!(Subtensor::set_registeration_key(<<Test as Config>::Origin>::root(), registration_id));
// 		assert_ok!(Subtensor::register(<<Test as Config>::Origin>::signed(registration_id), email_hash, hotkey_account_id, coldkey_account_id));
//         Subtensor::add_stake_to_neuron_hotkey_account(0, 1_000_000_000); // Add the stake.

//         // Registered neuron has zero priority because they have no stake.
//         let call = SubtensorCall::set_weights(vec![0], vec![0]).into();
//         let info = DispatchInfo::default();
//         let len = 10;
//         assert_eq!(ChargeTransactionPayment::<Test>(PhantomData).validate(&hotkey_account_id, &call, &info, len).unwrap().priority, 0);

//         step_block (1);

//         // Priority has not accumulates based on self-emission. But has no stake, thus still zero.
//         let call = SubtensorCall::set_weights(vec![0], vec![0]).into();
//         let info = DispatchInfo::default();
//         let len = 10;
//         assert_eq!(ChargeTransactionPayment::<Test>(PhantomData).validate(&hotkey_account_id, &call, &info, len).unwrap().priority, 100000000);
//     });
// }

// /************************************************************
// 	ChargeTransactionPayment::get_priority_vanilla() tests
// ************************************************************/

// #[test]
// fn test_charge_transaction_payment_get_priority_vanilla() {
//     new_test_ext().execute_with(|| {
//         assert_eq!(ChargeTransactionPayment::<Test>::get_priority_vanilla(), u64::max_value());
//     });
// }


// /************************************************************
// 	ChargeTransactionPayment::validate() tests
// ************************************************************/

// #[test]
// fn test_charge_transaction_payment_validate_set_weights_ok() {
//     new_test_ext().execute_with(|| {
//         // Register a neuron
//         let registration_id = 0;
//         let hotkey_account_id = 1;
//         let coldkey_account_id = 667; // Neighbour of the beast, har har
//         let email_hash: Vec<u8> = vec![0;32];
//         assert_ok!(Subtensor::set_registeration_key(<<Test as Config>::Origin>::root(), registration_id));
//         assert_ok!(Subtensor::register(<<Test as Config>::Origin>::signed(registration_id), email_hash, hotkey_account_id, coldkey_account_id));
//         Subtensor::add_stake_to_neuron_hotkey_account(0, 1_000_000_000); // Add the stake.

//         let call: mock::Call = SubtensorCall::set_weights(vec![0], vec![0]).into();
//         let info = call.get_dispatch_info();

//         let result = ChargeTransactionPayment::<Test>(PhantomData).validate(&hotkey_account_id, &call, &info, 10);
//         assert_eq!(result, Ok(ValidTransaction {
//             priority: 0,
//             longevity: 1,
//             ..Default::default()
//         }))
//     });
// }

// #[test]
// fn test_charge_transaction_payment_validate_add_stake_ok() {
//     new_test_ext().execute_with(|| {
//         // Register a neuron
//         let registration_id = 0;
//         let hotkey_account_id = 1;
//         let coldkey_account_id = 667; // Neighbour of the beast, har har
//         let email_hash: Vec<u8> = vec![0;32];
//         assert_ok!(Subtensor::set_registeration_key(<<Test as Config>::Origin>::root(), registration_id));
//         assert_ok!(Subtensor::register(<<Test as Config>::Origin>::signed(registration_id), email_hash, hotkey_account_id, coldkey_account_id));
//         Subtensor::add_stake_to_neuron_hotkey_account(0, 1_000_000_000); // Add the stake.

//         let call: mock::Call = SubtensorCall::add_stake(hotkey_account_id, 5_000).into();
//         let info = call.get_dispatch_info();

//         let result = ChargeTransactionPayment::<Test>(PhantomData).validate(&hotkey_account_id, &call, &info, 10);
//         assert_eq!(result, Ok(ValidTransaction {
//             priority: 18446744073709551615,
//             longevity: 18446744073709551615,
//             ..Default::default()
//         }))
//     });
// }

// #[test]
// fn test_charge_transaction_payment_validate_remove_stake_ok() {
//     new_test_ext().execute_with(|| {
//         // Register a neuron
//         let registration_id = 0;
//         let hotkey_account_id = 1;
//         let coldkey_account_id = 667; // Neighbour of the beast, har har
//         let email_hash: Vec<u8> = vec![0;32];
//         assert_ok!(Subtensor::set_registeration_key(<<Test as Config>::Origin>::root(), registration_id));
//         assert_ok!(Subtensor::register(<<Test as Config>::Origin>::signed(registration_id), email_hash, hotkey_account_id, coldkey_account_id));
//         Subtensor::add_stake_to_neuron_hotkey_account(0, 1_000_000_000); // Add the stake.

//         let call: mock::Call = SubtensorCall::add_stake(hotkey_account_id, 5_000).into();
//         let info = call.get_dispatch_info();

//         let result = ChargeTransactionPayment::<Test>(PhantomData).validate(&hotkey_account_id, &call, &info, 10);
//         assert_eq!(result, Ok(ValidTransaction {
//             priority: 18446744073709551615,
//             longevity: 18446744073709551615,
//             ..Default::default()
//         }))
//     });
// }

// #[test]
// fn test_charge_transaction_payment_validate_serve_axon_ok() {
//     new_test_ext().execute_with(|| {
//         // Register a neuron
//         let registration_id = 0;
//         let hotkey_account_id = 1;
//         let coldkey_account_id = 667; // Neighbour of the beast, har har
//         let email_hash: Vec<u8> = vec![0;32];
//         assert_ok!(Subtensor::set_registeration_key(<<Test as Config>::Origin>::root(), registration_id));
//         assert_ok!(Subtensor::register(<<Test as Config>::Origin>::signed(registration_id), email_hash, hotkey_account_id, coldkey_account_id));
//         Subtensor::add_stake_to_neuron_hotkey_account(0, 1_000_000_000); // Add the stake.

//         let version = 0;
// 		let ip = ipv4(8,8,8,8);
// 		let port = 8883;
// 		let ip_type = 4;
//         let modality = 0;
//         let call: mock::Call = SubtensorCall::serve_axon(version, ip, port, ip_type, modality).into();
//         let info = call.get_dispatch_info();

//         let result = ChargeTransactionPayment::<Test>(PhantomData).validate(&hotkey_account_id, &call, &info, 10);
//         assert_eq!(result, Ok(ValidTransaction {
//             priority: 18446744073709551615,
//             longevity: 18446744073709551615,
//             ..Default::default()
//         }))
//     });
// }

// #[test]
// fn test_charge_transaction_payment_validate_other_ok() {
//     let coldkey_id = 0;
//     let dest_id = 4332;
//     let len = 200;

//     test_ext_with_balances(vec![(coldkey_id, 100_000)]).execute_with(|| {
//         let call: mock::Call = BalanceCall::transfer(dest_id, 5_000).into();
//         let info = call.get_dispatch_info();

//         let result = ChargeTransactionPayment::<Test>(PhantomData).validate(&coldkey_id, &call, &info, len);
//         assert_eq!(result, Ok(ValidTransaction {
//             priority: u64::max_value(),
//             longevity: u64::max_value(), // Forevah
//             ..Default::default()
//         }))
//     });
// }

// #[test]
// fn pre_dispatch_works() {
//     new_test_ext().execute_with(|| {
//         let registration_id = 0;
//         let hotkey_account_id = 1;
//         let coldkey_account_id = 667; // Neighbour of the beast, har har
//         let email_hash: Vec<u8> = vec![0;32];
//         assert_ok!(Subtensor::set_registeration_key(<<Test as Config>::Origin>::root(), registration_id));
//         assert_ok!(Subtensor::register(<<Test as Config>::Origin>::signed(registration_id), email_hash, hotkey_account_id, coldkey_account_id));

//         assert_ok!(Subtensor::set_weights(Origin::signed(hotkey_account_id), vec![0], vec![u32::MAX]));
//         Subtensor::add_stake_to_neuron_hotkey_account(0, 1000000000); // Add the stake.
//         let call = SubtensorCall::set_weights(vec![0], vec![0]).into();
//         let info = DispatchInfo::default();
//         let len = 10;


//         let mut result = ChargeTransactionPayment::<Test>(PhantomData).pre_dispatch(&hotkey_account_id, &call, &info, len).unwrap();
//         assert_eq!(result.0, CallType::SetWeights);
//         assert_eq!(result.1, 0);
//         assert_eq!(result.2, hotkey_account_id);

//         run_to_block(1);

//         result = ChargeTransactionPayment::<Test>(PhantomData).pre_dispatch(&hotkey_account_id, &call, &info, len).unwrap();
//         assert_eq!(result.0, CallType::SetWeights);
//         assert_eq!(result.1, 0);
//         assert_eq!(result.2, hotkey_account_id);
//     });
// }

// #[test]
// fn post_dispatch_works() {
//     new_test_ext().execute_with(|| {
//         let registration_id = 0;
//         let hotkey_account_id = 1;
//         let coldkey_account_id = 667; // Neighbour of the beast, har har
//         let email_hash: Vec<u8> = vec![0;32];
//         assert_ok!(Subtensor::set_registeration_key(<<Test as Config>::Origin>::root(), registration_id));
//         assert_ok!(Subtensor::register(<<Test as Config>::Origin>::signed(registration_id), email_hash, hotkey_account_id, coldkey_account_id));
//         assert_ok!(Subtensor::set_weights(Origin::signed(hotkey_account_id), vec![0], vec![u32::MAX]));
//         Subtensor::add_stake_to_neuron_hotkey_account(0, 1000000000); // Add the stake.
        
//         let call = SubtensorCall::set_weights(vec![0], vec![0]).into();
//         let info = DispatchInfo::default();
//         let len = 10;
//         run_to_block(1);

//         let result = ChargeTransactionPayment::<Test>(PhantomData).pre_dispatch(&hotkey_account_id, &call, &info, len);
//         assert_ok!(result);

//         let pre = ChargeTransactionPayment::<Test>(PhantomData).pre_dispatch(&hotkey_account_id, &call, &info, len).unwrap();
//         assert!(ChargeTransactionPayment::<Test>::post_dispatch(pre, &info, &PostDispatchInfo {actual_weight: Some(0), pays_fee: Default::default()}, len, &Ok(())).is_ok());
//     });
// }


// #[test]
// fn test_sudo_call_does_not_pay_transaction_fee() {
//     let source_key_id = 8888;
//     let dest_key_id = 99889;
//     let balance = 1_000_000_000;
//     let amount = 500_000_000;
//     let sudo_key = 1;

//     test_ext_with_balances(vec![(source_key_id, balance)]).execute_with(|| {
//         let registration_id = 0;
//         let hotkey_account_id = 1;
//         let coldkey_account_id = 667; // Neighbour of the beast, har har
//         let email_hash: Vec<u8> = vec![0;32];
//         assert_ok!(Subtensor::set_registeration_key(<<Test as Config>::Origin>::root(), registration_id));
//         assert_ok!(Subtensor::register(<<Test as Config>::Origin>::signed(registration_id), email_hash, hotkey_account_id, coldkey_account_id));
//         assert_ok!(Subtensor::set_weights(Origin::signed(hotkey_account_id), vec![0], vec![u32::MAX]));
//         Subtensor::add_stake_to_neuron_hotkey_account(0, 1000000000); // Add the stake.

//         let call = Box::new(Call::Subtensor(SubtensorCall::add_stake(dest_key_id, amount)));
//         let sudo_call = Call::Sudo(SudoCall::sudo_unchecked_weight(call, 1_000));

//         let xt = TestXt::new(sudo_call, mock::sign_extra(sudo_key, 0));
//         let result = mock::Executive::apply_extrinsic(xt);
//         assert_ok!(result);
//     });
// }