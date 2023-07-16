use crate::constants::{DAY_LOWER_BOUND, DAY_UPPER_BOUND};
use derive_more::{Display, Into};
use thiserror::Error;

#[derive(Debug, Error)]
#[error(
    "Day {} is not between {} and {}",
    _0,
    DAY_LOWER_BOUND,
    DAY_UPPER_BOUND
)]
pub struct InvalidDay(pub u32);

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Display, Clone, Copy, Into)]
pub struct ValidatedDay(u32);

impl ValidatedDay {
    pub fn new(to_validated: u32) -> Result<Self, InvalidDay> {
        to_validated.try_into()
    }
}

impl TryFrom<u32> for ValidatedDay {
    type Error = InvalidDay;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if !(DAY_LOWER_BOUND..=DAY_UPPER_BOUND).contains(&value) {
            Err(InvalidDay(value))
        } else {
            Ok(Self(value))
        }
    }
}

#[cfg(test)]
mod testing {
    use super::*;
    use crate::units::testing_utitliy;

    #[test]
    fn should_accept_valid_days() {
        for month_index in 1..=31 {
            testing_utitliy::assert_accept_valid_unit::<ValidatedDay>(month_index);
        }
    }

    #[test]
    fn invalid_day_correct_error_msg() {
        let invalid = ValidatedDay::new(40);
        if let Err(error) = invalid {
            assert_eq!("Day 40 is not between 1 and 31", error.to_string());
        } else {
            panic!("Should have resulted into an error for an invalid day");
        }
    }

    #[test]
    fn should_deny_invalid_days() {
        testing_utitliy::assert_deny_invalid_unit::<ValidatedDay>(0);
        testing_utitliy::assert_deny_invalid_unit::<ValidatedDay>(32);
        testing_utitliy::assert_deny_invalid_unit::<ValidatedDay>(u32::MAX);
        testing_utitliy::assert_deny_invalid_unit::<ValidatedDay>(u32::MAX / 2);
    }
}
