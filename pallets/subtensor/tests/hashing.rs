use frame_support::{assert_ok};
use sp_core::{H256, U256};
mod mock;
use mock::*;


#[test]
fn check_vec_to_hash() {
    new_test_ext().execute_with(|| {
        let values: Vec<u8> = vec![0;32];
        let hash: H256 = Subtensor::vec_to_hash( values );
    });
}

#[test]
fn check_get_block_hash() {
    new_test_ext().execute_with(|| {
        Subtensor::get_block_hash_from_u64( 0 );
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
