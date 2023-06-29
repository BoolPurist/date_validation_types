use crate::constants::{DAY_LOWER_BOUND, DAY_UPPER_BOUND};
use derive_more::{Display, Into};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Day {} is not between {} and {}", 0, DAY_LOWER_BOUND, DAY_UPPER_BOUND)]
pub struct InvalidDay(pub u32);

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Display, Clone, Copy, Into)]
pub struct ValidatedDay(u32);

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
    fn should_deny_invalid_days() {
        testing_utitliy::assert_deny_invalid_unit::<ValidatedDay>(0);
        testing_utitliy::assert_deny_invalid_unit::<ValidatedDay>(32);
        testing_utitliy::assert_deny_invalid_unit::<ValidatedDay>(u32::MAX);
        testing_utitliy::assert_deny_invalid_unit::<ValidatedDay>(u32::MAX / 2);
    }
}