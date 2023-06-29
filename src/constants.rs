use chrono::Datelike;
use once_cell::sync::Lazy;

pub const MONTH_LOWER_BOUND: u32 = 1;
pub const MONTH_UPPER_BOUND: u32 = 12;

pub const DAY_LOWER_BOUND: u32 = 1;
pub const DAY_UPPER_BOUND: u32 = 31;
pub static YEAR_UPPER_BOUND: Lazy<u32> = Lazy::new(|| chrono::NaiveDate::MAX.year() as u32);
