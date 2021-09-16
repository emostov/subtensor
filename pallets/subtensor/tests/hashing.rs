use frame_support::{assert_ok};
use sp_core::{H256, U256};
mod mock;
use mock::*;
use sp_std::if_std; // Import into scope the if_std! macro.

#[test]
fn check_vec_to_hash() {
    let values: Vec<u8> = vec![0;32];
    let hash: H256 = Subtensor::vec_to_hash( values );
}

#[test]
fn check_get_block_hash() {
    new_test_ext().execute_with(|| {
        let block_hash_0: H256 = Subtensor::get_block_hash_from_u64( 0 );
        println!( "{:?}", block_hash_0 );
        let block_hash_1: H256 = Subtensor::get_block_hash_from_u64( 1 );
        println!( "{:?}", block_hash_1);
        assert_eq! ( block_hash_1, Subtensor::vec_to_hash( vec![0;32] ));
        step_block (1);
        let block_hash_1: H256 = Subtensor::get_block_hash_from_u64( 1 );
        println!( "{:?}", block_hash_1);
    });
}

#[test]
fn check_vec_to_hash_meets_difficulty() {
    let difficulty: U256 = U256::zero();
    let values: Vec<u8> = vec![0;32];
    let hash: H256 = Subtensor::vec_to_hash( values );
    assert!( Subtensor::hash_meets_difficulty(&hash, difficulty) );
}

#[test]
fn check_vec_to_hash_does_not_meet_difficulty() {
    let difficulty: U256 = U256::from(2);
    let values: Vec<u8> = vec![u8::max_value();32];
    let hash: H256 = Subtensor::vec_to_hash( values );
    assert!( !Subtensor::hash_meets_difficulty(&hash, difficulty) );
}


