
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased 

## 2.0.0 - 2023.07.16 

### Added 

- Added more From trait impls for ValidatedDate
- Added "new" functions like "fn new(..) -> Self" for ValidatedDay, ValidatedMonth and ValidatedYear
- Added conversion via From trait between ValidatedDate and NaiveDate of chrono
behind the feature "chrono"

### Fixed

- Errors for invalid year, month and day now show the wrong value instead of zero. 

### Changed 

- Reexport of chrono is now behind the compile feature "chrono"

## 1.0.0 - 2023.06.29

