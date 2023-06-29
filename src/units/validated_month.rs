use crate::constants::{MONTH_LOWER_BOUND, MONTH_UPPER_BOUND};
use derive_more::{Display, Into};
use thiserror::Error;

#[derive(Debug, Error)]
#[error(
    "Month {} is not between {} and {}",
    0,
    MONTH_LOWER_BOUND,
    MONTH_UPPER_BOUND
)]
pub struct InvalidMonth(pub u32);

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Display, Clone, Copy, Into)]
pub struct ValidatedMonth(u32);

impl TryFrom<u32> for ValidatedMonth {
    type Error = InvalidMonth;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if !(MONTH_LOWER_BOUND..=MONTH_UPPER_BOUND).contains(&value) {
            Err(InvalidMonth(value))
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
    fn should_accept_valid_month() {
        for month_index in 1..=12 {
            testing_utitliy::assert_accept_valid_unit::<ValidatedMonth>(month_index);
        }
    }

    #[test]
    fn should_deny_invalid_months() {
        testing_utitliy::assert_deny_invalid_unit::<ValidatedMonth>(0);
        testing_utitliy::assert_deny_invalid_unit::<ValidatedMonth>(13);
        testing_utitliy::assert_deny_invalid_unit::<ValidatedMonth>(u32::MAX);
        testing_utitliy::assert_deny_invalid_unit::<ValidatedMonth>(u32::MAX / 2);
    }
}
