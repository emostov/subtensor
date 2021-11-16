
   
use frame_support::{assert_ok};
use frame_system::Config;
mod mock;
use mock::*;
use frame_support::sp_runtime::DispatchError;

#[test]
fn test_sudo_set_blocks_per_step() {
	new_test_ext().execute_with(|| {
        let blocks_per_step: u64 = 10;
		assert_ok!(Subtensor::sudo_set_blocks_per_step(<<Test as Config>::Origin>::root(), blocks_per_step));
        assert_eq!(Subtensor::get_blocks_per_step(), blocks_per_step)
    });
}

#[test]
fn test_sudo_set_difficulty() {
	new_test_ext().execute_with(|| {
        let difficulty: u64 = 10;
		assert_ok!(Subtensor::sudo_set_difficulty(<<Test as Config>::Origin>::root(), difficulty));
        assert_eq!(Subtensor::get_difficulty_as_u64(), difficulty)
    });
}


#[test]
fn test_sudo_set_adjustment_interval() {
	new_test_ext().execute_with(|| {
        let adjustment_interval: u64 = 10;
		assert_ok!(Subtensor::sudo_set_adjustment_interval(<<Test as Config>::Origin>::root(), adjustment_interval));
        assert_eq!(Subtensor::get_adjustment_interval(), adjustment_interval)

    });
}

#[test]
fn test_sudo_set_activity_cutoff() {
	new_test_ext().execute_with(|| {
        let activity_cutoff: u64 = 10;
		assert_ok!(Subtensor::sudo_set_activity_cutoff(<<Test as Config>::Origin>::root(), activity_cutoff));
        assert_eq!(Subtensor::get_activity_cutoff(), activity_cutoff)

    });
}

#[test]
fn test_sudo_target_registrations_per_interval() {
	new_test_ext().execute_with(|| {
        let target_registrations_per_interval: u64 = 10;
		assert_ok!(Subtensor::sudo_target_registrations_per_interval(<<Test as Config>::Origin>::root(), target_registrations_per_interval));
        assert_eq!(Subtensor::get_target_registrations_per_interval(), target_registrations_per_interval)
    });
}


#[test]
fn test_fails_sudo_set_blocks_per_step() {
	new_test_ext().execute_with(|| {
        let blocks_per_step: u64 = 10;
        let init_blocks_per_step: u64 = Subtensor::get_blocks_per_step();
		assert_ok!(Subtensor::sudo_set_blocks_per_step(<<Test as Config>::Origin>::signed(0), blocks_per_step));
        assert_eq!(Subtensor::get_blocks_per_step(), init_blocks_per_step)
    });
}


#[test]
fn test_fails_sudo_set_difficulty() {
	new_test_ext().execute_with(|| {
        let difficulty: u64 = 10;
        let init_difficulty: u64 = Subtensor::get_difficulty_as_u64();
		assert_eq!(Subtensor::sudo_set_difficulty(<<Test as Config>::Origin>::signed(0), difficulty),  Err(DispatchError::BadOrigin.into()));
        assert_eq!(Subtensor::get_difficulty_as_u64(), init_difficulty)
    });
}


#[test]
fn test_fails_sudo_set_adjustment_interval() {
	new_test_ext().execute_with(|| {
        let adjustment_interval: u64 = 10;
        let init_adjustment_interval: u64 = Subtensor::get_adjustment_interval();
		assert_eq!(Subtensor::sudo_set_adjustment_interval(<<Test as Config>::Origin>::signed(0), adjustment_interval),  Err(DispatchError::BadOrigin.into()));
        assert_eq!(Subtensor::get_adjustment_interval(), init_adjustment_interval)

    });
}

#[test]
fn test_fails_sudo_set_activity_cutoff() {
	new_test_ext().execute_with(|| {
        let activity_cutoff: u64 = 10;
        let init_activity_cutoff: u64 = Subtensor::get_activity_cutoff();
		assert_eq!(Subtensor::sudo_set_activity_cutoff(<<Test as Config>::Origin>::signed(0), activity_cutoff),  Err(DispatchError::BadOrigin.into()));
        assert_eq!(Subtensor::get_activity_cutoff(), init_activity_cutoff)
    });
}

#[test]
fn test_fails_sudo_target_registrations_per_interval() {
	new_test_ext().execute_with(|| {
        let target_registrations_per_interval: u64 = 10;
        let init_target_registrations_per_interval: u64 = Subtensor::get_target_registrations_per_interval();
		assert_eq!(Subtensor::sudo_target_registrations_per_interval(<<Test as Config>::Origin>::signed(0), target_registrations_per_interval),  Err(DispatchError::BadOrigin.into()));
        assert_eq!(Subtensor::get_target_registrations_per_interval(), init_target_registrations_per_interval)
    });
}