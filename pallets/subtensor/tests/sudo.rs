
   
use frame_support::{assert_ok};
use frame_system::Config;
mod mock;
use mock::*;
use frame_support::sp_runtime::DispatchError;


#[test]
fn test_sudo_set_rho() {
	new_test_ext().execute_with(|| {
        let rho: u64 = 11;
		assert_ok!(Subtensor::sudo_set_rho(<<Test as Config>::Origin>::root(), rho));
        assert_eq!(Subtensor::get_rho(), rho);
    });
}

#[test]
fn test_sudo_set_kappa() {
	new_test_ext().execute_with(|| {
        let kappa: u64 = 11;
		assert_ok!(Subtensor::sudo_set_kappa(<<Test as Config>::Origin>::root(), kappa));
        assert_eq!(Subtensor::get_kappa(), kappa);
    });
}

#[test]
fn test_sudo_set_blocks_per_step() {
	new_test_ext().execute_with(|| {
        let blocks_per_step: u64 = 10;
		assert_ok!(Subtensor::sudo_set_blocks_per_step(<<Test as Config>::Origin>::root(), blocks_per_step));
        assert_eq!(Subtensor::get_blocks_per_step(), blocks_per_step);
    });
}

#[test]
fn test_sudo_set_bonds_moving_average() {
	new_test_ext().execute_with(|| {
        let bonds_moving_average: u64 = 10;
		assert_ok!(Subtensor::sudo_set_bonds_moving_average(<<Test as Config>::Origin>::root(), bonds_moving_average));
        assert_eq!(Subtensor::get_bonds_moving_average(), bonds_moving_average);
    });
}

#[test]
fn test_sudo_set_difficulty() {
	new_test_ext().execute_with(|| {
        let difficulty: u64 = 10;
		assert_ok!(Subtensor::sudo_set_difficulty(<<Test as Config>::Origin>::root(), difficulty));
        assert_eq!(Subtensor::get_difficulty_as_u64(), difficulty);
    });
}


#[test]
fn test_sudo_set_adjustment_interval() {
	new_test_ext().execute_with(|| {
        let adjustment_interval: u64 = 10;
		assert_ok!(Subtensor::sudo_set_adjustment_interval(<<Test as Config>::Origin>::root(), adjustment_interval));
        assert_eq!(Subtensor::get_adjustment_interval(), adjustment_interval);

    });
}

#[test]
fn test_sudo_set_activity_cutoff() {
	new_test_ext().execute_with(|| {
        let activity_cutoff: u64 = 10;
		assert_ok!(Subtensor::sudo_set_activity_cutoff(<<Test as Config>::Origin>::root(), activity_cutoff));
        assert_eq!(Subtensor::get_activity_cutoff(), activity_cutoff);

    });
}

#[test]
fn test_sudo_target_registrations_per_interval() {
	new_test_ext().execute_with(|| {
        let target_registrations_per_interval: u64 = 10;
		assert_ok!(Subtensor::sudo_target_registrations_per_interval(<<Test as Config>::Origin>::root(), target_registrations_per_interval));
        assert_eq!(Subtensor::get_target_registrations_per_interval(), target_registrations_per_interval);
    });
}

#[test]
fn test_sudo_set_validator_epoch_len() {
	new_test_ext().execute_with(|| {
        let validator_epoch_len: u64 = 10;
		assert_ok!(Subtensor::sudo_set_validator_epoch_len(<<Test as Config>::Origin>::root(), validator_epoch_len));
        assert_eq!(Subtensor::get_validator_epoch_len(), validator_epoch_len);
    });
}

#[test]
fn test_sudo_set_validator_epochs_per_reset() {
	new_test_ext().execute_with(|| {
        let validator_epochs_per_reset: u64 = 10;
		assert_ok!(Subtensor::sudo_set_validator_epochs_per_reset(<<Test as Config>::Origin>::root(), validator_epochs_per_reset));
        assert_eq!(Subtensor::get_validator_epochs_per_reset(), validator_epochs_per_reset);
    });
}

#[test]
fn test_sudo_incentive_pruning_denominator() {
	new_test_ext().execute_with(|| {
        let incentive_pruning_denominator: u64 = 10;
		assert_ok!(Subtensor::sudo_set_incentive_pruning_denominator(<<Test as Config>::Origin>::root(), incentive_pruning_denominator));
        assert_eq!(Subtensor::get_incentive_pruning_denominator(), incentive_pruning_denominator);
    });
}

#[test]
fn test_sudo_stake_pruning_denominator() {
	new_test_ext().execute_with(|| {
        let stake_pruning_denominator: u64 = 10;
		assert_ok!(Subtensor::sudo_set_stake_pruning_denominator(<<Test as Config>::Origin>::root(), stake_pruning_denominator));
        assert_eq!(Subtensor::get_stake_pruning_denominator(), stake_pruning_denominator);
    });
}

#[test]
fn test_sudo_set_foundation_account() {
	new_test_ext().execute_with(|| {
        let foundation_account: AccountId = 10;
		assert_ok!(Subtensor::sudo_set_foundation_account(<<Test as Config>::Origin>::root(), foundation_account));
        assert_eq!(Subtensor::get_foundation_account(), foundation_account);
    });
}

#[test]
fn test_sudo_set_foundation_distribution() {
	new_test_ext().execute_with(|| {
        let foundation_distribution: AccountId = 10;
		assert_ok!(Subtensor::sudo_set_foundation_distribution(<<Test as Config>::Origin>::root(), foundation_distribution));
        assert_eq!(Subtensor::get_foundation_distribution(), foundation_distribution);
    });
}


#[test]
fn test_sudo_max_allowed_uids() {
	new_test_ext().execute_with(|| {
        let max_allowed_uids: u64 = 10;
		assert_ok!(Subtensor::sudo_set_max_allowed_uids(<<Test as Config>::Origin>::root(), max_allowed_uids));
        assert_eq!(Subtensor::get_max_allowed_uids(), max_allowed_uids);
    });
}

#[test]
fn test_sudo_min_allowed_weights() {
	new_test_ext().execute_with(|| {
        let min_allowed_weights: u64 = 1;
		assert_ok!(Subtensor::sudo_set_min_allowed_weights(<<Test as Config>::Origin>::root(), min_allowed_weights));
        assert_eq!(Subtensor::get_min_allowed_weights(), min_allowed_weights);
    });
}

#[test]
fn test_sudo_immunity_period() {
	new_test_ext().execute_with(|| {
        let immunity_period: u64 = 10;
		assert_ok!(Subtensor::sudo_set_immunity_period(<<Test as Config>::Origin>::root(), immunity_period));
        assert_eq!(Subtensor::get_immunity_period(), immunity_period);
    });
}

#[test]
fn test_sudo_validator_batch_size() {
	new_test_ext().execute_with(|| {
        let validator_batch_size: u64 = 10;
		assert_ok!(Subtensor::sudo_set_validator_batch_size(<<Test as Config>::Origin>::root(), validator_batch_size));
        assert_eq!(Subtensor::get_validator_batch_size(), validator_batch_size);
    });
}

#[test]
fn test_sudo_validator_sequence_length() {
	new_test_ext().execute_with(|| {
        let validator_sequence_length: u64 = 10;
		assert_ok!(Subtensor::sudo_set_validator_sequence_length(<<Test as Config>::Origin>::root(), validator_sequence_length));
        assert_eq!(Subtensor::get_validator_sequence_length(), validator_sequence_length);
    });
}

#[test]
fn test_fails_sudo_immunity_period () {
	new_test_ext().execute_with(|| {
        let immunity_period: u64 = 10;
        let initial_immunity_period: u64 = Subtensor::get_immunity_period();
		assert_eq!(Subtensor::sudo_set_immunity_period(<<Test as Config>::Origin>::signed(0), immunity_period), Err(DispatchError::BadOrigin.into()));
        assert_eq!(Subtensor::get_immunity_period(), initial_immunity_period);
    });
}

#[test]
fn test_fails_sudo_set_rho() {
	new_test_ext().execute_with(|| {
        let rho: u64 = 10;
        let init_rho: u64 = Subtensor::get_rho();
		assert_eq!(Subtensor::sudo_set_rho(<<Test as Config>::Origin>::signed(0), rho), Err(DispatchError::BadOrigin.into()));
        assert_eq!(Subtensor::get_rho(), init_rho);
    });
}

#[test]
fn test_fails_sudo_set_kappa() {
	new_test_ext().execute_with(|| {
        let kappa: u64 = 10;
        let init_kappa: u64 = Subtensor::get_kappa();
		assert_eq!(Subtensor::sudo_set_kappa(<<Test as Config>::Origin>::signed(0), kappa), Err(DispatchError::BadOrigin.into()));
        assert_eq!(Subtensor::get_kappa(), init_kappa);
    });
}

#[test]
fn test_fails_sudo_set_blocks_per_step() {
	new_test_ext().execute_with(|| {
        let blocks_per_step: u64 = 10;
        let init_blocks_per_step: u64 = Subtensor::get_blocks_per_step();
		assert_eq!(Subtensor::sudo_set_blocks_per_step(<<Test as Config>::Origin>::signed(0), blocks_per_step), Err(DispatchError::BadOrigin.into()));
        assert_eq!(Subtensor::get_blocks_per_step(), init_blocks_per_step);
    });
}


#[test]
fn test_fails_sudo_set_bonds_moving_average() {
	new_test_ext().execute_with(|| {
        let bonds_moving_average: u64 = 10;
        let init_bonds_moving_average: u64 = Subtensor::get_bonds_moving_average();
		assert_eq!(Subtensor::sudo_set_bonds_moving_average(<<Test as Config>::Origin>::signed(0), bonds_moving_average), Err(DispatchError::BadOrigin.into()));
        assert_eq!(Subtensor::get_bonds_moving_average(), init_bonds_moving_average);
    });
}


#[test]
fn test_fails_sudo_set_difficulty() {
	new_test_ext().execute_with(|| {
        let difficulty: u64 = 10;
        let init_difficulty: u64 = Subtensor::get_difficulty_as_u64();
		assert_eq!(Subtensor::sudo_set_difficulty(<<Test as Config>::Origin>::signed(0), difficulty),  Err(DispatchError::BadOrigin.into()));
        assert_eq!(Subtensor::get_difficulty_as_u64(), init_difficulty);
    });
}


#[test]
fn test_fails_sudo_set_adjustment_interval() {
	new_test_ext().execute_with(|| {
        let adjustment_interval: u64 = 10;
        let init_adjustment_interval: u64 = Subtensor::get_adjustment_interval();
		assert_eq!(Subtensor::sudo_set_adjustment_interval(<<Test as Config>::Origin>::signed(0), adjustment_interval),  Err(DispatchError::BadOrigin.into()));
        assert_eq!(Subtensor::get_adjustment_interval(), init_adjustment_interval);

    });
}

#[test]
fn test_fails_sudo_set_activity_cutoff() {
	new_test_ext().execute_with(|| {
        let activity_cutoff: u64 = 10;
        let init_activity_cutoff: u64 = Subtensor::get_activity_cutoff();
		assert_eq!(Subtensor::sudo_set_activity_cutoff(<<Test as Config>::Origin>::signed(0), activity_cutoff),  Err(DispatchError::BadOrigin.into()));
        assert_eq!(Subtensor::get_activity_cutoff(), init_activity_cutoff);
    });
}

#[test]
fn test_fails_sudo_target_registrations_per_interval() {
	new_test_ext().execute_with(|| {
        let target_registrations_per_interval: u64 = 10;
        let init_target_registrations_per_interval: u64 = Subtensor::get_target_registrations_per_interval();
		assert_eq!(Subtensor::sudo_target_registrations_per_interval(<<Test as Config>::Origin>::signed(0), target_registrations_per_interval),  Err(DispatchError::BadOrigin.into()));
        assert_eq!(Subtensor::get_target_registrations_per_interval(), init_target_registrations_per_interval);
    });
}

#[test]
fn test_fails_sudo_set_min_allowed_weights() {
	new_test_ext().execute_with(|| {
        let min_allowed_weights: u64 = 10;
        let init_min_allowed_weights: u64 = Subtensor::get_min_allowed_weights();
		assert_eq!(Subtensor::sudo_set_min_allowed_weights(<<Test as Config>::Origin>::signed(0), min_allowed_weights),  Err(DispatchError::BadOrigin.into()));
        assert_eq!(Subtensor::get_min_allowed_weights(), init_min_allowed_weights);
    });
}

#[test]
fn test_fails_sudo_set_validator_batch_size() {
	new_test_ext().execute_with(|| {
        let validator_batch_size: u64 = 10;
        let init_validator_batch_size: u64 = Subtensor::get_validator_batch_size();
		assert_eq!(Subtensor::sudo_set_validator_batch_size(<<Test as Config>::Origin>::signed(0), validator_batch_size),  Err(DispatchError::BadOrigin.into()));
        assert_eq!(Subtensor::get_validator_batch_size(), init_validator_batch_size);
    });
}


#[test]
fn test_fails_sudo_set_validator_sequence_length() {
	new_test_ext().execute_with(|| {
        let validator_sequence_length: u64 = 10;
        let init_validator_sequence_length: u64 = Subtensor::get_validator_sequence_length();
		assert_eq!(Subtensor::sudo_set_validator_sequence_length(<<Test as Config>::Origin>::signed(0), validator_sequence_length),  Err(DispatchError::BadOrigin.into()));
        assert_eq!(Subtensor::get_validator_sequence_length(), init_validator_sequence_length);
    });
}

#[test]
fn test_fails_sudo_set_incentive_pruning_denominator() {
	new_test_ext().execute_with(|| {
        let incentive_pruning_denominator: u64 = 10;
        let init_incentive_pruning_denominator: u64 = Subtensor::get_incentive_pruning_denominator();
		assert_eq!(Subtensor::sudo_set_incentive_pruning_denominator(<<Test as Config>::Origin>::signed(0), incentive_pruning_denominator),  Err(DispatchError::BadOrigin.into()));
        assert_eq!(Subtensor::get_incentive_pruning_denominator(), init_incentive_pruning_denominator);
    });
}

#[test]
fn test_fails_sudo_set_stake_pruning_denominator() {
	new_test_ext().execute_with(|| {
        let stake_pruning_denominator: u64 = 10;
        let init_stake_pruning_denominator: u64 = Subtensor::get_stake_pruning_denominator();
		assert_eq!(Subtensor::sudo_set_stake_pruning_denominator(<<Test as Config>::Origin>::signed(0), stake_pruning_denominator),  Err(DispatchError::BadOrigin.into()));
        assert_eq!(Subtensor::get_stake_pruning_denominator(), init_stake_pruning_denominator);
    });
}

#[test]
fn test_fails_sudo_set_foundation_account() {
	new_test_ext().execute_with(|| {
        let foundation_account: AccountId = 10;
        let init_foundation_account: AccountId = Subtensor::get_foundation_account();
		assert_eq!(Subtensor::sudo_set_foundation_account(<<Test as Config>::Origin>::signed(0), foundation_account),  Err(DispatchError::BadOrigin.into()));
        assert_eq!(Subtensor::get_foundation_account(), init_foundation_account);
    });
}

#[test]
fn test_fails_sudo_set_foundation_distribution() {
	new_test_ext().execute_with(|| {
        let foundation_distribution: AccountId = 10;
        let init_foundation_distribution: AccountId = Subtensor::get_foundation_distribution();
		assert_eq!(Subtensor::sudo_set_foundation_distribution(<<Test as Config>::Origin>::signed(0), foundation_distribution),  Err(DispatchError::BadOrigin.into()));
        assert_eq!(Subtensor::get_foundation_distribution(), init_foundation_distribution);
    });
}

#[test]
fn test_fails_sudo_set_validator_epoch_len() {
	new_test_ext().execute_with(|| {
        let validator_epoch_len: u64 = 10;
        let init_validator_epoch_len: u64 = Subtensor::get_validator_epoch_len();
		assert_eq!(Subtensor::sudo_set_validator_epoch_len(<<Test as Config>::Origin>::signed(0), validator_epoch_len),  Err(DispatchError::BadOrigin.into()));
        assert_eq!(Subtensor::get_validator_epoch_len(), init_validator_epoch_len);
    });
}

#[test]
fn test_fails_sudo_set_validator_epochs_per_reset() {
	new_test_ext().execute_with(|| {
        let validator_epochs_per_reset: u64= 10;
        let init_validator_epochs_per_reset: u64 = Subtensor::get_validator_epochs_per_reset();
		assert_eq!(Subtensor::sudo_set_validator_epochs_per_reset(<<Test as Config>::Origin>::signed(0), validator_epochs_per_reset),  Err(DispatchError::BadOrigin.into()));
        assert_eq!(Subtensor::get_validator_epochs_per_reset(), init_validator_epochs_per_reset);
    });
}