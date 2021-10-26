// use frame_support::{assert_ok};
// use frame_system::{Config};
// mod mock;
// use mock::*;

// /***********************************************************
// 	staking::add_stake() tests
// ************************************************************/


// // Tests the step without a neuron in the graph.
// #[test]
// fn test_run_step_ok() {
// 	new_test_ext().execute_with(|| {
//         assert_eq!( Subtensor::get_neuron_count(), 0 );
//         assert_eq!( Subtensor::get_total_stake(), 0 );
//         assert_eq!( Subtensor::get_total_issuance(), 0 );
//         run_to_block( 1 );
//         assert_eq!( Subtensor::get_neuron_count(), 0 );
//         assert_eq!( Subtensor::get_total_stake(), 0 );
//         assert_eq!( Subtensor::get_total_issuance(), 0 );
// 	});
// }

// // Tests the step with a single neuron no stake.
// #[test]
// fn test_step_with_neuron_no_balances() {
//     let coldkey:u64 = 1;
//     let hotkey:u64 = 2;
//     new_test_ext().execute_with( || {
//         assert_ok!(Subtensor::set_registeration_key(<<Test as Config>::Origin>::root(), 0));
//         let neuron = register_ok_neuron(0,0, hotkey, coldkey );
//         assert_eq!( Subtensor::get_neuron_count(), 1 );
//         assert_eq!( Subtensor::get_total_stake(), 0 );
//         assert_eq!( Subtensor::get_total_issuance(), 0 );
//         assert_eq!( Subtensor::get_stake(), vec![0] );
//         assert_eq!( Subtensor::get_ranks(), vec![0] );
//         assert_eq!( Subtensor::get_trust(), vec![0] );
//         assert_eq!( Subtensor::get_active(), vec![1] );
//         assert_eq!( Subtensor::get_consensus(), vec![0] );
//         assert_eq!( Subtensor::get_incentive(), vec![0] );
//         assert_eq!( Subtensor::get_dividends(), vec![0] );
//         assert_eq!( Subtensor::get_bonds_for_neuron(&neuron), vec![0] );
//         assert_eq!( Subtensor::get_weights_for_neuron(&neuron), vec![u32::max_value()] );
//         run_to_block( 1 );
//         assert_eq!( Subtensor::get_neuron_count(), 1 );
//         assert_eq!( Subtensor::get_total_stake(), 0);
//         assert_eq!( Subtensor::get_total_issuance(), 0 );
//         assert_eq!( Subtensor::get_stake(), vec![0] );
//         assert_eq!( Subtensor::get_ranks(), vec![0] );
//         assert_eq!( Subtensor::get_trust(), vec![0] );
//         assert_eq!( Subtensor::get_active(), vec![1] );
//         assert_eq!( Subtensor::get_consensus(), vec![0] );
//         assert_eq!( Subtensor::get_incentive(), vec![0] );
//         assert_eq!( Subtensor::get_dividends(), vec![0] );
//         assert_eq!( Subtensor::get_bonds_for_neuron(&neuron), vec![0] );
//         assert_eq!( Subtensor::get_weights_for_neuron(&neuron), vec![u32::max_value()] );
//     });
// }

// // Tests the step with a single neuron with stake.
// #[test]
// fn test_step_with_neuron_with_balances() {
//     let coldkey:u64 = 1;
//     let hotkey:u64= 2;
//     let initial_stake:u64 = 1000000000;
//     new_test_ext().execute_with( || {
//         assert_ok!(Subtensor::set_registeration_key(<<Test as Config>::Origin>::root(), 0));
//         let neuron = register_ok_neuron(0,0, hotkey, coldkey );
//         Subtensor::add_stake_to_neuron_hotkey_account(neuron.uid, initial_stake);
//         assert_eq!( Subtensor::get_total_stake(), initial_stake );
//         assert_eq!( Subtensor::get_total_issuance(), 0 );
//         assert_eq!( Subtensor::get_neuron_count(), 1 );
//         assert_eq!( Subtensor::get_stake(), vec![initial_stake] );
//         assert_eq!( Subtensor::get_ranks(), vec![0] );
//         assert_eq!( Subtensor::get_trust(), vec![0] );
//         assert_eq!( Subtensor::get_active(), vec![1] );
//         assert_eq!( Subtensor::get_consensus(), vec![0] );
//         assert_eq!( Subtensor::get_incentive(), vec![0] );
//         assert_eq!( Subtensor::get_dividends(), vec![0] );
//         assert_eq!( Subtensor::get_bonds_for_neuron(&neuron), vec![0] );
//         assert_eq!( Subtensor::get_weights_for_neuron(&neuron), vec![u32::max_value()] );
//         run_to_block( 1 );
//         assert_eq!( Subtensor::get_total_stake(), initial_stake );
//         assert_eq!( Subtensor::get_total_issuance(), 0 );
//         assert_eq!( Subtensor::get_neuron_count(), 1 );
//         assert_eq!( Subtensor::get_stake(), vec![ initial_stake ] );
//         assert_eq!( Subtensor::get_ranks(), vec![0] );
//         assert_eq!( Subtensor::get_trust(), vec![0] );
//         assert_eq!( Subtensor::get_active(), vec![1] );
//         assert_eq!( Subtensor::get_consensus(), vec![0] );
//         assert_eq!( Subtensor::get_incentive(), vec![0] );
//         assert_eq!( Subtensor::get_dividends(), vec![0] );
//     });
// }

// // Tests the step with a single neuron with stake.
// #[test]
// fn test_step_with_many() {
//     new_test_ext().execute_with( || {
//         let initial_stake:u64 = 1000000000;
//         assert_ok!(Subtensor::set_registeration_key(<<Test as Config>::Origin>::root(), 0));
//         for i in 0..4 {
//             register_ok_neuron(0,0, i as u64, i as u64 );
//         }
//         // Set weights.
//         let weights_matrix: Vec<Vec<u32>> = vec! [
//             vec! [u32::max_value(), 0, 0, 0],
//             vec! [0, u32::max_value(), 0, 0],
//             vec! [0, 0, u32::max_value(), 0], 
//             vec! [0, 0, 0, u32::max_value()],
//         ];
//         Subtensor::set_stake_from_vector( vec![ initial_stake; 4 ] );
//         assert_eq!( Subtensor::get_neuron_count(), 4 );
//         assert_eq!( Subtensor::get_total_issuance(), 0 );
//         assert_eq!( Subtensor::get_stake(), vec![ initial_stake; 4 ] );
//         assert_eq!( Subtensor::get_ranks(), vec![0; 4] );
//         assert_eq!( Subtensor::get_trust(), vec![0; 4] );
//         assert_eq!( Subtensor::get_active(), vec![1; 4] );
//         assert_eq!( Subtensor::get_consensus(), vec![0; 4] );
//         assert_eq!( Subtensor::get_incentive(), vec![0; 4] );
//         assert_eq!( Subtensor::get_emission(), vec![0; 4] );
//         assert_eq!( Subtensor::get_dividends(), vec![0; 4] );
//         assert_eq!( Subtensor::get_bonds(), vec![ [ 0; 4]; 4]);
//         assert_eq!( Subtensor::get_weights(), weights_matrix );
//         run_to_block( 1 );
//         assert_eq!( Subtensor::get_neuron_count(), 4 );
//         assert_eq!( Subtensor::get_total_issuance(), 0 );
//         assert_eq!( Subtensor::get_stake(), vec![ initial_stake; 4 ] );
//         assert_eq!( Subtensor::get_ranks(), vec![0; 4] );
//         assert_eq!( Subtensor::get_trust(), vec![0; 4] );
//         assert_eq!( Subtensor::get_active(), vec![1; 4] );
//         assert_eq!( Subtensor::get_consensus(), vec![0; 4] );
//         assert_eq!( Subtensor::get_incentive(), vec![0; 4] );
//         assert_eq!( Subtensor::get_emission(), vec![0; 4] );
//         assert_eq!( Subtensor::get_dividends(), vec![0; 4] );
//         assert_eq!( Subtensor::get_bonds(), vec![ [ 0; 4]; 4]);
//         assert_eq!( Subtensor::get_weights(), weights_matrix );
//     });
// }

// // Tests the step with a single neuron with stake.
// #[test]
// fn test_step_with_many_zero_weights() {
//     new_test_ext().execute_with( || {
//         let initial_stake:u64 = 1000000000;
//         assert_ok!(Subtensor::set_registeration_key(<<Test as Config>::Origin>::root(), 0));
//         for i in 0..4 {
//             register_ok_neuron(0,0, i as u64, i as u64 );
//         }
//         // Set stake.
//         Subtensor::set_stake_from_vector( vec![ initial_stake; 4 ] );
//         // Set weights.
//         let weights_matrix: Vec<Vec<u32>> = vec! [
//             vec! [u32::max_value(), 0, 0, 0],
//             vec! [0, u32::max_value(), 0, 0],
//             vec! [0, 0, u32::max_value(), 0], 
//             vec! [0, 0, 0, u32::max_value()],
//         ];
//         Subtensor::set_weights_from_matrix( weights_matrix.clone() );

//         assert_eq!( Subtensor::get_neuron_count(), 4 );
//         assert_eq!( Subtensor::get_total_issuance(), 0 );
//         assert_eq!( Subtensor::get_stake(), vec![ initial_stake; 4 ] );
//         assert_eq!( Subtensor::get_ranks(), vec![0; 4] );
//         assert_eq!( Subtensor::get_trust(), vec![0; 4] );
//         assert_eq!( Subtensor::get_active(), vec![1; 4] );
//         assert_eq!( Subtensor::get_consensus(), vec![0; 4] );
//         assert_eq!( Subtensor::get_incentive(), vec![0; 4] );
//         assert_eq!( Subtensor::get_emission(), vec![0; 4] );
//         assert_eq!( Subtensor::get_dividends(), vec![0; 4] );
//         assert_eq!( Subtensor::get_bonds(), vec![ [ 0; 4]; 4]);
//         assert_eq!( Subtensor::get_weights(), weights_matrix );
//         run_to_block( 1 );
//         assert_eq!( Subtensor::get_neuron_count(), 4 );
//         assert_eq!( Subtensor::get_total_issuance(), 0 );
//         assert_eq!( Subtensor::get_stake(), vec![ initial_stake; 4 ] );
//         assert_eq!( Subtensor::get_ranks(), vec![0; 4] );
//         assert_eq!( Subtensor::get_trust(), vec![0; 4] );
//         assert_eq!( Subtensor::get_active(), vec![1; 4] );
//         assert_eq!( Subtensor::get_consensus(), vec![0; 4] );
//         assert_eq!( Subtensor::get_incentive(), vec![0; 4] );
//         assert_eq!( Subtensor::get_emission(), vec![0; 4] );
//         assert_eq!( Subtensor::get_dividends(), vec![0; 4] );
//         assert_eq!( Subtensor::get_bonds(), vec![ [ 0; 4]; 4]);
//         assert_eq!( Subtensor::get_weights(), weights_matrix );
//     });
// }

// // Tests the step with a single neuron with stake.
// #[test]
// fn test_step_with_many_self_weights() {
//     new_test_ext().execute_with( || {
//         let initial_stake:u64 = 1000000000;
//         assert_ok!(Subtensor::set_registeration_key(<<Test as Config>::Origin>::root(), 0));
//         for i in 0..4 {
//             register_ok_neuron(0,0, i as u64, i as u64 );
//         }
//         // Set stake.
//         Subtensor::set_stake_from_vector( vec![ initial_stake; 4 ] );
//         // Set weights.
//         let weights_matrix: Vec<Vec<u32>> = vec! [
//             vec! [u32::max_value(), 0, 0, 0 ],
//             vec! [0, u32::max_value(), 0, 0 ],
//             vec! [0, 0, u32::max_value(), 0 ], 
//             vec! [0, 0, 0, u32::max_value() ],
//         ];
//         Subtensor::set_weights_from_matrix( weights_matrix.clone() );

//         assert_eq!( Subtensor::get_neuron_count(), 4 );
//         assert_eq!( Subtensor::get_stake(), vec![ initial_stake; 4 ] );
//         assert_eq!( Subtensor::get_weights(), weights_matrix );
//         run_to_block( 1 );
//         assert_eq!( Subtensor::get_neuron_count(), 4 );
//         assert_eq!( Subtensor::get_total_issuance(), 0 );
//         assert_eq!( Subtensor::get_stake(), vec![ initial_stake; 4 ] );
//         assert_eq!( Subtensor::get_ranks(), vec![0; 4] );
//         assert_eq!( Subtensor::get_trust(), vec![0; 4] );
//         assert_eq!( Subtensor::get_active(), vec![1; 4] );
//         assert_eq!( Subtensor::get_consensus(), vec![0; 4] );
//         assert_eq!( Subtensor::get_incentive(), vec![0; 4] );
//         assert_eq!( Subtensor::get_emission(), vec![0; 4] );
//         assert_eq!( Subtensor::get_dividends(), vec![0; 4] );
//         assert_eq!( Subtensor::get_bonds(), vec![ [ 0; 4]; 4]);
//         assert_eq!( Subtensor::get_weights(), weights_matrix );
//     });
// }

// pub fn approx_equals( a:u64, b: u64, eps: u64 ) -> bool {
//     if a > b {
//         if a - b > eps {
//             println!("a({:?}) - b({:?}) > {:?}", a, b, eps);
//             return false;
//         }
//     }
//     if b > a {
//         if b - a > eps {
//             println!("b({:?}) - a({:?}) > {:?}", b, a, eps);
//             return false;
//         }
//     }
//     return true;
// }

// pub fn vec_approx_equals( a_vec: &Vec<u64>, b_vec: &Vec<u64>, eps: u64 ) -> bool {
//     for (a, b) in a_vec.iter().zip(b_vec.iter()) {
//         if !approx_equals( *a, *b, eps ){
//             return false;
//         }
//     }
//     return true;
// }

// pub fn mat_approx_equals( a_vec: &Vec<Vec<u64>>, b_vec: &Vec<Vec<u64>>, eps: u64 ) -> bool {
//     for (a, b) in a_vec.iter().zip(b_vec.iter()) {
//         if !vec_approx_equals( a, b, eps ){
//             return false;
//         }
//     }
//     return true;
// }

// #[test]
// fn test_two_steps_with_many_outward_weights() {
//     new_test_ext().execute_with( || {
//         let initial_stake:u64 = 1000000000;
//         let u64m: u64 = 18446744073709551615;
//         assert_ok!(Subtensor::set_registeration_key(<<Test as Config>::Origin>::root(), 0));
//         for i in 0..4 {
//             register_ok_neuron(0, 0, i as u64, i as u64 );
//         }
//         // Set stake.
//         Subtensor::set_stake_from_vector( vec![ initial_stake; 4 ] );
//         // Shifted weights.
//         let weights_matrix: Vec<Vec<u32>> = vec! [
//             vec! [0, u32::max_value(), 0, 0 ],
//             vec! [0, 0, u32::max_value(), 0 ],
//             vec! [0, 0, 0, u32::max_value() ], 
//             vec! [u32::max_value(), 0, 0, 0 ],
//         ];
//         Subtensor::set_weights_from_matrix( weights_matrix.clone() );
//         assert_eq!( Subtensor::get_neuron_count(), 4 );
//         assert_eq!( Subtensor::get_stake(), vec![ initial_stake; 4 ] );
//         assert_eq!( Subtensor::get_weights(), weights_matrix );

//         step_block (1);

//         assert_eq!( Subtensor::get_neuron_count(), 4 );
//         assert!( approx_equals( Subtensor::get_total_issuance(), 1000000000, 10)); // approx
//         assert!( vec_approx_equals ( &Subtensor::get_stake(), &vec![1250000000, 1250000000, 1250000000, 1250000000], 10) );
//         assert!( vec_approx_equals ( &Subtensor::get_ranks(), &vec![u64m/4, u64m/4, u64m/4, u64m/4], 100) );
//         assert!( vec_approx_equals ( &Subtensor::get_trust(), &vec![u64m/4, u64m/4, u64m/4, u64m/4], 100) );
//         assert_eq!( Subtensor::get_active(), vec![1; 4] );
//         assert!( vec_approx_equals ( &Subtensor::get_consensus(), &vec![1399336432749266785, 1399336432749266785, 1399336432749266785, 1399336432749266785], 10) );
//         assert!( vec_approx_equals ( &Subtensor::get_incentive(), &vec![u64m/4, u64m/4, u64m/4, u64m/4], 100) );
//         assert!( vec_approx_equals ( &Subtensor::get_dividends(), &vec![u64m/4, u64m/4, u64m/4, u64m/4], 100) );
//         assert!( vec_approx_equals ( &Subtensor::get_emission(), &vec![250000000, 250000000, 250000000, 250000000], 10) );
//         let expected_bonds: Vec<Vec<u64>> = vec! [
//             vec! [0, 250000000, 0, 0 ],
//             vec! [0, 0, 250000000, 0 ],
//             vec! [0, 0, 0, 250000000 ], 
//             vec! [250000000, 0, 0, 0 ],
//         ];
//         assert!( mat_approx_equals ( &Subtensor::get_bonds(), &expected_bonds, 10) );

//         step_block (1);

//         assert_eq!( Subtensor::get_neuron_count(), 4 );
//         assert!( approx_equals( Subtensor::get_total_issuance(), 2000000000, 100)); // approx
//         assert!( vec_approx_equals ( &Subtensor::get_stake(), &vec![1500000000, 1500000000, 1500000000, 1500000000], 100) );
//         assert!( vec_approx_equals ( &Subtensor::get_ranks(), &vec![u64m/4, u64m/4, u64m/4, u64m/4], 100) );
//         assert!( vec_approx_equals ( &Subtensor::get_trust(), &vec![u64m/4, u64m/4, u64m/4, u64m/4], 100) );
//         assert_eq!( Subtensor::get_active(), vec![1; 4] );
//         assert!( vec_approx_equals ( &Subtensor::get_consensus(), &vec![1399336432749266785, 1399336432749266785, 1399336432749266785, 1399336432749266785], 10) );
//         assert!( vec_approx_equals ( &Subtensor::get_incentive(), &vec![u64m/4, u64m/4, u64m/4, u64m/4], 100) );
//         assert!( vec_approx_equals ( &Subtensor::get_dividends(), &vec![u64m/4, u64m/4, u64m/4, u64m/4], 100) );
//         assert!( vec_approx_equals ( &Subtensor::get_emission(), &vec![250000000, 250000000, 250000000, 250000000], 10) );
//         let expected_bonds: Vec<Vec<u64>> = vec! [
//             vec! [0, 500000000, 0, 0 ],
//             vec! [0, 0, 500000000, 0 ],
//             vec! [0, 0, 0, 500000000 ], 
//             vec! [500000000, 0, 0, 0 ],
//         ];
//         assert!( mat_approx_equals ( &Subtensor::get_bonds(), &expected_bonds, 100) );
        
//         step_block ( 8 );

//         assert_eq!( Subtensor::get_neuron_count(), 4 );
//         assert!( approx_equals( Subtensor::get_total_issuance(), 1000000000 * 10, 100)); // approx
//         assert!( vec_approx_equals ( &Subtensor::get_stake(), &vec![1000000000 + 250000000 * 10, 1000000000 + 250000000 * 10, 1000000000 + 250000000 * 10, 1000000000 + 250000000 * 10], 100) );
//         assert!( vec_approx_equals ( &Subtensor::get_ranks(), &vec![u64m/4, u64m/4, u64m/4, u64m/4], 100) );
//         assert!( vec_approx_equals ( &Subtensor::get_trust(), &vec![u64m/4, u64m/4, u64m/4, u64m/4], 100) );
//         assert_eq!( Subtensor::get_active(), vec![1; 4] );
//         assert!( vec_approx_equals ( &Subtensor::get_consensus(), &vec![1399336432749266785, 1399336432749266785, 1399336432749266785, 1399336432749266785], 10) );
//         assert!( vec_approx_equals ( &Subtensor::get_incentive(), &vec![u64m/4, u64m/4, u64m/4, u64m/4], 100) );
//         assert!( vec_approx_equals ( &Subtensor::get_dividends(), &vec![u64m/4, u64m/4, u64m/4, u64m/4], 100) );
//         assert!( vec_approx_equals (  &Subtensor::get_emission(), &vec![250000000, 250000000, 250000000, 250000000], 10) );
//         let expected_bonds: Vec<Vec<u64>> = vec! [
//             vec! [0, 250000000 * 10, 0, 0 ],
//             vec! [0, 0, 250000000 * 10, 0 ],
//             vec! [0, 0, 0, 250000000 * 10 ], 
//             vec! [250000000 * 10, 0, 0, 0 ],
//         ];
//         assert!( mat_approx_equals ( &Subtensor::get_bonds(), &expected_bonds, 100) );

//     });
// }
