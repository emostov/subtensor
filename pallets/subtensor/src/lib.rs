#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>

pub use pallet::*;


use frame_support::{IterableStorageMap, dispatch, ensure, traits::{
		Currency, 
		ExistenceRequirement,
		tokens::{
			WithdrawReasons
		}
	}
};

use frame_system::{
	self as system, 
	ensure_signed
};

use substrate_fixed::types::U64F64;

use sp_std::convert::TryInto;
use sp_std::vec::Vec;
use sp_std::vec;

mod weights;
mod emission;
mod staking;
mod block_reward;
mod subscribing;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{Printable, dispatch::DispatchResult, log::debug, pallet_prelude::*, traits::{Currency}};
	use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;
	use sp_std::convert::TryInto;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// --- Currency type that will be used to place deposits on neurons
		type Currency: Currency<Self::AccountId> + Send + Sync;
		
		/// - The transaction fee in RAO per byte
		type TransactionByteFee: Get<BalanceOf<Self>>;
	}

    pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
    pub type NeuronMetadataOf<T> = NeuronMetadata<AccountIdOf<T>>;
	pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

    // ---- Neuron endpoint information
    #[derive(Encode, Decode, Default)]
    pub struct NeuronMetadata<AccountId> {
        /// ---- The endpoint's u128 encoded ip address of type v6 or v4.
        pub ip: u128,

        /// ---- The endpoint's u16 encoded port.
        pub port: u16,

        /// ---- The endpoint's ip type, 4 for ipv4 and 6 for ipv6.
        pub ip_type: u8,

        /// ---- The endpoint's unique identifier. The chain can have
        /// 18,446,744,073,709,551,615 neurons before we overflow. However
        /// by this point the chain would be 10 terabytes just from metadata
        /// alone.
        pub uid: u64,

        /// ---- The neuron modality. Modalities specify which datatype
        /// the neuron endpoint can process. This information is non
        /// verifiable. However, neurons should set this correctly
        /// in order to be detected by others with this datatype.
        /// The initial modality codes are:
        /// TEXT: 0
        /// IMAGE: 1
        /// TENSOR: 2
        pub modality: u8,

        /// ---- The associated hotkey account.
        /// Subscribing, emitting and changing weights can be made by this
        /// account. Subscription can never change the associated coldkey
        /// account.
        pub hotkey: AccountId,

        /// ---- The associated coldkey account.
        /// Staking and unstaking transactions must be made by this account.
        /// The hotkey account (in the Neurons map) has permission to call emit
        /// subscribe and unsubscribe.
        pub coldkey: AccountId,
    }

	// The pallet's runtime storage items.
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

    #[pallet::storage]
	/// ---- Stores the amount of currently staked token.
    pub type TotalStake<T> = StorageValue<_, u64, ValueQuery>;

    #[pallet::storage]
	/// ---- The next uid allocated to a subscribing neuron. Or a count of how many peers
	/// have ever subscribed.
    pub type NextUID<T> = StorageValue<_, u64, ValueQuery>;

    #[pallet::storage]
    /// ---- The number of subscriptions this block, used in conjunction with ... 
    pub type SubscriptionsThisBlock<T> = StorageValue<_, u32, ValueQuery>;    

    #[pallet::storage]
    pub type LastSubscriptionBlock<T:Config> = StorageValue<_, T::BlockNumber, ValueQuery>;    

    #[pallet::storage]
	/// ---- The total amount of transaction fees accumulated during a block
    pub type TransactionFeePool<T> = StorageValue<_, u64, ValueQuery>;

    #[pallet::storage]
    /// --- The transaction fees that are added to the current block reward.
    pub type TransactionFeesForBlock<T> = StorageValue<_, u64, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn uid)]
    /// ---- Active set map between a hotkey account and network uids.
	/// Used by subtensor for checking peer existence.
    pub(super) type Active<T:Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u64, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn neuron)]
    /// ----  Maps between a neuron's hotkey account address and additional 
    /// metadata associated with that neuron. All other maps, map between the with a uid. 
    /// The metadata contains that uid, the ip, port, and coldkey address.
    pub(super) type Neurons<T> = StorageMap<_, Identity, u64, NeuronMetadataOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn last_emit)]
    /// ---- Maps between a neuron's hotkey uid and the block number
    /// when that peer last called an emission/subscribe.
    pub(super) type LastEmit<T:Config> = StorageMap<_, Identity, u64, T::BlockNumber, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn pending_emission)]
	/// --- Maps between a neuron's hotkey uid and this peer's pending emission.
	/// pending emission is the quantity 
    pub(super) type PendingEmission<T:Config> = StorageMap<_, Identity, u64, u64, ValueQuery>;

    /// ---- List of values which map between a neuron's uid an that neuron's
    /// weights, a.k.a is row_weights in the square matrix W. Each outward edge
    /// is represented by a (u64, u64) tuple determining the endpoint and weight
    /// value respectively. Each giga byte of chain storage can hold history for
    /// 83 million weights. 
    #[pallet::storage]
    pub(super) type WeightUids<T> = StorageMap<_, Identity, u64, Vec<u64>, ValueQuery>;

    #[pallet::storage]
    pub(super) type WeightVals<T> = StorageMap<_, Identity, u64, Vec<u32>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn stake)]
    pub(super) type Stake<T> = StorageMap<_, Identity, u64, u64, ValueQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig {
        pub pending_emissions: Vec<(u64, u64)>,
        pub stake: Vec<(u64, u64)>,
        //pub transaction_fee_pool: u64,
    }

	#[cfg(feature = "std")]
	impl Default for GenesisConfig {
		fn default() -> Self {
			Self {
				pending_emissions: Default::default(),
				stake: Default::default(),
				//transaction_fee_pool: Default::default(),
			}
		}
	}
    
    #[pallet::genesis_build]
    impl<T:Config> GenesisBuild<T> for GenesisConfig {
        fn build(&self) {
            for (uid, emission) in &self.pending_emissions {
                PendingEmission::<T>::insert(uid, emission);
            };

            for (uid, stake) in &self.stake {
                Stake::<T>::insert(uid, stake);
            };

            // if self.transaction_fee_pool > 0 {
            //     TransactionFeePool::<T>::put(self.transaction_fee_pool);
            // };
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

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),

        /// ---- Event created when a caller successfully set's their weights
		/// on the chain.
		WeightsSet(T::AccountId),

		/// --- Event created when a new neuron account has been subscribed to 
		/// the neuron set.
		NeuronAdded(u64),

		/// --- Event created when the neuron information associated with a hotkey
		/// is changed, for instance, when the ip/port changes.
		NeuronUpdated(u64),

		/// --- Event created during when stake has been transfered from 
		/// the coldkey onto the hotkey staking account.
		StakeAdded(T::AccountId, u64),

		/// --- Event created when stake has been removed from 
		/// the staking account into the coldkey account.
		StakeRemoved(T::AccountId, u64),

		/// --- Event created when a transaction triggers and incentive
		/// mechanism emission.
		Emission(T::AccountId, u64),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,

        /// ---- Thrown when the user tries to subscribe a neuron which is not of type
	    /// 4 (IPv4) or 6 (IPv6).
		InvalidIpType,

		/// --- Thrown when an invalid IP address is passed to the subscribe function.
		InvalidIpAddress,

		/// --- Thrown when an invalid modality attempted on subscribe.
		/// Currently the chain only accepts modality TEXT = 0.
		InvalidModality,

		/// --- Thrown when subscriptions this block have exeeded the number of 
		/// allowed.
		TooManySubscriptionsThisBlock,

		/// ---- Thrown when the caller attempts to set the weight keys
		/// and values but these vectors have different size.
		WeightVecNotEqualSize,

		/// ---- Thrown when the caller attempts to set weights with duplicate uids
		/// in the weight matrix.
		DuplicateUids,

		/// ---- Thrown when a caller attempts to set weight to at least one uid that
		/// does not exist in the metagraph.
		InvalidUid,

		/// ---- Thrown when the caller triggers an emit but the computed amount
		/// to emit is zero.
		NothingToEmit,

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
                Error::NothingToEmit => "There is nothing to emit".print(),
                Error::WeightVecNotEqualSize => "The vec of keys and the vec of values are not of the same size".print(),
                Error::NonAssociatedColdKey => "The used cold key is not associated with the hot key acccount".print(),
                _ => "Invalid Error Case".print(),
            }
        }
    }

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {

		/// ---- Called on the initialization of this pallet. (the order of on_finalize calls is determined in the runtime)
		///
		/// # Args:
		/// 	* 'n': (T::BlockNumber):
		/// 		- The number of the block we are initializing.
		fn on_initialize(_n: BlockNumberFor<T>) -> Weight {
		    //Self::move_transaction_fee_pool_to_block_reward();
			Self::update_pending_emissions()
		}


	}
    

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		
        /// --- Sets the caller weights for the incentive mechanism. The call can be
		/// made from the hotkey account so is potentially insecure, however, the damage
		/// of changing weights is minimal if caught early. This function includes all the
		/// checks that the passed weights meet the requirements. Stored as u64s they represent
		/// rational values in the range [0,1] which sum to 1 and can be interpreted as
		/// probabilities. The specific weights determine how inflation propagates outward
		/// from this peer. Because this function changes the inflation distribution it
		/// triggers an emit before values are changed on the chain.
		/// 
		/// Note: The 32 bit integers weights should represent 1.0 as the max u64.
		/// However, the function normalizes all integers to u64_max anyway. This means that if the sum of all
		/// elements is larger or smaller than the amount of elements * u64_max, all elements
		/// will be corrected for this deviation. 
		/// 
		/// # Args:
		/// 	* `origin`: (<T as frame_system::Config>Origin):
		/// 		- The caller, a hotkey who wishes to set their weights.
		/// 
		/// 	* `uids` (Vec<u64>):
		/// 		- The edge endpoint for the weight, i.e. j for w_ij.
		///
		/// 	* 'weights' (Vec<u64>):
		/// 		- The u64 integer encoded weights. Interpreted as rational
		/// 		values in the range [0,1]. They must sum to in32::MAX.
		///
		/// # Emits:
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
		pub fn set_weights(origin:OriginFor<T>, dests: Vec<u64>, weights: Vec<u32>) -> DispatchResult {
			Self::do_set_weights(origin, dests, weights)
		}
		
		/// --- Adds stake to a neuron account. The call is made from the
		/// coldkey account linked in the neurons's NeuronMetadata.
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
		/// # Emits:
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
		#[pallet::weight((0, DispatchClass::Normal, Pays::No))]
		pub fn add_stake(origin:OriginFor<T>, hotkey: T::AccountId, ammount_staked: u64) -> DispatchResult {
			Self::do_add_stake(origin, hotkey, ammount_staked)
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
		/// # Emits:
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
		#[pallet::weight((0, DispatchClass::Normal, Pays::No))]
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
		/// # Emits:
		/// 	* 'NeuronAdded':
		/// 		- On subscription of a new neuron to the active set.
		///
		/// 	* 'NeuronUpdated':
		/// 		- On subscription of new metadata attached to the calling hotkey.
		#[pallet::weight((0, DispatchClass::Normal, Pays::No))]
		pub fn subscribe(origin:OriginFor<T>, ip: u128, port: u16, ip_type: u8, modality: u8, coldkey: T::AccountId) -> DispatchResult {
			debug!("Subscribing neuron with coldkey {:?}", coldkey.clone());
			Self::do_subscribe(origin, ip, port, ip_type, modality, coldkey)
		}
	}
	

	// ---- Subtensor helper functions.
	impl<T: Config> Pallet<T> {
		// --- Returns Option if the u64 converts to a balance
		// use .unwarp if the result returns .some().
		pub fn u64_to_balance(input: u64) -> Option<<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance>
		{
			input.try_into().ok()
		}

		// --- Returns true if the account-id has an active
		// account on chain.
		pub fn add_hotkey_to_active_set(hotkey_id: &T::AccountId, uid: u64) {
			Active::<T>::insert(&hotkey_id, uid);
		}

		// --- Returns true if the account-id has an active
		// account on chain.
		pub fn is_hotkey_active(hotkey_id: &T::AccountId) -> bool {
			return Active::<T>::contains_key(&hotkey_id);
		}

		// --- Returns false if the account-id has an active
		// account on chain.
		pub fn is_not_active(hotkey_id: &T::AccountId) -> bool {
			return !Self::is_hotkey_active(hotkey_id);
		}

		// --- Returns true if the uid is active, i.e. there
		// is a staking, last_emit, and neuron account associated
		// with this uid.
		pub fn is_uid_active(uid: u64) -> bool {
			return Neurons::<T>::contains_key(uid);
		}

		// --- Returns hotkey associated with the hotkey account.
		// This should be called in conjunction with is_hotkey_active
		// to ensure this function does not throw an error.
		pub fn get_uid_for_hotkey(hotkey_id: &T::AccountId) -> u64{
			return Active::<T>::get(&hotkey_id);
		}

		// --- Returns the neuron associated with the passed uid.
		// The function makes a single mapping from uid -> neuron.
		pub fn get_neuron_for_uid(uid: u64) -> NeuronMetadataOf<T> {
			return Neurons::<T>::get(uid);
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
		pub fn get_neuron_count() -> u64 {
			let uid = NextUID::<T>::get();
			uid
		}

		// --- Returns the next available network uid.
		// uids increment up to u64:MAX, this allows the chain to
		// have 18,446,744,073,709,551,615 peers before an overflow.
		pub fn get_next_uid() -> u64 {
			let uid = NextUID::<T>::get();
			assert!(uid < u64::MAX);  // The system should fail if this is ever reached.
			NextUID::<T>::put(uid + 1);
			uid
		}


		pub fn calculate_transaction_fee(len: u64) -> u64 {
			return len * 100;
		}

		// pub fn can_pay_transaction_fee_from_coldkey_account(balance: <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance, amount: u64, transaction_fee: u64) -> bool
		// {
		// 	return balance - Self::u64_to_balance(amount).unwrap() > Self::u64_to_balance(transaction_fee).unwrap();
		// }
	}
}



/************************************************************
	CallType definition
************************************************************/

// #[derive(Debug, PartialEq)]
// pub enum CallType {
//     SetWeights,
//     AddStake,
//     RemoveStake,
//     Subscribe,
//     Other,
// }

// impl Default for CallType {
//     fn default() -> Self {
//         CallType::Other
//     }
// }


// type TransactionFee = u64;

// impl<T: Config> Pallet<T> where
// 	BalanceOf<T>: FixedPointOperand
// {
// 	/// Query the data that we know about the fee of a given `call`.
// 	///
// 	/// This module is not and cannot be aware of the internals of a signed extension, for example
// 	/// a tip. It only interprets the extrinsic as some encoded value and accounts for its weight
// 	/// and length, the runtime's extrinsic base weight, and the current fee multiplier.
// 	///
// 	/// All dispatchables must be annotated with weight and will have some fee info. This function
// 	/// always returns.
// 	pub fn query_info<Extrinsic: GetDispatchInfo>(
// 		unchecked_extrinsic: Extrinsic,
// 		_len: u32,
// 	) -> RuntimeDispatchInfo<BalanceOf<T>>
// 	where
// 		T: Send + Sync,
// 		BalanceOf<T>: Send + Sync,
// 		T::Call: Dispatchable<Info=DispatchInfo>,
// 	{
// 		// NOTE: we can actually make it understand `ChargeTransactionPayment`, but would be some
// 		// hassle for sure. We have to make it aware of the index of `ChargeTransactionPayment` in
// 		// `Extra`. Alternatively, we could actually execute the tx's per-dispatch and record the
// 		// balance of the sender before and after the pipeline.. but this is way too much hassle for
// 		// a very very little potential gain in the future.
// 		let dispatch_info = <Extrinsic as GetDispatchInfo>::get_dispatch_info(&unchecked_extrinsic);

// 	    let partial_fee = <BalanceOf<T>>::from(0u32);
// 		let DispatchInfo { weight, class, .. } = dispatch_info;

// 		RuntimeDispatchInfo { weight, class, partial_fee }
// 	}
// }



/************************************************************
	ChargeTransactionPayment definition
************************************************************/

// #[derive(Encode, Decode, Clone, Eq, PartialEq)]
// pub struct ChargeTransactionPayment<T: Config + Send + Sync>(pub PhantomData<T>);

// impl<T: Config + Send + Sync> ChargeTransactionPayment<T> where
//     T::Call: Dispatchable<Info=DispatchInfo, PostInfo=PostDispatchInfo>,
//     <T as frame_system::Config>::Call: IsSubType<Call<T>>,
// {
//     pub fn new() -> Self {
//         Self(Default::default())
//     }

//     pub fn can_pay_set_weights(who: &T::AccountId) -> Result<TransactionFee, TransactionValidityError> {
//         let transaction_fee = Pallet::<T>::get_transaction_fee_for_emission(who);
//         Ok(transaction_fee)
//     }

//     pub fn can_pay_add_stake(who: &T::AccountId, len: u64) -> Result<TransactionFee, TransactionValidityError> {
//         let transaction_fee = Pallet::<T>::calculate_transaction_fee(len as u64);
//         let transaction_fee_as_balance = Pallet::<T>::u64_to_balance(transaction_fee);

//         if Pallet::<T>::can_remove_balance_from_coldkey_account(&who, transaction_fee_as_balance.unwrap()) {
//             Ok(transaction_fee)
//         } else {
//             Err(InvalidTransaction::Payment.into())
//         }
//     }

//     pub fn can_pay_remove_stake(who: &T::AccountId, hotkey_id: &T::AccountId, len: u64) -> Result<TransactionFee, TransactionValidityError> {
//         let neuron = Pallet::<T>::get_neuron_for_hotkey(&hotkey_id);
//         let transaction_fee = Pallet::<T>::calculate_transaction_fee(len as u64);
//         let transaction_fee_as_balance = Pallet::<T>::u64_to_balance(transaction_fee).unwrap();

//         if Pallet::<T>::can_remove_balance_from_coldkey_account(&who, transaction_fee_as_balance) ||
//             Pallet::<T>::has_enough_stake(&neuron, transaction_fee) {
//             Ok(transaction_fee)
//         } else {
//             Err(InvalidTransaction::Payment.into())
//         }
//     }

//     pub fn can_pay_subscribe() -> Result<TransactionFee, TransactionValidityError> {
//         Ok(0)
//     }

//     pub fn can_pay_other(info: &DispatchInfoOf<T::Call>, who: &T::AccountId, len: u64) -> Result<TransactionFee, TransactionValidityError> {
//         let transaction_fee = Pallet::<T>::calculate_transaction_fee(len as u64);

//         if info.pays_fee == Pays::No {
//             return Ok(transaction_fee);
//         }

//         let transaction_fee_as_balance = Pallet::<T>::u64_to_balance(transaction_fee);
//         if Pallet::<T>::can_remove_balance_from_coldkey_account(&who, transaction_fee_as_balance.unwrap()) {
//             Ok(transaction_fee)
//         } else {
//             Err(InvalidTransaction::Payment.into())
//         }
//     }

//     pub fn get_priority_set_weights(transaction_fee: u64, len: u64) -> u64 {
//         // Sanity check
//         if len == 0 {
//             return 0;
//         }
//         return transaction_fee / len;
//     }

//     pub fn get_priority_vanilla() -> u64 {
//         // Just return a rediculously high priority. This means that all extrinsics exept
//         // the set_weights function will have a priority over the set_weights calls.
//         // This should probably be refined in the future.
//         return u64::max_value();
//     }
// }


// impl<T: Config + Send + Sync> sp_std::fmt::Debug for ChargeTransactionPayment<T> {
//     fn fmt(&self, f: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
//         write!(f, "ChargeTransactionPayment")
//     }
// }

// impl<T: Config + Send + Sync> SignedExtension for ChargeTransactionPayment<T>
//     where
//         T::Call: Dispatchable<Info=DispatchInfo, PostInfo=PostDispatchInfo>,
//         <T as frame_system::Config>::Call: IsSubType<Call<T>>,
// {
// 	const IDENTIFIER: &'static str = "ChargeTransactionPayment";

//     type AccountId = T::AccountId;
//     type Call = T::Call;
//     //<T as frame_system::Trait>::Call;
//     type AdditionalSigned = ();
//     type Pre = (CallType, u64, Self::AccountId);
//     fn additional_signed(&self) -> Result<Self::AdditionalSigned, TransactionValidityError> { Ok(()) }

//     fn validate(
//         &self,
//         who: &Self::AccountId,
//         call: &Self::Call,
//         info: &DispatchInfoOf<Self::Call>,
//         len: usize,
//     ) -> TransactionValidity {
//         match call.is_sub_type() {
//             Some(Call::set_weights(..)) => {
//                 let transaction_fee = Self::can_pay_set_weights(who)?;
//                 Ok(ValidTransaction {
//                     priority: Self::get_priority_set_weights(transaction_fee, len as u64),
//                     longevity: 1,
//                     ..Default::default()
//                 })
//             }
//             Some(Call::add_stake(..)) => {
//                 let _transaction_fee = Self::can_pay_add_stake(who, len as u64)?;
//                 Ok(ValidTransaction {
//                     priority: Self::get_priority_vanilla(),
//                     ..Default::default()
//                 })
//             }
//             Some(Call::remove_stake(hotkey_id, ..)) => {
//                 let _transaction_fee = Self::can_pay_remove_stake(who, hotkey_id, len as u64)?;
//                 Ok(ValidTransaction {
//                     priority: Self::get_priority_vanilla(),
//                     ..Default::default()
//                 })
//             }
//             Some(Call::subscribe(..)) => {
//                 let _transaction_fee = Self::can_pay_subscribe()?;
//                 Ok(ValidTransaction {
//                     priority: Self::get_priority_vanilla(),
//                     ..Default::default()
//                 })
//             }
//             _ => {
//                 let _transaction_fee = Self::can_pay_other(info, who, len as u64)?;
//                 Ok(ValidTransaction {
//                     priority: Self::get_priority_vanilla(),
//                     ..Default::default()
//                 })
//             }
//         }
//     }

//     // NOTE: Add later when we put in a pre and post dispatch step.
//     fn pre_dispatch(
//         self,
//         who: &Self::AccountId,
//         call: &Self::Call,
//         info: &DispatchInfoOf<Self::Call>,
//         len: usize,
//     ) -> Result<Self::Pre, TransactionValidityError> {

//         //debug::info!(&("PRE DISPATCH: Transaction length: {:?}", len));

//         match call.is_sub_type() {
//             Some(Call::set_weights(..)) => {
//                 // To pay for the set_weights operation, the self_weight of a neuron is used for payment
//                 // This can be >= 0, however the lower the self weight, the lower the priority in the block
//                 // and may result the transaction is not put into a block
//                 let transaction_fee = Self::can_pay_set_weights(who)?;
//                 Ok((CallType::SetWeights, transaction_fee, who.clone())) // 0 indicates that post_dispatch should use the self-weight to pay for the transaction
//             }
//             Some(Call::add_stake(..)) => {
//                 // The transaction fee for the add_stake function is paid from the coldkey balance
//                 // let transaction_fee = Module::<T>::calculate_transaction_fee(len as u64);
//                 // let transaction_fee_as_balance = Module::<T>::u64_to_balance( transaction_fee );
//                 let transaction_fee = Self::can_pay_add_stake(who, len as u64)?;
//                 Ok((CallType::AddStake, transaction_fee, who.clone()))
//             }
//             Some(Call::remove_stake(hotkey_id, ..)) => {
//                 // The tranaction fee for the remove_stake call is paid from the coldkey balance
//                 // after the transaction completes. For this, a check is done on both the stake
//                 // as well as the coldkey balance to see if one of both is sufficient to pay
//                 // for the transaction

//                 let transaction_fee = Self::can_pay_remove_stake(who, hotkey_id, len as u64)?;
//                 Ok((CallType::RemoveStake, transaction_fee, who.clone()))
//             }
//             Some(Call::subscribe(..)) => {
//                 let transaction_fee = Self::can_pay_subscribe()?;
//                 Ok((CallType::Subscribe, transaction_fee, who.clone()))
//             }
//             _ => {
//                 let transaction_fee = Self::can_pay_other(info, who, len as u64)?;
//                 Ok((CallType::Other, transaction_fee, who.clone()))
//             }
//         }
//     }

//     fn post_dispatch(
//         pre: Self::Pre,
//         info: &DispatchInfoOf<Self::Call>,
//         _post_info: &PostDispatchInfoOf<Self::Call>,
//         _len: usize,
//         result: &dispatch::DispatchResult,
//     ) -> Result<(), TransactionValidityError> {
//         let call_type = pre.0;
//         let transaction_fee = pre.1;
//         let account_id = pre.2;
//         let transaction_fee_as_balance = Pallet::<T>::u64_to_balance(transaction_fee).unwrap();

//         match result {
//             Ok(_) => {
//                 match call_type {
//                     CallType::SetWeights => {
//                         // account_id = hotkey_id, since this method is called with the hotkey
//                         let uid = Pallet::<T>::get_uid_for_hotkey(&account_id);
//                         Pallet::<T>::remove_stake_from_neuron_hotkey_account(uid, transaction_fee);
//                         Pallet::<T>::update_transaction_fee_pool(transaction_fee);
//                         Ok(Default::default())
//                     }
//                     CallType::AddStake => {
//                         // account_id = coldkey_id, since this method is called with the coldkey
//                         Pallet::<T>::remove_balance_from_coldkey_account(&account_id, transaction_fee_as_balance);
//                         Pallet::<T>::update_transaction_fee_pool(transaction_fee); // uid 0 == Adam
//                         Ok(Default::default())
//                     }
//                     CallType::RemoveStake => {
//                         // account_id = coldkey_id, since this method is called with the coldkey
//                         Pallet::<T>::remove_balance_from_coldkey_account(&account_id, transaction_fee_as_balance);
//                         Pallet::<T>::update_transaction_fee_pool(transaction_fee); // uid 0 == Adam
//                         Ok(Default::default())
//                     }
//                     CallType::Subscribe => {
//                         Ok(Default::default())
//                     }
//                     _ => {
//                         // Default behaviour for calls not otherwise specified
//                         match info.pays_fee {
//                             Pays::No => Ok(Default::default()),
//                             Pays::Yes => {
//                                 Pallet::<T>::remove_balance_from_coldkey_account(&account_id, transaction_fee_as_balance);
//                                 Pallet::<T>::update_transaction_fee_pool(transaction_fee); // uid 0 == Adam
//                                 Ok(Default::default())
//                             }
//                         }
//                     }
//                 }
//             }
//             Err(_) => Ok(Default::default())
//         }
//     }
// }
