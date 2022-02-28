
   
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