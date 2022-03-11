use super::*;

impl<T: Config> Pallet<T> {
    pub fn do_set_weights(origin: T::Origin, uids: Vec<u32>, values: Vec<u32>) -> dispatch::DispatchResult
    {
        // ---- We check the caller signature
        let hotkey_id = ensure_signed(origin)?;

        // ---- We check to see that the calling neuron is in the active set.
        ensure!(Self::is_hotkey_active(&hotkey_id), Error::<T>::NotRegistered);
        let mut neuron = Self::get_neuron_for_hotkey(&hotkey_id);

        // --- We check that the length of these two lists are equal.
        ensure!(uids_match_values(&uids, &values), Error::<T>::WeightVecNotEqualSize);

        // --- We check if the uids vector does not contain duplicate ids
        ensure!(!has_duplicate_uids(&uids), Error::<T>::DuplicateUids);

        // --- We check if the weight uids are valid
        ensure!(!Self::contains_invalid_uids(&uids), Error::<T>::InvalidUid);

        // --- We check if the weights have the desired length.
        ensure!( Self::check_length(neuron.uid, &uids, &values), Error::<T>::NotSettingEnoughWeights);

        // Normalize weights.
        let normalized_values = normalize(values);

        // --- We check if the weights have an allowed max min multiple.
        ensure!( Self::min_is_allowed_multiple_of_max(&normalized_values), Error::<T>::MaxAllowedMaxMinRatioExceeded );

        // Zip weights.
        let mut zipped_weights: Vec<(u32,u32)> = vec![];
        for (uid, val) in uids.iter().zip(normalized_values.iter()) {
            zipped_weights.push((*uid, *val))
        }
        neuron.weights = zipped_weights;
        neuron.active = 1; // Set activity back to 1.
        neuron.priority = 0; // Priority is drained.
        neuron.last_update = Self::get_current_block_as_u64();

        // Sink update.
        Neurons::<T>::insert(neuron.uid, neuron);

        // ---- Emit the staking event.
        Self::deposit_event(Event::WeightsSet(hotkey_id));

        // --- Emit the event and return ok.
        Ok(())
    }

    /********************************
    --==[[  Helper functions   ]]==--
   *********************************/
    
    pub fn contains_invalid_uids(uids: &Vec<u32>) -> bool {
        for uid in uids {
            if !Self::is_uid_active(*uid) {
                return true;
            }
        }
        return false;
    }

    pub fn check_length( uid: u32, uids: &Vec<u32>, weights: &Vec<u32>) -> bool {
        let min_allowed_length: usize = Self::get_min_allowed_weights() as usize;

        // Check the self weight.
        if weights.len() == 1 {
            if uid == uids[0] {
                // Allows the self weight.
                return true;
            } else {
                // Always fails when setting just a single weight.
                return false;
            }

        // Otherwise we check to ensure we passed the weigh limit.
        } else if weights.len() >= min_allowed_length {
            return true
        } else {
            return false
        }
    }

    pub fn min_is_allowed_multiple_of_max( weights: &Vec<u32>) -> bool {
        // We allow the 0 value multiple to be cardinal -> We always return true.
        let max_allowed_max_min_ratio: u32 = Self::get_max_allowed_max_min_ratio() as u32;
        if max_allowed_max_min_ratio == 0 {
            return true;
        }
    
        let min: u32 = *weights.iter().min().unwrap();
        let max: u32 = *weights.iter().max().unwrap();
        if min == 0 { 
            return false
        } else {
            // Check that the min is a allowed multiple of the max.
            if max / min > max_allowed_max_min_ratio {
                return false;
            } else {
                return true;
            }
        }
    }
}

fn uids_match_values(uids: &Vec<u32>, values: &Vec<u32>) -> bool {
    return uids.len() == values.len();
}

/**
* This function tests if the uids half of the weight matrix contains duplicate uid's.
* If it does, an attacker could
*/
fn has_duplicate_uids(items: &Vec<u32>) -> bool {
    let mut parsed: Vec<u32> = Vec::new();
    for item in items {
        if parsed.contains(&item) { return true; }
        parsed.push(item.clone());
    }

    return false;
}


fn normalize(mut weights: Vec<u32>) -> Vec<u32> {
    let sum: u64 = weights.iter().map(|x| *x as u64).sum();
    if sum == 0 {
        return weights;
    }
    weights.iter_mut().for_each(|x| {
        *x = (*x as u64 * u32::max_value() as u64 / sum) as u32;
    });
    return weights;
}


#[cfg(test)]
mod tests {
    use crate::weights::{normalize, has_duplicate_uids};

    #[test]
    fn normalize_sum_smaller_than_one() {
        let values: Vec<u32> = vec![u32::max_value() / 10, u32::max_value() / 10];
        assert_eq!(normalize(values), vec![u32::max_value() / 2, u32::max_value() / 2]);
    }

    #[test]
    fn normalize_sum_greater_than_one() {
        let values: Vec<u32> = vec![u32::max_value() / 7, u32::max_value() / 7];
        assert_eq!(normalize(values), vec![u32::max_value() / 2, u32::max_value() / 2]);
    }

    #[test]
    fn normalize_sum_zero() {
        let weights: Vec<u32> = vec![0, 0];
        assert_eq!(normalize(weights), vec![0, 0]);
    }

    #[test]
    fn normalize_values_maxed() {
        let weights: Vec<u32> = vec![u32::max_value(), u32::max_value()];
        assert_eq!(normalize(weights), vec![u32::max_value() / 2, u32::max_value() / 2]);
    }

    #[test]
    fn has_duplicate_elements_true() {
        let weights = vec![1, 2, 3, 4, 4, 4, 4];
        assert_eq!(has_duplicate_uids(&weights), true);
    }

    #[test]
    fn has_duplicate_elements_false() {
        let weights = vec![1, 2, 3, 4, 5];
        assert_eq!(has_duplicate_uids(&weights), false);
    }
}
