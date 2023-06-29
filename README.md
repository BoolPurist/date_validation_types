# date_validation_types 

Current version: 1.0.0

## Purpose

- Types for validating if a u32 value is valid for a  day, month or year.
- Type to for a valid date coming from simple integer value

## Example

```rust
use date_validation_types::{
    InvalidDay, InvalidMonth, InvalidYear, ValidatedDate, ValidatedDay, ValidatedMonth,
    ValidatedYear,
};
let valid_day: ValidatedDay = 10.try_into().unwrap();
let valid_month: ValidatedMonth = 10.try_into().unwrap();
let valid_year: ValidatedYear = 1990.try_into().unwrap();

let valid_date = ValidatedDate::new(valid_year, valid_month, valid_day).unwrap();
assert_eq!(valid_date.day(), 10);
assert_eq!(valid_date.month(), 10);
assert_eq!(valid_date.year(), 1990);

let invalid_day: Result<ValidatedDay, InvalidDay> = 32.try_into();
let invalid_month: Result<ValidatedMonth, InvalidMonth> = 13.try_into();
let invalid_year: Result<ValidatedYear, InvalidYear> = 455555555.try_into();
assert!(invalid_day.is_err());
assert!(invalid_month.is_err());
assert!(invalid_year.is_err());

// Construct date via a day within a the year instead of a month
let ordinal_date = ValidatedDate::from_ordinal(1990, 42).unwrap();
assert_eq!(ordinal_date.year(), 1990);
assert_eq!(ordinal_date.month(), 2);
assert_eq!(ordinal_date.day(), 11);
```

## Requirements

- Rust version 1.60 or higher

## License

MIT OR Apache-2.0
