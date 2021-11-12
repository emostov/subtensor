#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>

/// ************************************************************
/// -Substrate-Imports
/// ************************************************************
pub use pallet::*;

use codec::{Decode, Encode};
use frame_support::{dispatch, ensure, traits::{
		Currency, 
		ExistenceRequirement,
		IsSubType, 
		tokens::{
			WithdrawReasons
		}
	}, weights::{
		DispatchInfo, 
		PostDispatchInfo, 
		Pays
	}
};

use frame_support::sp_runtime::FixedPointOperand;
use frame_support::dispatch::GetDispatchInfo;
use frame_support::sp_runtime::transaction_validity::ValidTransaction;
use frame_system::{
	self as system, 
	ensure_signed
};

use pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo;
use substrate_fixed::types::U64F64;
use sp_runtime::{
	traits::{
		Dispatchable, 
		DispatchInfoOf, 
		SignedExtension, 
		PostDispatchInfoOf,
	},
	transaction_validity::{
        TransactionValidityError, 
		TransactionValidity, 
		InvalidTransaction,
    }
};
use sp_std::vec::Vec;
use sp_std::vec;
use sp_std::marker::PhantomData;

/// ************************************************************
///	-Subtensor-Imports
/// ************************************************************
mod weights;
mod staking;
mod serving;
mod step;
mod registration;

#[frame_support::pallet]
pub mod pallet {
	use sp_core::{U256};
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*, Printable, traits::{Currency}};
	use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;
	use sp_std::vec;
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

		/// --- Currency type that will be used to place deposits on neurons
		type Currency: Currency<Self::AccountId> + Send + Sync;
		
		/// --- The transaction fee in RAO per byte
		type TransactionByteFee: Get<BalanceOf<Self>>;

		/// Debug is on
		#[pallet::constant]
		type SDebug: Get<u64>;

		/// Activity constant
		#[pallet::constant]
		type StepRho: Get<u64>;

		/// Activity constant
		#[pallet::constant]
		type StepKappa: Get<u64>;

		/// Activity constant
		#[pallet::constant]
		type SelfOwnership: Get<u64>;

		/// Activity constant
		#[pallet::constant]
		type InitialActivityCutoff: Get<u64>;

		/// Initial registration difficulty.
		#[pallet::constant]
		type InitialIssuance: Get<u64>;

		/// Initial registration difficulty.
		#[pallet::constant]
		type InitialDifficulty: Get<u64>;

		/// Minimum registration difficulty
		#[pallet::constant]
		type MinimumDifficulty: Get<u64>;

		/// Maximum registration difficulty
		#[pallet::constant]
		type MaximumDifficulty: Get<u64>;

		/// Initial adjustment interval.
		#[pallet::constant]
		type InitialAdjustmentInterval: Get<u64>;

		/// Initial max registrations per block.
		#[pallet::constant]
		type InitialMaxRegistrationsPerBlock: Get<u64>;

		/// Initial target registrations per interval.
		#[pallet::constant]
		type InitialTargetRegistrationsPerInterval: Get<u64>;
	}

	/// ************************************************************
	///	-Pallet-Types
	/// ************************************************************
	/// Subtensor custom types.
	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub type NeuronMetadataOf<T> = NeuronMetadata<AccountIdOf<T>>;
	pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
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
        /// verifiable. However, neurons should set this correctly
        /// in order to be detected by others with this datatype.
        /// The initial modality codes are:
        /// TEXT: 0
        /// IMAGE: 1
        /// TENSOR: 2
        pub modality: u8,

        /// ---- The associated hotkey account.
        /// Registration and changing weights can be made by this
        /// account.
        pub hotkey: AccountId,

        /// ---- The associated coldkey account.
        /// Staking and unstaking transactions must be made by this account.
        /// The hotkey account (in the Neurons map) has permission to call
        /// subscribe and unsubscribe.
        pub coldkey: AccountId,

		/// ---- Is this neuron active in the incentive mechanism.
		pub active: u32,

		/// ---- Block number of last chain update.
		pub last_update: u64,

		/// ---- Transaction priority.
		pub priority: u64,

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

		/// ---- The associated dividends in this account.
		pub dividends: u64,

		/// ---- The associated emission last block for this account.
		pub emission: u64,

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
	pub type TotalEmission<T> = StorageValue<
		_, 
		u64, 
		ValueQuery
	>;

	#[pallet::storage]
	pub type TotalBondsPurchased<T> = StorageValue<
		_, 
		u64, 
		ValueQuery
	>;


	#[pallet::type_value] 
	pub fn DefaultTotalIssuance<T: Config>() -> u64 { T::InitialIssuance::get() }
	#[pallet::storage]
	pub type TotalIssuance<T> = StorageValue<
		_, 
		u64, 
		ValueQuery,
		DefaultTotalIssuance<T>
	>;

	#[pallet::type_value] 
	pub fn DefaultDifficulty<T: Config>() -> u64 { T::InitialDifficulty::get() }
	#[pallet::storage]
	pub type Difficulty<T> = StorageValue<
		_, 
		u64, 
		ValueQuery,
		DefaultDifficulty<T>
	>;

	#[pallet::type_value] 
	pub fn DefaultActivityCutoff<T: Config>() -> u64 { T::InitialActivityCutoff::get() }
	#[pallet::storage]
	pub type ActivityCutoff<T> = StorageValue<
		_, 
		u64, 
		ValueQuery,
		DefaultActivityCutoff<T>
	>;

	#[pallet::type_value] 
	pub fn DefaultAdjustmentInterval<T: Config>() -> u64 { T::InitialAdjustmentInterval::get() }
	#[pallet::storage]
	pub type AdjustmentInterval<T> = StorageValue<
		_, 
		u64, 
		ValueQuery,
		DefaultAdjustmentInterval<T>
	>;

	#[pallet::type_value] 
	pub fn DefaultTargetRegistrationsPerInterval<T: Config>() -> u64 { T::InitialTargetRegistrationsPerInterval::get() }
	#[pallet::storage]
	pub type TargetRegistrationsPerInterval<T> = StorageValue<
		_, 
		u64, 
		ValueQuery,
		DefaultTargetRegistrationsPerInterval<T>
	>;

	#[pallet::type_value] 
	pub fn DefaultMaxRegistrationsPerBlock<T: Config>() -> u64 { T::InitialMaxRegistrationsPerBlock::get() }
	#[pallet::storage]
	pub type MaxRegistrationsPerBlock<T> = StorageValue<
		_, 
		u64, 
		ValueQuery,
		DefaultMaxRegistrationsPerBlock<T>
	>;

	#[pallet::storage]
	pub type LastDifficultyAdjustmentBlock<T> = StorageValue<
		_, 
		u64, 
		ValueQuery
	>;


	#[pallet::storage]
	pub type RegistrationsThisInterval<T> = StorageValue<
		_, 
		u64, 
		ValueQuery
	>;

	#[pallet::storage]
	pub type RegistrationsThisBlock<T> = StorageValue<
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
    pub(super) type Neurons<T:Config> = StorageMap<
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

		/// --- Event created when a new neuron account has been registered to 
		/// the chain.
		NeuronRegistered(u32),

		/// --- Event created when the axon server information is added to the network.
		AxonServed(u32),

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
        /// ---- Thrown when the user tries to serve an axon which is not of type
	    /// 4 (IPv4) or 6 (IPv6).
		InvalidIpType,

		/// --- Thrown when an invalid IP address is passed to the serve function.
		InvalidIpAddress,

		/// --- Thrown when an invalid modality attempted on serve.
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

		/// ---- Thrown if the supplied pow hash block is in the future or negative
		InvalidWorkBlock,

		/// ---- Thrown if the supplied pow hash block does not meet the network difficulty.
		InvalidDifficulty,

		/// ---- Thrown if the supplied pow hash seal does not match the supplied work.
		InvalidSeal,

		/// ---- Thrown when registrations this block exceeds allowed number.
		ToManyRegistrationsThisBlock,

		/// ---- Thrown when the caller requests setting or removing data from
		/// a neuron which does not exist in the active set.
		NotRegistered,

		/// ---- Thrown when the caller requests registering a neuron which 
		/// already exists in the active set.
		AlreadyRegistered,

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
                Error::AlreadyRegistered => "The node with the supplied public key is already registered".print(),
                Error::NotRegistered  => "The node with the supplied public key is not registered".print(),
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
			Self::update_difficulty();
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
		pub fn set_weights(
			origin:OriginFor<T>, 
			dests: Vec<u32>, 
			weights: Vec<u32>
		) -> DispatchResult {
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
		/// # Event:
		/// 	* 'StakeAdded':
		/// 		- On the successful staking of funds.
		///
		/// # Raises:
		/// 	* 'NotRegistered':
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
		pub fn add_stake(
			origin:OriginFor<T>, 
			hotkey: T::AccountId, 
			ammount_staked: u64
		) -> DispatchResult {
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
		#[pallet::weight((0, DispatchClass::Normal, Pays::No))]
		pub fn remove_stake(
			origin:OriginFor<T>, 
			hotkey: T::AccountId, 
			ammount_unstaked: u64
		) -> DispatchResult {
			Self::do_remove_stake(origin, hotkey, ammount_unstaked)
		}

		/// ---- Serves or updates axon information for the neuron associated with the caller. If the caller
		/// already registered the metadata is updated. If the caller is not registered this call throws NotRegsitered.
		///
		/// # Args:
		/// 	* 'origin': (<T as frame_system::Config>Origin):
		/// 		- The caller, a hotkey associated of the registered neuron.
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
		/// # Event:
		/// 	* 'AxonServed':
		/// 		- On subscription of a new neuron to the active set.
		///
		#[pallet::weight((0, DispatchClass::Normal, Pays::No))]
		pub fn serve_axon (
			origin:OriginFor<T>, 
			version: u32, 
			ip: u128, 
			port: u16, 
			ip_type: u8, 
			modality: u8 
		) -> DispatchResult {
			Self::do_serve_axon( origin, version, ip, port, ip_type, modality )
		}

		/// ---- Registers a new neuron to the graph. 
		///
		/// # Args:
		/// 	* 'origin': (<T as frame_system::Config>Origin):
		/// 		- The caller, registration key as found in RegistrationKey::get(0);
		///
		/// 	* 'block_number' (u64):
		/// 		- Block number of hash to attempt.
		///
		/// 	* 'nonce' (u64):
		/// 		- Hashing nonce as a u64.
		///
		/// 	* 'work' (Vec<u8>):
		/// 		- Work hash as list of bytes.
		/// 
		/// 	* 'hotkey' (T::AccountId,):
		/// 		- Hotkey to register.
		/// 
		/// 	* 'coldkey' (T::AccountId,):
		/// 		- Coldkey to register.
		///
		/// # Event:
		/// 	* 'NeuronRegistered':
		/// 		- On subscription of a new neuron to the active set.
		///
		#[pallet::weight((0, DispatchClass::Normal, Pays::No))]
		pub fn register( 
				origin:OriginFor<T>, 
				block_number: u64, 
				nonce: u64, 
				work: Vec<u8>,
				hotkey: T::AccountId, 
				coldkey: T::AccountId 
		) -> DispatchResult {
			Self::do_registration(origin, block_number, nonce, work, hotkey, coldkey)
		}

	}
	
	// ---- Subtensor helper functions.
	impl<T: Config> Pallet<T> {

		// TURN ON DEBUG
		pub fn debug() -> bool {
			return T::SDebug::get() == 1
		}

		// Adjustable Constants.
		// -- Difficulty.
		pub fn get_difficulty( ) -> U256 {
			return U256::from( Self::get_difficulty_as_u64() );
		}
		pub fn get_difficulty_as_u64( ) -> u64 {
			Difficulty::<T>::get()
		}
		pub fn set_difficulty_from_u64( difficulty: u64 ) {
			Difficulty::<T>::set( difficulty );
		}
		// -- Activity cuttoff
		pub fn get_activity_cutoff( ) -> u64 {
			return ActivityCutoff::<T>::get();
		}
		pub fn set_activity_cutoff( cuttoff: u64 ) {
			ActivityCutoff::<T>::set( cuttoff );
		}
		// -- Adjustment Interval.
		pub fn get_adjustment_interval() -> u64 {
			AdjustmentInterval::<T>::get()
		}
		pub fn set_adjustment_interval( interval: u64 ) {
			AdjustmentInterval::<T>::put( interval );
		}
		// -- Target registrations per interval.
		pub fn get_target_registrations_per_interval() -> u64 {
			TargetRegistrationsPerInterval::<T>::get()
		}
		pub fn set_target_registrations_per_interval( target: u64 ) {
			TargetRegistrationsPerInterval::<T>::put( target );
		}
		pub fn get_max_registratations_per_block( ) -> u64 {
			MaxRegistrationsPerBlock::<T>::get()
		}
		pub fn set_max_registratations_per_block( max_registrations: u64 ){
			MaxRegistrationsPerBlock::<T>::put( max_registrations );
		}
		// -- Minimum difficulty
		pub fn get_minimum_difficulty( ) -> u64 {
			return T::MinimumDifficulty::get();
		}
		// -- Maximum difficulty
		pub fn get_maximum_difficulty( ) -> u64 {
			return T::MaximumDifficulty::get();
		}
		// -- Get Block emission.
		pub fn get_block_emission( ) -> u64 {
			return 1000000000;
		}
		// -- Get step consensus temperature (rho)
		pub fn get_rho( ) -> u64 {
			return T::StepRho::get();
		}
		// -- Get step consensus shift (1/kappa)
		pub fn get_kappa( ) -> u64 {
			return T::StepKappa::get();
		}
		// -- Get self ownership proportion denominator
		pub fn get_self_ownership( ) -> u64 {
			return T::SelfOwnership::get();
		}

		// Variable Parameters
		pub fn get_registrations_this_interval( ) -> u64 {
			RegistrationsThisInterval::<T>::get()
		}
		pub fn get_registrations_this_block( ) -> u64 {
			RegistrationsThisBlock::<T>::get()
		}
		pub fn get_total_stake( ) -> u64 {
			return TotalStake::<T>::get();
		}
		pub fn get_total_issuance( ) -> u64 {
			return TotalIssuance::<T>::get();
		}
		pub fn get_initial_total_issuance( ) -> u64 {
			return T::InitialIssuance::get();
		}
		pub fn get_lastupdate( ) -> Vec<u64> {
			let mut result: Vec<u64> = vec![ 0; Self::get_neuron_count() as usize ];
			for ( uid_i, neuron_i ) in <Neurons<T> as IterableStorageMap<u32, NeuronMetadataOf<T>>>::iter() {
				result[ uid_i as usize ] = neuron_i.last_update;
			}
			return result
		}
		pub fn get_stake( ) -> Vec<u64> {
			let mut result: Vec<u64> = vec![ 0; Self::get_neuron_count() as usize ];
			for ( uid_i, neuron_i ) in <Neurons<T> as IterableStorageMap<u32, NeuronMetadataOf<T>>>::iter() {
				result[ uid_i as usize ] = neuron_i.stake;
			}
			return result
		}
		pub fn get_ranks( ) -> Vec<u64> {
			let mut result: Vec<u64> = vec![ 0; Self::get_neuron_count() as usize ];
			for ( uid_i, neuron_i ) in <Neurons<T> as IterableStorageMap<u32, NeuronMetadataOf<T>>>::iter() {
				result[ uid_i as usize ] = neuron_i.rank;
			}
			return result
		}
		pub fn get_trust( ) -> Vec<u64> {
			let mut result: Vec<u64> = vec![ 0; Self::get_neuron_count() as usize ];
			for ( uid_i, neuron_i ) in <Neurons<T> as IterableStorageMap<u32, NeuronMetadataOf<T>>>::iter() {
				result[ uid_i as usize ] = neuron_i.trust;
			}
			return result
		}
		pub fn get_consensus( ) -> Vec<u64> {
			let mut result: Vec<u64> = vec![ 0; Self::get_neuron_count() as usize ];
			for ( uid_i, neuron_i ) in <Neurons<T> as IterableStorageMap<u32, NeuronMetadataOf<T>>>::iter() {
				result[ uid_i as usize ] = neuron_i.consensus;
			}
			return result
		}
		pub fn get_incentive( ) -> Vec<u64> {
			let mut result: Vec<u64> = vec![ 0; Self::get_neuron_count() as usize ];
			for ( uid_i, neuron_i ) in <Neurons<T> as IterableStorageMap<u32, NeuronMetadataOf<T>>>::iter() {
				result[ uid_i as usize ] = neuron_i.incentive;
			}
			return result
		}
		pub fn get_dividends( ) -> Vec<u64> {
			let mut result: Vec<u64> = vec![ 0; Self::get_neuron_count() as usize ];
			for ( uid_i, neuron_i ) in <Neurons<T> as IterableStorageMap<u32, NeuronMetadataOf<T>>>::iter() {
				result[ uid_i as usize] = neuron_i.dividends;
			}
			return result
		}
		pub fn get_emission( ) -> Vec<u64> {
			let mut result: Vec<u64> = vec![ 0; Self::get_neuron_count() as usize ];
			for ( uid_i, neuron_i ) in <Neurons<T> as IterableStorageMap<u32, NeuronMetadataOf<T>>>::iter() {
				result[ uid_i as usize ] = neuron_i.emission;
			}
			return result
		}
		pub fn get_active( ) -> Vec<u32> {
			let mut result: Vec<u32> = vec![ 0; Self::get_neuron_count() as usize ];
			for ( uid_i, neuron_i ) in <Neurons<T> as IterableStorageMap<u32, NeuronMetadataOf<T>>>::iter() {
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
			for ( uid_i, neuron_i ) in <Neurons<T> as IterableStorageMap<u32, NeuronMetadataOf<T>>>::iter() {
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
			for ( uid_i, neuron_i ) in <Neurons<T> as IterableStorageMap<u32, NeuronMetadataOf<T>>>::iter() {
				weights[ uid_i as usize ] = Self::get_weights_for_neuron( &neuron_i );
			}
			return weights
		}		

		// Setters
		pub fn set_stake_from_vector( stake: Vec<u64> ) {
			let mut total_stake: u64 = 0;
			for uid_i in 0..Self::get_neuron_count() {
				let mut neuron = Neurons::<T>::get(uid_i);
				neuron.stake = stake[ uid_i as usize ];
				Neurons::<T>::insert( uid_i, neuron );
				total_stake += stake[ uid_i as usize ];
			}
			TotalStake::<T>::set( total_stake );
		}
		pub fn set_last_update_from_vector( last_update: Vec<u64> ) {
			for uid_i in 0..Self::get_neuron_count() {
				let mut neuron = Neurons::<T>::get(uid_i);
				neuron.last_update = last_update[ uid_i as usize ];
				Neurons::<T>::insert( uid_i, neuron );
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
				let mut neuron = Neurons::<T>::get(uid_i);
				neuron.weights = sparse_weights;
				Neurons::<T>::insert( uid_i, neuron );
			}
		}
	
		// Helpers.
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
			return Neurons::<T>::contains_key(uid);
		}

		// --- Returns hotkey associated with the hotkey account.
		// This should be called in conjunction with is_hotkey_active
		// to ensure this function does not throw an error.
		pub fn get_uid_for_hotkey(hotkey_id: &T::AccountId) -> u32{
			return Hotkeys::<T>::get(&hotkey_id);
		}
		pub fn get_neuron_for_uid ( uid: u32 ) -> NeuronMetadataOf<T> {
			return Neurons::<T>::get( uid );
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

		// --- Returns a vanilla transaction fee for transactions as rao.
		pub fn calculate_transaction_fee(len: u64) -> u64 {
			return len * 100;
		}

		// --- Returns the transaction priority for setting weights.
		pub fn get_priority_set_weights( hotkey: &T::AccountId, len: u64 ) -> u64 {
			if Hotkeys::<T>::contains_key( hotkey ) {
				let uid = Hotkeys::<T>::get( hotkey );
				let neuron = Neurons::<T>::get( uid );
				return neuron.priority / len;
			} else{
				return 0;
			}
		}

	}
}


/************************************************************
	CallType definition
************************************************************/
#[derive(Debug, PartialEq)]
pub enum CallType {
    SetWeights,
    AddStake,
    RemoveStake,
    Register,
    Serve,
	Other,
}
impl Default for CallType {
    fn default() -> Self {
        CallType::Other
    }
}


type TransactionFee = u64;
impl<T: Config> Pallet<T> where BalanceOf<T>: FixedPointOperand
{
	/// Query the data that we know about the fee of a given `call`.
	///
	/// This module is not and cannot be aware of the internals of a signed extension, for example
	/// a tip. It only interprets the extrinsic as some encoded value and accounts for its weight
	/// and length, the runtime's extrinsic base weight, and the current fee multiplier.
	///
	/// All dispatchables must be annotated with weight and will have some fee info. This function
	/// always returns.
	pub fn query_info<Extrinsic: GetDispatchInfo>(
		unchecked_extrinsic: Extrinsic,
		_len: u32,
	) -> RuntimeDispatchInfo<BalanceOf<T>>
	where
		T: Send + Sync,
		BalanceOf<T>: Send + Sync,
		T::Call: Dispatchable<Info=DispatchInfo>,
	{
		// NOTE: we can actually make it understand `ChargeTransactionPayment`, but would be some
		// hassle for sure. We have to make it aware of the index of `ChargeTransactionPayment` in
		// `Extra`. Alternatively, we could actually execute the tx's per-dispatch and record the
		// balance of the sender before and after the pipeline.. but this is way too much hassle for
		// a very very little potential gain in the future.
		let dispatch_info = <Extrinsic as GetDispatchInfo>::get_dispatch_info(&unchecked_extrinsic);
	    let partial_fee = <BalanceOf<T>>::from(0u32);
		let DispatchInfo { weight, class, .. } = dispatch_info;
		RuntimeDispatchInfo { weight, class, partial_fee }
	}
}



/************************************************************
	ChargeTransactionPayment definition
************************************************************/

#[derive(Encode, Decode, Clone, Eq, PartialEq)]
pub struct ChargeTransactionPayment<T: Config + Send + Sync>(pub PhantomData<T>);

impl<T: Config + Send + Sync> ChargeTransactionPayment<T> where
    T::Call: Dispatchable<Info=DispatchInfo, PostInfo=PostDispatchInfo>,
    <T as frame_system::Config>::Call: IsSubType<Call<T>>,
{
    pub fn new() -> Self {
        Self(Default::default())
	}

    pub fn can_pay_add_stake(who: &T::AccountId, len: u64) -> Result<TransactionFee, TransactionValidityError> {
        let transaction_fee = Pallet::<T>::calculate_transaction_fee(len as u64);
        let transaction_fee_as_balance = Pallet::<T>::u64_to_balance(transaction_fee);

        if Pallet::<T>::can_remove_balance_from_coldkey_account(&who, transaction_fee_as_balance.unwrap()) {
            Ok(transaction_fee)
        } else {
            Err(InvalidTransaction::Payment.into())
        }
    }

    pub fn can_pay_remove_stake(who: &T::AccountId, hotkey_id: &T::AccountId, len: u64) -> Result<TransactionFee, TransactionValidityError> {
        let neuron = Pallet::<T>::get_neuron_for_hotkey(&hotkey_id);
        let transaction_fee = Pallet::<T>::calculate_transaction_fee(len as u64);
        let transaction_fee_as_balance = Pallet::<T>::u64_to_balance(transaction_fee).unwrap();

        if Pallet::<T>::can_remove_balance_from_coldkey_account(&who, transaction_fee_as_balance) ||
            Pallet::<T>::has_enough_stake(&neuron, transaction_fee) {
            Ok(transaction_fee)
        } else {
            Err(InvalidTransaction::Payment.into())
        }
    }

    pub fn can_pay_other(info: &DispatchInfoOf<T::Call>, who: &T::AccountId, len: u64) -> Result<TransactionFee, TransactionValidityError> {
        let transaction_fee = Pallet::<T>::calculate_transaction_fee(len as u64);

        if info.pays_fee == Pays::No {
            return Ok(transaction_fee);
        }

        let transaction_fee_as_balance = Pallet::<T>::u64_to_balance(transaction_fee);
        if Pallet::<T>::can_remove_balance_from_coldkey_account(&who, transaction_fee_as_balance.unwrap()) {
            Ok(transaction_fee)
        } else {
            Err(InvalidTransaction::Payment.into())
        }
    }

    pub fn get_priority_vanilla() -> u64 {
        // Just return a rediculously high priority. This means that all extrinsics exept
        // the set_weights function will have a priority over the set_weights calls.
        // This should probably be refined in the future.
        return u64::max_value();
    }

	pub fn get_priority_set_weights( who: &T::AccountId, len: u64 ) -> u64 {
        return Pallet::<T>::get_priority_set_weights( who, len );
    }
}

impl<T: Config + Send + Sync> sp_std::fmt::Debug for ChargeTransactionPayment<T> {
    fn fmt(&self, f: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
        write!(f, "ChargeTransactionPayment")
    }
}

impl<T: Config + Send + Sync> SignedExtension for ChargeTransactionPayment<T>
    where
        T::Call: Dispatchable<Info=DispatchInfo, PostInfo=PostDispatchInfo>,
        <T as frame_system::Config>::Call: IsSubType<Call<T>>,
{
	const IDENTIFIER: &'static str = "ChargeTransactionPayment";

    type AccountId = T::AccountId;
    type Call = T::Call;
    //<T as frame_system::Trait>::Call;
    type AdditionalSigned = ();
    type Pre = (CallType, u64, Self::AccountId);
    fn additional_signed(&self) -> Result<Self::AdditionalSigned, TransactionValidityError> { Ok(()) }

    fn validate(
        &self,
        who: &Self::AccountId,
        call: &Self::Call,
        info: &DispatchInfoOf<Self::Call>,
        len: usize,
    ) -> TransactionValidity {
        match call.is_sub_type() {
            Some(Call::set_weights(..)) => {
				let priority: u64 = Self::get_priority_set_weights(who, len as u64);
                Ok(ValidTransaction {
                    priority: priority,
                    longevity: 1,
                    ..Default::default()
                })
            }
            Some(Call::add_stake(..)) => {
                // let _transaction_fee = Self::can_pay_add_stake(who, len as u64)?;
                Ok(ValidTransaction {
                    priority: Self::get_priority_vanilla(),
                    ..Default::default()
                })
            }
            Some(Call::remove_stake(..)) => {
                Ok(ValidTransaction {
                    priority: Self::get_priority_vanilla(),
                    ..Default::default()
                })
            }
            Some(Call::register(..)) => {
                // let _transaction_fee = Self::can_pay_subscribe()?;
                Ok(ValidTransaction {
                    priority: Self::get_priority_vanilla(),
                    ..Default::default()
                })
            }
            _ => {
                let _transaction_fee = Self::can_pay_other(info, who, len as u64)?;
                Ok(ValidTransaction {
                    priority: Self::get_priority_vanilla(),
                    ..Default::default()
                })
            }
        }
    }

    // NOTE: Add later when we put in a pre and post dispatch step.
    fn pre_dispatch(
        self,
        who: &Self::AccountId,
        call: &Self::Call,
        info: &DispatchInfoOf<Self::Call>,
        len: usize,
    ) -> Result<Self::Pre, TransactionValidityError> {

        match call.is_sub_type() {
            Some(Call::add_stake(..)) => {
				let transaction_fee = 0;
                Ok((CallType::AddStake, transaction_fee, who.clone()))
            }
            Some(Call::remove_stake(..)) => {
				let transaction_fee = 0;
                Ok((CallType::RemoveStake, transaction_fee, who.clone()))
            }
			Some(Call::set_weights(..)) => {
				let transaction_fee = 0;
                Ok((CallType::SetWeights, transaction_fee, who.clone())) // 0 indicates that post_dispatch should use the self-weight to pay for the transaction
            }
			Some(Call::register(..)) => {
                let transaction_fee = 0;
                Ok((CallType::Serve, transaction_fee, who.clone()))
            }
            Some(Call::serve_axon(..)) => {
                let transaction_fee = 0;
                Ok((CallType::Serve, transaction_fee, who.clone()))
            }
            _ => {
                let transaction_fee = Self::can_pay_other(info, who, len as u64)?;
                Ok((CallType::Other, transaction_fee, who.clone()))
            }
        }
    }

    fn post_dispatch(
        pre: Self::Pre,
        info: &DispatchInfoOf<Self::Call>,
        _post_info: &PostDispatchInfoOf<Self::Call>,
        _len: usize,
        result: &dispatch::DispatchResult,
    ) -> Result<(), TransactionValidityError> {
        let call_type = pre.0;
        let transaction_fee = pre.1;
        let account_id = pre.2;
        let transaction_fee_as_balance = Pallet::<T>::u64_to_balance(transaction_fee).unwrap();

        match result {
            Ok(_) => {
                match call_type {
                    CallType::SetWeights => {
                        Ok(Default::default())
                    }
                    CallType::AddStake => {
                        Ok(Default::default())
                    }
                    CallType::RemoveStake => {
                        Ok(Default::default())
                    }
                    CallType::Register => {
                        Ok(Default::default())
                    }
                    _ => {
                        // Default behaviour for calls not otherwise specified
                        match info.pays_fee {
                            Pays::No => Ok(Default::default()),
                            Pays::Yes => {
                                Pallet::<T>::remove_balance_from_coldkey_account(&account_id, transaction_fee_as_balance);
                                // Pallet::<T>::update_transaction_fee_pool(transaction_fee); // uid 0 == Adam
                                Ok(Default::default())
                            }
                        }
                    }
                }
            }
            Err(_) => Ok(Default::default())
        }
    }
}
