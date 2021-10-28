use sp_core::{H256, U256};
mod mock;
use mock::*;
use sp_std::if_std; // Import into scope the if_std! macro.

#[test]
fn check_vec_to_hash() {
    let values: Vec<u8> = vec![0;32];
    Subtensor::vec_to_hash( values );
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

#[test]
fn check_seal() {
    new_test_ext().execute_with(|| {
        Subtensor::create_seal_hash( 0 ,  0);
    });
}

#[test]
fn test_nonce_bytes() {
    new_test_ext().execute_with(|| {
        Subtensor::print_seal( 0, 0, 1);
        Subtensor::print_seal( 0, 10, 10);
        Subtensor::print_seal( 0, 100, 100);
        Subtensor::print_seal( 0, 1000, 1000);
        Subtensor::print_seal( 0, 10000, 10000);

        // Subtensor::print_seal( 1 );
        // Subtensor::print_seal( 2 );
        // Subtensor::print_seal( 10 );
        // Subtensor::print_seal( 13 );
        // Subtensor::print_seal( 23 );
        // Subtensor::print_seal( 56 );
        // Subtensor::print_seal( 100 );
        // Subtensor::print_seal( 221 );
        // Subtensor::print_seal( 255 );
        // Subtensor::print_seal( 256 );
        // Subtensor::print_seal( 511 );
        // Subtensor::print_seal( 512 );
        // Subtensor::print_seal( 513 );
        // Subtensor::print_seal( u64::MAX );
        // Subtensor::print_seal( u64::MAX - 1 );
        // Subtensor::print_seal( u64::MAX/2 );
        // Subtensor::print_seal( u64::MAX/4 );
    });
}


#[test]
fn check_work_10() {
    new_test_ext().execute_with(|| {
        let block_number: u64 = 0;
        let difficulty: U256 = U256::from( 10 );
        let mut nonce: u64 = 0;
        let mut hash: H256 = Subtensor::create_seal_hash( block_number,  nonce );
        while !Subtensor::hash_meets_difficulty(&hash, difficulty) {
            nonce = nonce + 1;
            hash = Subtensor::create_seal_hash( block_number, nonce  );  
            if_std! {
                println!("nonce:{:?}, hash: {:?}", nonce, hash);
            }  
        }
        assert!( Subtensor::hash_meets_difficulty(&hash, difficulty) );
        assert!( nonce == 5 );
        
    });
}

#[test]
fn check_work_100() {
    new_test_ext().execute_with(|| {
        let block_number: u64 = 0;
        let difficulty: U256 = U256::from( 100 );
        let mut nonce: u64 = 0;
        let mut hash: H256 = Subtensor::create_seal_hash( block_number,  nonce );
        while !Subtensor::hash_meets_difficulty(&hash, difficulty) {
            nonce = nonce + 1;
            hash = Subtensor::create_seal_hash( block_number, nonce );    
            if_std! {
                println!("nonce:{:?}, hash: {:?}", nonce, hash);
            }
        }
        assert!( Subtensor::hash_meets_difficulty(&hash, difficulty) );
        assert!( nonce == 178 );
    });
}

#[test]
fn check_work_10000() {
    new_test_ext().execute_with(|| {
        let block_number: u64 = 0;
        let difficulty: U256 = U256::from( 10000 );
        let mut nonce: u64 = 0;
        let mut hash: H256 = Subtensor::create_seal_hash( block_number,  nonce );
        while !Subtensor::hash_meets_difficulty(&hash, difficulty) {
            nonce = nonce + 1;
            hash = Subtensor::create_seal_hash( block_number, nonce );    
        }
        assert!( Subtensor::hash_meets_difficulty(&hash, difficulty) );
        assert!( nonce == 13102 );
    });
}




