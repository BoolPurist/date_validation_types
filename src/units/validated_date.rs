use chrono::{Datelike, NaiveDate};
use derive_more::{Display, From, Into};

use super::{InvalidDay, InvalidMonth, InvalidYear, ValidatedDay, ValidatedMonth, ValidatedYear};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InvalidDate {
    #[error("{}", 0)]
    WrongYear(#[from] InvalidYear),
    #[error("{}", 0)]
    WrongMonth(#[from] InvalidMonth),
    #[error("{}", 0)]
    WrongDay(#[from] InvalidDay),
    #[error("Oridinal day {} in year {} is not valid", year, day)]
    WrongOrdinalDate { year: ValidatedYear, day: u32 },
    #[error("Date {}.{}.{} is not valid", year, month, day)]
    WrongDate {
        year: ValidatedYear,
        month: ValidatedMonth,
        day: ValidatedDay,
    },
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Display, Clone, Copy, Into, From)]
pub struct ValidatedDate(NaiveDate);

impl ValidatedDate {
    pub fn new(
        year: ValidatedYear,
        month: ValidatedMonth,
        day: ValidatedDay,
    ) -> Result<Self, InvalidDate> {
        let year_u32: u32 = year.into();
        let date = NaiveDate::from_ymd_opt(year_u32 as i32, month.into(), day.into())
            .ok_or(InvalidDate::WrongDate { year, month, day })?;

        Ok(Self(date))
    }

    pub fn from_ymd(year: u32, month: u32, day: u32) -> Result<Self, InvalidDate> {
        let validated_year: ValidatedYear = year.try_into()?;
        let validated_month: ValidatedMonth = month.try_into()?;
        let validated_day: ValidatedDay = day.try_into()?;

        Self::new(validated_year, validated_month, validated_day)
    }

    pub fn from_ordinal(year: u32, day_of_year: u32) -> Result<Self, InvalidDate> {
        let year: ValidatedYear = year.try_into()?;
        let ordianal_date: NaiveDate = NaiveDate::from_yo_opt(u32::from(year) as i32, day_of_year)
            .ok_or(InvalidDate::WrongOrdinalDate {
                year,
                day: day_of_year,
            })?;

        Ok(Self(ordianal_date))
    }

    pub fn day(&self) -> u32 {
        self.0.day()
    }
    pub fn month(&self) -> u32 {
        self.0.month()
    }
    pub fn year(&self) -> u32 {
        self.0.year() as u32
    }
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn should_accept_valid_dates() {
        assert_if_valid_date_is_accepted(2015, 3, 14);
        assert_if_valid_date_is_accepted(2015, 1, 14);
        assert_if_valid_date_is_accepted(4, 2, 27);
        assert_if_valid_date_is_accepted(2023, 1, 12);
        assert_if_valid_date_is_accepted(2023, 2, 28);
        assert_if_valid_date_is_accepted(2020, 2, 29);
    }
    #[test]
    fn should_deny_invalid_date() {
        assert_deny_of_invalid_dates(2015, 4, 31);
        assert_deny_of_invalid_dates(2023, 2, 29);
        assert_deny_of_invalid_dates(2020, 2, 30);
    }

    fn assert_if_valid_date_is_accepted(year: u32, month: u32, day: u32) {
        let (validated_year, validated_month, validated_day) =
            create_validated_ymd(year, month, day);

        match ValidatedDate::new(validated_year, validated_month, validated_day) {
            Ok(date) => {
                let actual_date: NaiveDate = date.into();
                assert_eq!(
                    actual_date,
                    NaiveDate::from_ymd_opt(year as i32, month, day).expect("")
                )
            }
            Err(error) => panic!("Error: Encountered for valid date. \n{}", error),
        }
    }

    fn assert_deny_of_invalid_dates(year: u32, month: u32, day: u32) {
        let (validated_year, validated_month, validated_day) =
            create_validated_ymd(year, month, day);
        let actual = ValidatedDate::new(validated_year, validated_month, validated_day);
        assert!(actual.is_err());
    }

    fn create_validated_ymd(
        year: u32,
        month: u32,
        day: u32,
    ) -> (ValidatedYear, ValidatedMonth, ValidatedDay) {
        let validated_year: ValidatedYear = year
            .try_into()
            .expect("day for date is not valid in general");
        let validated_month: ValidatedMonth = month
            .try_into()
            .expect("day for date is not valid in general");
        let validated_day: ValidatedDay = day
            .try_into()
            .expect("day for date is not valid in general");

        (validated_year, validated_month, validated_day)
    }
}
