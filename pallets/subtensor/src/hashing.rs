use super::*;
use sp_std::if_std; // Import into scope the if_std! macro.
use sp_std::convert::TryInto;
use sp_core::{H256, U256};
// use sha3::{Digest, Sha3_256};

impl<T: Config> Pallet<T> {

    pub fn get_current_block_as_u64( ) -> u64 {
        let block_as_u64: u64 = TryInto::try_into( system::Pallet::<T>::block_number() ).ok().expect("blockchain will not exceed 2^64 blocks; QED.");
        block_as_u64
    }

    pub fn get_block_hash_from_u64 ( block_number: u64 ) -> H256 {
        let block_number: T::BlockNumber = TryInto::<T::BlockNumber>::try_into( block_number ).ok().expect("convert u64 to block number.");
        let block_hash_at_number: <T as frame_system::Config>::Hash = system::Pallet::<T>::block_hash( block_number );
        let vec_hash: Vec<u8> = block_hash_at_number.as_ref().into_iter().cloned().collect();
        let deref_vec_hash: &[u8] = &vec_hash; // c: &[u8]
        let real_hash: H256 = H256::from_slice( deref_vec_hash );
        return real_hash;
    }

    pub fn vec_to_hash( vec_hash: Vec<u8> ) -> H256 {
        let de_ref_hash = &vec_hash; // b: &Vec<u8>
        let de_de_ref_hash: &[u8] = &de_ref_hash; // c: &[u8]
        let real_hash: H256 = H256::from_slice( de_de_ref_hash );
        if_std! {
            println!("real_hash: {:?}, vec_hash{:?}", real_hash, vec_hash);
        }
        return real_hash
    }

    /// Determine whether the given hash satisfies the given difficulty.
    /// The test is done by multiplying the two together. If the product
    /// overflows the bounds of U256, then the product (and thus the hash)
    /// was too high.
    pub fn hash_meets_difficulty(hash: &H256, difficulty: U256) -> bool {
        let num_hash = U256::from(&hash[..]);
        let (value, overflowed) = num_hash.overflowing_mul(difficulty);
        if_std! {
            println!("num_hash: {:?}, value: {:?} overflowed: {:?}", num_hash, value, overflowed);
        }
        !overflowed
    }

    pub fn hash_matches_seal ( block_number: u64, block_hash: H256, nonce: U256, submitted_work: H256 ) -> bool {

        // Check block number range.
        let current_block_number: u64 = Self::get_current_block_as_u64();
        if current_block_number < block_number {
            return false
        }

        // Hash for block number
        let block_hash_at_number: H256 = Self::get_block_hash_from_u64( block_number );
        if block_hash_at_number != block_hash {
            return false;
        }

        let hash_as_bytes: &[u8] = block_hash::as_bytes();
        let nonce_as_bytes: &[u8] = nonce::as_bytes();
        let seal_as_bytes: [u8; 32]

        let vv: Vec<u8> = vec![1,2,3];
        let vv = &vv; // b: &Vec<u8>
        let vv: &[u8] = &vv; // c: &[u8]
        let vv: [u8; 32] = sha2_256( vv );
        return true;
    }

}
