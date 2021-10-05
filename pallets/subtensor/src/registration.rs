use super::*;
use frame_system::{ensure_signed, ensure_root};

impl<T: Config> Pallet<T> {

    pub fn set_registration_auth( origin: T::Origin, registration_key: T::AccountId ) -> dispatch::DispatchResult {
        ensure_root(origin)?;
        RegistrationKey::<T>::insert(1, registration_key.clone() );
        Self::deposit_event(Event::RegistrationKeySet( registration_key ));
        Ok(())
    }

    pub fn do_registration( origin: T::Origin, email_hash: Vec<u8>, hotkey: T::AccountId, coldkey: T::AccountId ) -> dispatch::DispatchResult {

        // Check origin is the authorized registration key.
        let register_key = ensure_signed(origin)?;
        ensure!( RegistrationKey::<T>::contains_key(1), Error::<T>::RegistrationDisabled );

        let authorized_registration_key: T::AccountId = RegistrationKey::<T>::get(0);
        ensure!( register_key == authorized_registration_key, Error::<T>::NonAuthorizedRegistrationKey);
        
        // Check that the hotkey has not already been registered.
        ensure!( !Hotkeys::<T>::contains_key(&hotkey), Error::<T>::AlreadyRegistered );

        // Check that the email hash is the correct length.
        ensure! ( email_hash.len() == 32, Error::<T>::InvalidEmailHash );

        // We check that the user is not exceeding their registration limit.
        let mut email_hotkeys: Vec<T::AccountId> = EmailHashes::<T>::get( &email_hash );
        ensure! ( email_hotkeys.len() as u32 <= Self::get_max_registrations_per_email(), Error::<T>::MaxRegistrationsReached );

        // We extend the hotkey vector with this passed hotkey.
        email_hotkeys.push( hotkey.clone() );

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
            stake: 0,
            rank: 0,
            trust: 0,
            consensus: 0,
            incentive: 0,
            inflation: 0,
            dividends: 0,
            bonds: vec![],
            weights: vec![(uid, u32::MAX)], // self weight set to 1.
        };
        
        // --- We deposit the neuron registered event.
        EmailHashes::<T>::insert( email_hash, email_hotkeys ); // Insert the new hotkey under the email set.
        Neurons::<T>::insert(uid, neuron); // Insert neuron info under uid.
        Hotkeys::<T>::insert(&hotkey, uid); // Add hotkey into hotkey set.
        Self::deposit_event(Event::NeuronRegistered(uid));

        Ok(())
    }
}
