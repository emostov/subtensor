use super::*;
use sp_std::if_std; 
use sp_std::convert::TryInto;
use sp_core::{H256, U256};
use sp_io::hashing::sha2_256;
use frame_system::{ensure_signed};

impl<T: Config> Pallet<T> {

    pub fn do_registration ( 
        origin: T::Origin, 
        block_number: u64, 
        nonce: u64, 
        work: Vec<u8>,
        hotkey: T::AccountId, 
        coldkey: T::AccountId 
    ) -> dispatch::DispatchResult {

        // --- Check the callers hotkey signature.
        ensure_signed(origin)?;

        // --- Check that registrations per block and hotkey.
        let registrations_this_block: u64 = Self::get_registrations_this_block();
        ensure! ( registrations_this_block < Self::get_max_registratations_per_block(), Error::<T>::ToManyRegistrationsThisBlock ); // Number of registrations this block exceeded.
        ensure!( !Hotkeys::<T>::contains_key(&hotkey), Error::<T>::AlreadyRegistered );  // Hotkey has already registered.

        // --- Check block number validity.
        let current_block_number: u64 = Self::get_current_block_as_u64_here();
        ensure! ( current_block_number >= block_number, Error::<T>::InvalidWorkBlock ); // TODO(const): change error.
        ensure! ( current_block_number - block_number < 100, Error::<T>::InvalidWorkBlock ); // TODO(const): change error.

        // --- Check difficulty.
        let difficulty: U256 = Self::get_difficulty();
        let work_hash: H256 = Self::vec_to_hash( work );
        ensure! ( Self::hash_meets_difficulty( &work_hash, difficulty ), Error::<T>::InvalidDifficulty ); // TODO(const): change error.

        // --- Check work.
        let seal: H256 = Self::create_seal_hash( block_number, nonce );
        ensure! ( seal == work_hash, Error::<T>::InvalidSeal ); // TODO(const): change error.
        
        // Check that the hotkey has not already been registered.
        ensure!( !Hotkeys::<T>::contains_key(&hotkey), Error::<T>::AlreadyRegistered );
        
        // --- We get the next available subscription uid.
        let uid: u32 = Self::get_next_uid();

        // --- Wee create a new entry in the table with the new metadata.
        let neuron = NeuronMetadataOf::<T> {
            version: 0,
            ip: 0,
            port: 0,
            ip_type: 0,
            uid: uid,
            modality: 0,
            hotkey: hotkey.clone(),
            coldkey: coldkey.clone(),
            active: 1,
            last_update: Self::get_current_block_as_u64(),
            priority: 0,
            stake: 0,
            rank: 0,
            trust: 0,
            consensus: 0,
            incentive: 0,
            emission: 0,
            dividends: 0,
            bonds: vec![],
            weights: vec![(uid, u32::MAX)], // self weight set to 1.
        };

        // --- Update avg registrations per 1000 block.
        RegistrationsThisInterval::<T>::mutate( |val| *val += 1 );
        RegistrationsThisBlock::<T>::mutate( |val| *val += 1 );
        
        // --- We deposit the neuron registered event.
        Neurons::<T>::insert(uid, neuron); // Insert neuron info under uid.
        Hotkeys::<T>::insert(&hotkey, uid); // Add hotkey into hotkey set.
        Self::deposit_event(Event::NeuronRegistered(uid));

        Ok(())
    }

    pub fn get_current_block_as_u64_here( ) -> u64 {
        let block_as_u64: u64 = TryInto::try_into( system::Pallet::<T>::block_number() ).ok().expect("blockchain will not exceed 2^64 blocks; QED.");
        block_as_u64
    }

    pub fn vec_to_hash( vec_hash: Vec<u8> ) -> H256 {
        let de_ref_hash = &vec_hash; // b: &Vec<u8>
        let de_de_ref_hash: &[u8] = &de_ref_hash; // c: &[u8]
        let real_hash: H256 = H256::from_slice( de_de_ref_hash );
        // if_std! {
        //     println!("real_hash: {:?}, vec_hash{:?}", real_hash, vec_hash);
        // }
        return real_hash
    }

    /// Determine whether the given hash satisfies the given difficulty.
    /// The test is done by multiplying the two together. If the product
    /// overflows the bounds of U256, then the product (and thus the hash)
    /// was too high.
    pub fn hash_meets_difficulty(hash: &H256, difficulty: U256) -> bool {
        let bytes: &[u8] = &hash.as_bytes();
        let num_hash: U256 = U256::from( bytes );
        let (_, overflowed) = num_hash.overflowing_mul(difficulty);
        // if_std! {
        //     println!("Difficulty: hash:{:?}, hash_bytes: {:?}, hash_as_num: {:?}, difficulty:{:?}, value: {:?} overflowed: {:?}", hash, bytes, num_hash, difficulty, value, overflowed);
        // }
        !overflowed
    }

    pub fn get_block_hash_from_u64 ( block_number: u64 ) -> H256 {
        let block_number: T::BlockNumber = TryInto::<T::BlockNumber>::try_into( block_number ).ok().expect("convert u64 to block number.");
        let block_hash_at_number: <T as frame_system::Config>::Hash = system::Pallet::<T>::block_hash( block_number );
        let vec_hash: Vec<u8> = block_hash_at_number.as_ref().into_iter().cloned().collect();
        let deref_vec_hash: &[u8] = &vec_hash; // c: &[u8]
        let real_hash: H256 = H256::from_slice( deref_vec_hash );
        // if_std! {
        //     println!("block_number: {:?}, vec_hash: {:?}, real_hash: {:?}", block_number, vec_hash, real_hash);
        // }
        return real_hash;
    }

    pub fn hash_to_vec( hash: H256 ) -> Vec<u8> {
        let hash_as_bytes: &[u8] = hash.as_bytes();
        let hash_as_vec: Vec<u8> = hash_as_bytes.iter().cloned().collect();
        return hash_as_vec
    }

    pub fn create_seal_hash( block_number_u64: u64, nonce_u64: u64 ) -> H256 {
        let nonce = U256::from( nonce_u64 );
        let block_hash_at_number: H256 = Self::get_block_hash_from_u64( block_number_u64 );
        let block_hash_bytes: &[u8] = block_hash_at_number.as_bytes();
        let full_bytes: &[u8; 40] = &[
            nonce.byte(0),  nonce.byte(1),  nonce.byte(2),  nonce.byte(3), 
            nonce.byte(4),  nonce.byte(5),  nonce.byte(6),  nonce.byte(7),

            block_hash_bytes[0], block_hash_bytes[1], block_hash_bytes[2], block_hash_bytes[3],
            block_hash_bytes[4], block_hash_bytes[5], block_hash_bytes[6], block_hash_bytes[7],
            block_hash_bytes[8], block_hash_bytes[9], block_hash_bytes[10], block_hash_bytes[11],
            block_hash_bytes[12], block_hash_bytes[13], block_hash_bytes[14], block_hash_bytes[15],

            block_hash_bytes[16], block_hash_bytes[17], block_hash_bytes[18], block_hash_bytes[19],
            block_hash_bytes[20], block_hash_bytes[21], block_hash_bytes[22], block_hash_bytes[23],
            block_hash_bytes[24], block_hash_bytes[25], block_hash_bytes[26], block_hash_bytes[27],
            block_hash_bytes[28], block_hash_bytes[29], block_hash_bytes[30], block_hash_bytes[31],
        ];
        let seal_hash_vec: [u8; 32] = sha2_256( full_bytes );
        let seal_hash: H256 = H256::from_slice( &seal_hash_vec );
        // if_std! {
        //     println!("\nblock_number: {:?}, \nnonce_u64: {:?}, \nblock_hash: {:?}, \nfull_bytes: {:?}, \nseal_hash_vec: {:?}, \nseal_hash: {:?}", block_number_u64, nonce_u64, block_hash_at_number, full_bytes, seal_hash_vec, seal_hash);
        // }
        return seal_hash;
    }

    // Helper function for creating nonce and work.
    pub fn create_work_for_block_number( block_number: u64 ) -> (u64, Vec<u8>) {
        let difficulty: U256 = Self::get_difficulty();
        let mut nonce: u64 = 0;
        let mut work: H256 = Self::create_seal_hash( block_number, nonce );
        while !Self::hash_meets_difficulty(&work, difficulty) {
            nonce = nonce + 1;
            work = Self::create_seal_hash( block_number, nonce );    
        }
        let vec_work: Vec<u8> = Self::hash_to_vec( work );
        return (nonce, vec_work)
    }

    pub fn print_seal( block_number: u64, nonce_u64: u64, difficulty: u64 ) {
        let block_hash: H256 = Self::get_block_hash_from_u64(block_number);
        let block_hash_bytes: &[u8] = block_hash.as_bytes();
        let nonce = U256::from( nonce_u64 );
        let full_bytes: &[u8; 40] = &[
            nonce.byte(0),  nonce.byte(1),  nonce.byte(2),  nonce.byte(3), 
            nonce.byte(4),  nonce.byte(5),  nonce.byte(6),  nonce.byte(7),
            block_hash_bytes[0], block_hash_bytes[1], block_hash_bytes[2], block_hash_bytes[3],
            block_hash_bytes[4], block_hash_bytes[5], block_hash_bytes[6], block_hash_bytes[7],
            block_hash_bytes[8], block_hash_bytes[9], block_hash_bytes[10], block_hash_bytes[11],
            block_hash_bytes[12], block_hash_bytes[13], block_hash_bytes[14], block_hash_bytes[15],

            block_hash_bytes[16], block_hash_bytes[17], block_hash_bytes[18], block_hash_bytes[19],
            block_hash_bytes[20], block_hash_bytes[21], block_hash_bytes[22], block_hash_bytes[23],
            block_hash_bytes[24], block_hash_bytes[25], block_hash_bytes[26], block_hash_bytes[27],
            block_hash_bytes[28], block_hash_bytes[29], block_hash_bytes[30], block_hash_bytes[31],
        ];
        //let pre_seal: Vec<u8> = &[nonce_bytes, block_hash_bytes].concat();
        let seal_hash_vec: [u8; 32] = sha2_256( full_bytes );
        let seal_hash: H256 = H256::from_slice( &seal_hash_vec );
        if_std! {
            println!("\nblock_number: {:?}, \nnonce_u64: {:?}, \nblock_hash: {:?}, \nfull_bytes: {:?}, \nblock_hash_bytes: {:?}, \nseal_hash_vec: {:?}, \nseal_hash: {:?}", block_number, nonce_u64, block_hash, full_bytes, block_hash_bytes, seal_hash_vec, seal_hash);
        }

        let difficulty = U256::from( difficulty );
        let bytes: &[u8] = &seal_hash.as_bytes();
        let num_hash: U256 = U256::from( bytes );
        let (value, overflowed) = num_hash.overflowing_mul(difficulty);
        if_std! {
            println!("Difficulty: \nseal_hash:{:?}, \nhash_bytes: {:?}, \nhash_as_num: {:?}, \ndifficulty:{:?}, \nvalue: {:?} \noverflowed: {:?}", seal_hash, bytes, num_hash, difficulty, value, overflowed);
        }
    }
}
