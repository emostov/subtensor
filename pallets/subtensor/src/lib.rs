#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>

/// ************************************************************
/// -Substrate-Imports
/// ************************************************************
pub use pallet::*;

// use codec::{Decode, Encode};
use frame_support::{dispatch, ensure, traits::{
		Currency, 
		ExistenceRequirement,
		// IsSubType, 
		tokens::{
			WithdrawReasons
		}
	},
	// }, weights::{
	// 	DispatchInfo, 
	// 	PostDispatchInfo, 
	// 	Pays
	// }
};

// use frame_support::sp_runtime::FixedPointOperand;
// use frame_support::dispatch::GetDispatchInfo;
// use frame_support::sp_runtime::transaction_validity::ValidTransaction;
use frame_system::{
	self as system, 
	ensure_signed
};

use substrate_fixed::types::U64F64;
// use sp_runtime::{
// 	traits::{
// 		Dispatchable, 
// 		DispatchInfoOf, 
// 		SignedExtension, 
// 		PostDispatchInfoOf
// 	},
// 	transaction_validity::{
//         TransactionValidityError, 
// 		TransactionValidity, 
// 		InvalidTransaction,
//     }
// };
// use sp_std::convert::TryInto;
use sp_std::vec::Vec;
use sp_std::vec;
// use sp_std::marker::PhantomData;

/// ************************************************************
///	-Subtensor-Imports
/// ************************************************************
mod weights;
mod staking;
mod subscribing;
mod step;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*, Printable, traits::{Currency}};
	use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;
	use sp_std::vec;
	use sp_std::if_std; // Import into scope the if_std! macro.
	use sp_core::{H256, U256};
	use sp_io::hashing::sha2_256;
	// use sha3::{Digest, Sha3_256};
	use sp_std::convert::TryInto;
	use frame_support::IterableStorageMap;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// ************************************************************
	///	-Parameters
	/// ************************************************************
	/// Substensor parameters.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// --- Currency type that will be used to place deposits on Metagraph
		type Currency: Currency<Self::AccountId> + Send + Sync;
		
		/// - The transaction fee in RAO per byte
		type TransactionByteFee: Get<BalanceOf<Self>>;
	}

	/// ************************************************************
	///	-Pallet-Types
	/// ************************************************************
	/// Subtensor custom types.
	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub type NeuronMetadataOf<T> = NeuronMetadata<AccountIdOf<T>>;
	pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
	/// A Seal struct that will be encoded to a Vec<u8> as used as the
	/// `RawSeal` type.
	#[derive(Clone, PartialEq, Eq, Encode, Decode, Debug)]
	pub struct Seal {
		pub block_hash: H256,
		pub nonce: U256,
	}

    #[derive(Encode, Decode, Default)]
    pub struct NeuronMetadata<AccountId> {

		/// ---- The endpoint's code version.
        pub version: u32,

        /// ---- The endpoint's u128 encoded ip address of type v6 or v4.
        pub ip: u128,

        /// ---- The endpoint's u16 encoded port.
        pub port: u16,

        /// ---- The endpoint's ip type, 4 for ipv4 and 6 for ipv6.
        pub ip_type: u8,

        /// ---- The endpoint's unique identifier.
        pub uid: u32,

        /// ---- The neuron modality. Modalities specify which datatype
        /// the neuron endpoint can process. This information is non
        /// verifiable. However, Metagraph should set this correctly
        /// in order to be detected by others with this datatype.
        /// The initial modality codes are:
        /// TEXT: 0
        /// IMAGE: 1
        /// TENSOR: 2
        pub modality: u8,

        /// ---- The associated hotkey account.
        /// Subscribing and changing weights can be made by this
        /// account. Subscription can never change the associated coldkey
        /// account.
        pub hotkey: AccountId,

        /// ---- The associated coldkey account.
        /// Staking and unstaking transactions must be made by this account.
        /// The hotkey account (in the Metagraph map) has permission to call
        /// subscribe and unsubscribe.
        pub coldkey: AccountId,

		/// ---- Is this neuron active in the incentive mechanism.
		pub active: u32,

		/// ---- Block number of last chain update.
		pub last_update: u64,

		/// ---- The associated stake in this account.
		pub stake: u64,

		/// ---- The associated rank in this account.
		pub rank: u64,

		/// ---- The associated trust in this account.
		pub trust: u64,

		/// ---- The associated consensus in this account.
		pub consensus: u64,

		/// ---- The associated incentive in this account.
		pub incentive: u64,

		/// ---- The associated inflation in this account.
		pub inflation: u64,

		/// ---- The associated dividends in this account.
		pub dividends: u64,

		/// ---- The associated bond ownership.
		pub bonds: Vec<(u32,u64)>,

		/// ---- The associated weights ownership.
		pub weights: Vec<(u32,u32)>,
    }

	/// ************************************************************
	///	*---- Storage Objects
	/// ************************************************************
	
	// --- Number of peers.
	#[pallet::storage]
	pub type N<T> = StorageValue<
		_, 
		u32, 
		ValueQuery
	>;

	#[pallet::storage]
	pub type TotalStake<T> = StorageValue<
		_, 
		u64, 
		ValueQuery
	>;

	#[pallet::storage]
	pub type TotalIssuance<T> = StorageValue<
		_, 
		u64, 
		ValueQuery
	>;

	/// ---- Maps from hotkey to uid.
	#[pallet::storage]
	#[pallet::getter(fn hotkey)]
    pub(super) type Hotkeys<T:Config> = StorageMap<
		_, 
		Blake2_128Concat, 
		T::AccountId, 
		u32, 
		ValueQuery
	>;

	/// ---- Maps from uid to neuron.
	#[pallet::storage]
    #[pallet::getter(fn uid)]
    pub(super) type Metagraph<T:Config> = StorageMap<
		_, 
		Identity, 
		u32, 
		NeuronMetadataOf<T>, 
		ValueQuery
	>;

	/// ************************************************************
	///	-Genesis-Configuration
	/// ************************************************************
	/// ---- Genesis Configuration (Mostly used for testing.)
    #[pallet::genesis_config]
    pub struct GenesisConfig {
        pub stake: Vec<(u64, u64)>,
    }

	#[cfg(feature = "std")]
	impl Default for GenesisConfig {
		fn default() -> Self {
			Self {
				stake: Default::default(),
			}
		}
	}
    
    #[pallet::genesis_build]
    impl<T:Config> GenesisBuild<T> for GenesisConfig {
        fn build(&self) {
        }
	}

	#[cfg(feature = "std")]
	impl GenesisConfig {
		/// Direct implementation of `GenesisBuild::build_storage`.
		///
		/// Kept in order not to break dependency.
		pub fn build_storage<T: Config>(&self) -> Result<sp_runtime::Storage, String> {
			<Self as GenesisBuild<T>>::build_storage(self)
		}

		/// Direct implementation of `GenesisBuild::assimilate_storage`.
		///
		/// Kept in order not to break dependency.
		pub fn assimilate_storage<T: Config>(
			&self,
			storage: &mut sp_runtime::Storage
		) -> Result<(), String> {
			<Self as GenesisBuild<T>>::assimilate_storage(self, storage)
		}
	}
	
	
	/// ************************************************************
	///	-Events
	/// ************************************************************
	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
        /// ---- Event created when a caller successfully set's their weights
		/// on the chain.
		WeightsSet(T::AccountId),

		/// --- Event created when a new neuron account has been subscribed to 
		/// the neuron set.
		NeuronAdded(u32),

		/// --- Event created when the neuron information associated with a hotkey
		/// is changed, for instance, when the ip/port changes.
		NeuronUpdated(u32),

		/// --- Event created during when stake has been transfered from 
		/// the coldkey onto the hotkey staking account.
		StakeAdded(T::AccountId, u64),

		/// --- Event created when stake has been removed from 
		/// the staking account into the coldkey account.
		StakeRemoved(T::AccountId, u64),
	}

	/// ************************************************************
	///	-Errors
	/// ************************************************************
	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
        /// ---- Thrown when the user tries to subscribe a neuron which is not of type
	    /// 4 (IPv4) or 6 (IPv6).
		InvalidIpType,

		/// --- Thrown when an invalid IP address is passed to the subscribe function.
		InvalidIpAddress,

		/// --- Thrown when an invalid modality attempted on subscribe.
		/// Currently the chain only accepts modality TEXT = 0.
		InvalidModality,

		/// ---- Thrown when the caller attempts to set the weight keys
		/// and values but these vectors have different size.
		WeightVecNotEqualSize,

		/// ---- Thrown when the caller attempts to set weights with duplicate uids
		/// in the weight matrix.
		DuplicateUids,

		/// ---- Thrown when a caller attempts to set weight to at least one uid that
		/// does not exist in the metagraph.
		InvalidUid,

		/// ---- Thrown when the caller requests setting or removing data from
		/// a neuron which does not exist in the active set.
		NotActive,

		/// ---- Thrown when the caller requests subscribing a neuron which 
		/// already exists in the active set.
		AlreadyActive,

		/// ---- Thrown when a stake, unstake or subscribe request is made by a coldkey
		/// which is not associated with the hotkey account. 
		/// See: fn add_stake and fn remove_stake.
		NonAssociatedColdKey,

		/// ---- Thrown when the caller requests removing more stake then there exists 
		/// in the staking account. See: fn remove_stake.
		NotEnoughStaketoWithdraw,

		///  ---- Thrown when the caller requests adding more stake than there exists
		/// in the cold key account. See: fn add_stake
		NotEnoughBalanceToStake,

		/// ---- Thrown when the caller tries to add stake, but for some reason the requested
		/// amount could not be withdrawn from the coldkey account
		BalanceWithdrawalError,

		/// ---- Thrown when the dispatch attempts to convert between a u64 and T::balance 
		/// but the call fails.
		CouldNotConvertToBalance
	}
    impl<T: Config> Printable for Error<T> {
        fn print(&self) {
            match self {
                Error::AlreadyActive => "The node with the supplied public key is already active".print(),
                Error::NotActive => "The node with the supplied public key is not active".print(),
                Error::WeightVecNotEqualSize => "The vec of keys and the vec of values are not of the same size".print(),
                Error::NonAssociatedColdKey => "The used cold key is not associated with the hot key acccount".print(),
                _ => "Invalid Error Case".print(),
            }
        }
    }

	/// ************************************************************
	/// -Block-Hooks
	/// ************************************************************
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {

		/// ---- Called on the initialization of this pallet. (the order of on_finalize calls is determined in the runtime)
		///
		/// # Args:
		/// 	* 'n': (T::BlockNumber):
		/// 		- The number of the block we are initializing.
		fn on_initialize( _n: BlockNumberFor<T> ) -> Weight {
			Self::block_step();
			return 0;
		}
	}
    

	/// ************************************************************
	///	-Dispatchable-functions
	/// ************************************************************
	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		
        /// --- Sets the caller weights for the incentive mechanism. The call can be
		/// made from the hotkey account so is potentially insecure, however, the damage
		/// of changing weights is minimal if caught early. This function includes all the
		/// checks that the passed weights meet the requirements. Stored as u32s they represent
		/// rational values in the range [0,1] which sum to 1 and can be interpreted as
		/// probabilities. The specific weights determine how inflation propagates outward
		/// from this peer. 
		/// 
		/// Note: The 32 bit integers weights should represent 1.0 as the max u32.
		/// However, the function normalizes all integers to u32_max anyway. This means that if the sum of all
		/// elements is larger or smaller than the amount of elements * u32_max, all elements
		/// will be corrected for this deviation. 
		/// 
		/// # Args:
		/// 	* `origin`: (<T as frame_system::Config>Origin):
		/// 		- The caller, a hotkey who wishes to set their weights.
		/// 
		/// 	* `uids` (Vec<u32>):
		/// 		- The edge endpoint for the weight, i.e. j for w_ij.
		///
		/// 	* 'weights' (Vec<u32>):
		/// 		- The u32 integer encoded weights. Interpreted as rational
		/// 		values in the range [0,1]. They must sum to in32::MAX.
		///
		/// # Event:
		/// 	* WeightsSet;
		/// 		- On successfully setting the weights on chain.
		///
		/// # Raises:
		/// 	* 'WeightVecNotEqualSize':
		/// 		- If the passed weights and uids have unequal size.
		///
		/// 	* 'WeightSumToLarge':
		/// 		- When the calling coldkey is not associated with the hotkey account.
		///
		/// 	* 'InsufficientBalance':
		/// 		- When the amount to stake exceeds the amount of balance in the
		/// 		associated colkey account.
		///
        #[pallet::weight((0, DispatchClass::Normal, Pays::No))]
		pub fn set_weights(origin:OriginFor<T>, dests: Vec<u32>, weights: Vec<u32>) -> DispatchResult {
			Self::do_set_weights(origin, dests, weights)
		}
		
		/// --- Adds stake to a neuron account. The call is made from the
		/// coldkey account linked in the Metagraph's NeuronMetadata.
		/// Only the associated coldkey is allowed to make staking and
		/// unstaking requests. This protects the neuron against
		/// attacks on its hotkey running in production code.
		///
		/// # Args:
		/// 	* 'origin': (<T as frame_system::Config>Origin):
		/// 		- The caller, a coldkey signature associated with the hotkey account.
		///
		/// 	* 'hotkey' (T::AccountId):
		/// 		- The hotkey account to add stake to.
		///
		/// 	* 'ammount_staked' (u64):
		/// 		- The ammount to transfer from the balances account of the cold key
		/// 		into the staking account of the hotkey.
		///
		/// # Event:
		/// 	* 'StakeAdded':
		/// 		- On the successful staking of funds.
		///
		/// # Raises:
		/// 	* 'NotActive':
		/// 		- If the hotkey account is not active (has not subscribed)
		///
		/// 	* 'NonAssociatedColdKey':
		/// 		- When the calling coldkey is not associated with the hotkey account.
		///
		/// 	* 'InsufficientBalance':
		/// 		- When the amount to stake exceeds the amount of balance in the
		/// 		associated colkey account.
		///
		#[pallet::weight((0, DispatchClass::Normal, Pays::Yes))]
		pub fn add_stake(origin:OriginFor<T>, hotkey: T::AccountId, ammount_staked: u64) -> DispatchResult {
			Self::do_add_stake(origin, hotkey, ammount_staked)
		}

		#[pallet::weight((0, DispatchClass::Normal, Pays::Yes))]
		pub fn hash(origin:OriginFor<T>, hash: Vec<u8>) -> DispatchResult {
			let de_ref_hash = &hash; // b: &Vec<u8>
			let de_de_ref_hash: &[u8] = &de_ref_hash; // c: &[u8]
			let hash: H256 = H256::from_slice( de_de_ref_hash );
			Ok(())
		}

		/// ---- Remove stake from the staking account. The call must be made
		/// from the coldkey account attached to the neuron metadata. Only this key
		/// has permission to make staking and unstaking requests.
		///
		/// # Args:
		/// 	* 'origin': (<T as frame_system::Config>Origin):
		/// 		- The caller, a coldkey signature associated with the hotkey account.
		///
		/// 	* 'hotkey' (T::AccountId):
		/// 		- The hotkey account to withdraw stake from.
		///
		/// 	* 'ammount_unstaked' (u64):
		/// 		- The ammount to transfer from the staking account into the balance
		/// 		of the coldkey.
		///
		/// # Event:
		/// 	* 'StakeRemoved':
		/// 		- On successful withdrawl.
		///
		/// # Raises:
		/// 	* 'NonAssociatedColdKey':
		/// 		- When the calling coldkey is not associated with the hotkey account.
		///
		/// 	* 'NotEnoughStaketoWithdraw':
		/// 		- When the amount to unstake exceeds the quantity staked in the
		/// 		associated hotkey staking account.
		///
		#[pallet::weight((0, DispatchClass::Normal, Pays::Yes))]
		pub fn remove_stake(origin:OriginFor<T>, hotkey: T::AccountId, ammount_unstaked: u64) -> DispatchResult {
			Self::do_remove_stake(origin, hotkey, ammount_unstaked)
		}

		/// ---- Subscribes or updates info for caller with the given metadata. If the caller
		/// already exists in the active set, the metadata is updated but the cold key remains unchanged.
		/// If the caller does not exist they make a link between this hotkey account
		/// and the passed coldkey account. Only the cold key has permission to make add_stake/remove_stake calls.
		///
		/// # Args:
		/// 	* 'origin': (<T as frame_system::Config>Origin):
		/// 		- The caller, a hotkey associated with the subscribing neuron.
		///
		/// 	* 'ip' (u128):
		/// 		- The u64 encoded IP address of type 6 or 4.
		///
		/// 	* 'port' (u16):
		/// 		- The port number where this neuron receives RPC requests.
		///
		/// 	* 'ip_type' (u8):
		/// 		- The ip type one of (4,6).
		/// 
		/// 	* 'modality' (u8):
		/// 		- The neuron modality type.
		///
		/// 	* 'coldkey' (T::AccountId):
		/// 		- The associated coldkey to be attached to the account.
		///
		/// # Event:
		/// 	* 'NeuronAdded':
		/// 		- On subscription of a new neuron to the active set.
		///
		/// 	* 'NeuronUpdated':
		/// 		- On subscription of new metadata attached to the calling hotkey.
		#[pallet::weight((0, DispatchClass::Normal, Pays::No))]
		pub fn subscribe(origin:OriginFor<T>, version: u32, ip: u128, port: u16, ip_type: u8, modality: u8, coldkey: T::AccountId) -> DispatchResult {
			Self::do_subscribe(origin, version, ip, port, ip_type, modality, coldkey)
		}
	}
	
	// ---- Subtensor helper functions.
	impl<T: Config> Pallet<T> {

		// Getters.
		pub fn get_total_stake( ) -> u64 {
			return TotalStake::<T>::get();
		}
		pub fn get_total_issuance( ) -> u64 {
			return TotalIssuance::<T>::get();
		}
		pub fn get_lastupdate( ) -> Vec<u64> {
			let mut result: Vec<u64> = vec![ 0; Self::get_neuron_count() as usize ];
			for ( uid_i, neuron_i ) in <Metagraph<T> as IterableStorageMap<u32, NeuronMetadataOf<T>>>::iter() {
				result[ uid_i as usize] = neuron_i.last_update;
			}
			return result
		}
		pub fn get_stake( ) -> Vec<u64> {
			let mut result: Vec<u64> = vec![ 0; Self::get_neuron_count() as usize ];
			for ( uid_i, neuron_i ) in <Metagraph<T> as IterableStorageMap<u32, NeuronMetadataOf<T>>>::iter() {
				result[ uid_i as usize ] = neuron_i.stake;
			}
			return result
		}
		pub fn get_ranks( ) -> Vec<u64> {
			let mut result: Vec<u64> = vec![ 0; Self::get_neuron_count() as usize ];
			for ( uid_i, neuron_i ) in <Metagraph<T> as IterableStorageMap<u32, NeuronMetadataOf<T>>>::iter() {
				result[ uid_i as usize] = neuron_i.rank;
			}
			return result
		}
		pub fn get_trust( ) -> Vec<u64> {
			let mut result: Vec<u64> = vec![ 0; Self::get_neuron_count() as usize ];
			for ( uid_i, neuron_i ) in <Metagraph<T> as IterableStorageMap<u32, NeuronMetadataOf<T>>>::iter() {
				result[ uid_i as usize] = neuron_i.trust;
			}
			return result
		}
		pub fn get_consensus( ) -> Vec<u64> {
			let mut result: Vec<u64> = vec![ 0; Self::get_neuron_count() as usize ];
			for ( uid_i, neuron_i ) in <Metagraph<T> as IterableStorageMap<u32, NeuronMetadataOf<T>>>::iter() {
				result[ uid_i as usize] = neuron_i.consensus;
			}
			return result
		}
		pub fn get_incentive( ) -> Vec<u64> {
			let mut result: Vec<u64> = vec![ 0; Self::get_neuron_count() as usize ];
			for ( uid_i, neuron_i ) in <Metagraph<T> as IterableStorageMap<u32, NeuronMetadataOf<T>>>::iter() {
				result[ uid_i as usize] = neuron_i.incentive;
			}
			return result
		}
		pub fn get_inflation( ) -> Vec<u64> {
			let mut result: Vec<u64> = vec![ 0; Self::get_neuron_count() as usize ];
			for ( uid_i, neuron_i ) in <Metagraph<T> as IterableStorageMap<u32, NeuronMetadataOf<T>>>::iter() {
				result[ uid_i as usize] = neuron_i.inflation;
			}
			return result
		}
		pub fn get_dividends( ) -> Vec<u64> {
			let mut result: Vec<u64> = vec![ 0; Self::get_neuron_count() as usize ];
			for ( uid_i, neuron_i ) in <Metagraph<T> as IterableStorageMap<u32, NeuronMetadataOf<T>>>::iter() {
				result[ uid_i as usize] = neuron_i.dividends;
			}
			return result
		}
		pub fn get_active( ) -> Vec<u32> {
			let mut result: Vec<u32> = vec![ 0; Self::get_neuron_count() as usize ];
			for ( uid_i, neuron_i ) in <Metagraph<T> as IterableStorageMap<u32, NeuronMetadataOf<T>>>::iter() {
				result[ uid_i as usize] = neuron_i.active;
			}
			return result
		}
		pub fn get_bonds_for_neuron( neuron: &NeuronMetadataOf<T> ) -> Vec<u64>  {
			let mut bonds: Vec<u64> = vec![ 0; Self::get_neuron_count() as usize ];
			for (uid_j, bonds_ij) in neuron.bonds.iter(){
				bonds[ *uid_j as usize ] = *bonds_ij;
			}
			return bonds
		}
		pub fn get_bonds( ) -> Vec<Vec<u64>>  {
			let mut bonds: Vec<Vec<u64>> = vec![ vec![]; Self::get_neuron_count() as usize ];
			for ( uid_i, neuron_i ) in <Metagraph<T> as IterableStorageMap<u32, NeuronMetadataOf<T>>>::iter() {
				bonds[ uid_i as usize ] = Self::get_bonds_for_neuron( &neuron_i );
			}
			return bonds
		}
		pub fn get_weights_for_neuron( neuron: &NeuronMetadataOf<T> ) -> Vec<u32>  {
			let mut weights: Vec<u32> = vec![ 0; Self::get_neuron_count() as usize ];
			for (uid_j, weights_ij) in neuron.weights.iter(){
				weights[ *uid_j as usize ] = *weights_ij;
			}
			return weights
		}
		pub fn get_weights( ) -> Vec<Vec<u32>>  {
			let mut weights: Vec<Vec<u32>> = vec![ vec![]; Self::get_neuron_count() as usize ];
			for ( uid_i, neuron_i ) in <Metagraph<T> as IterableStorageMap<u32, NeuronMetadataOf<T>>>::iter() {
				weights[ uid_i as usize ] = Self::get_weights_for_neuron( &neuron_i );
			}
			return weights
		}		

		// Setters
		pub fn set_stake_from_vector( stake: Vec<u64> ) {
			for uid_i in 0..Self::get_neuron_count() {
				let mut neuron = Metagraph::<T>::get(uid_i);
				neuron.stake = stake[ uid_i as usize ];
				Metagraph::<T>::insert( uid_i, neuron );
			}
		}
		pub fn set_weights_from_matrix( weights: Vec<Vec<u32>> ) {
			for uid_i in 0..Self::get_neuron_count() {
				let mut sparse_weights: Vec<(u32, u32)> = vec![];
				for uid_j in 0..Self::get_neuron_count() {
					let weight_ij: u32 = weights[uid_i as usize][uid_j as usize];
					if weight_ij != 0 {
						sparse_weights.push( (uid_j, weight_ij) );
					}
				}
				let mut neuron = Metagraph::<T>::get(uid_i);
				neuron.weights = sparse_weights;
				Metagraph::<T>::insert( uid_i, neuron );
			}
		}
	
		// --- Returns Option if the u64 converts to a balance
		// use .unwarp if the result returns .some().
		pub fn u64_to_balance(input: u64) -> Option<<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance>
		{
			input.try_into().ok()
		}

		// --- Returns true if the account-id has an active
		// account on chain.
		pub fn add_hotkey_to_active_set(hotkey_id: &T::AccountId, uid: u32) {
			Hotkeys::<T>::insert(&hotkey_id, uid);
		}

		// --- Returns true if the account-id has an active
		// account on chain.
		pub fn is_hotkey_active(hotkey_id: &T::AccountId) -> bool {
			return Hotkeys::<T>::contains_key(&hotkey_id);
		}

		// --- Returns false if the account-id has an active
		// account on chain.
		pub fn is_not_active(hotkey_id: &T::AccountId) -> bool {
			return !Self::is_hotkey_active(hotkey_id);
		}

		// --- Returns true if the uid is active, i.e. there
		// is a staking, last_update, and neuron account associated
		// with this uid.
		pub fn is_uid_active(uid: u32) -> bool {
			return Metagraph::<T>::contains_key(uid);
		}

		// --- Returns hotkey associated with the hotkey account.
		// This should be called in conjunction with is_hotkey_active
		// to ensure this function does not throw an error.
		pub fn get_uid_for_hotkey(hotkey_id: &T::AccountId) -> u32{
			return Hotkeys::<T>::get(&hotkey_id);
		}

		pub fn get_neuron_for_uid ( uid: u32 ) -> NeuronMetadataOf<T> {
			return Metagraph::<T>::get( uid );
		}

		// --- Returns the neuron associated with the passed hotkey.
		// The function makes a double mapping from hotkey -> uid -> neuron.
		pub fn get_neuron_for_hotkey(hotkey_id: &T::AccountId) -> NeuronMetadataOf<T> {
			let uid = Self::get_uid_for_hotkey(hotkey_id);
			return Self::get_neuron_for_uid(uid);
		}

		// --- Returns the next available network uid.
		// uids increment up to u64:MAX, this allows the chain to
		// have 18,446,744,073,709,551,615 peers before an overflow.
		pub fn get_neuron_count() -> u32 {
			let uid = N::<T>::get();
			uid
		}

		// --- Returns the next available network uid and increments uid.
		pub fn get_next_uid() -> u32 {
			let uid = N::<T>::get();
			assert!(uid < u32::MAX);  // The system should fail if this is ever reached.
			N::<T>::put(uid + 1);
			uid
		}


	}
}



