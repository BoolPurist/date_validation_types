use crate::constants::YEAR_UPPER_BOUND;
use derive_more::{Display, Into};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Year {} highter than {}", 0, *YEAR_UPPER_BOUND)]
pub struct InvalidYear(pub u32);

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Display, Clone, Copy, Into)]
pub struct ValidatedYear(u32);

impl TryFrom<u32> for ValidatedYear {
    type Error = InvalidYear;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value > *YEAR_UPPER_BOUND {
            Err(InvalidYear(value))
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
    fn should_accept_valid_years() {
        testing_utitliy::assert_accept_valid_unit::<ValidatedYear>(1);
        testing_utitliy::assert_accept_valid_unit::<ValidatedYear>(0);
        testing_utitliy::assert_accept_valid_unit::<ValidatedYear>(2000);
        testing_utitliy::assert_accept_valid_unit::<ValidatedYear>(1991);
        testing_utitliy::assert_accept_valid_unit::<ValidatedYear>(2030);
        testing_utitliy::assert_accept_valid_unit::<ValidatedYear>(1625);
        testing_utitliy::assert_accept_valid_unit::<ValidatedYear>(*YEAR_UPPER_BOUND);
    }

    #[test]
    fn should_deny_invalid_year() {
        testing_utitliy::assert_deny_invalid_unit::<ValidatedYear>(*YEAR_UPPER_BOUND + 1);
        testing_utitliy::assert_deny_invalid_unit::<ValidatedYear>(*YEAR_UPPER_BOUND + 1000);
        testing_utitliy::assert_deny_invalid_unit::<ValidatedYear>(u32::MAX);
        testing_utitliy::assert_deny_invalid_unit::<ValidatedYear>(u32::MAX - 100);
    }
}
