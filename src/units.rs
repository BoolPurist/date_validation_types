mod validated_date;
mod validated_day;
mod validated_month;
mod validated_year;

pub use validated_date::*;
pub use validated_day::*;
pub use validated_month::*;
pub use validated_year::*;

#[cfg(test)]
pub mod testing_utitliy {

    pub fn assert_deny_invalid_unit<T>(given: u32)
    where
        T: TryFrom<u32>,
    {
        let actual: Result<T, _> = given.try_into();
        assert!(actual.is_err());
    }

    pub fn assert_accept_valid_unit<T>(given: u32)
    where
        T: TryFrom<u32> + Into<u32>,
        <T as TryFrom<u32>>::Error: std::fmt::Debug,
    {
        let actual: T = given.try_into().expect("Valid year is treated as invalid");
        let actual_num: u32 = actual.into();
        assert_eq!(actual_num, given);
    }
}
