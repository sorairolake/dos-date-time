// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Implementations of conversions between [`Date`] and other types.

#[cfg(feature = "chrono")]
use chrono::{Datelike, NaiveDate};
#[cfg(feature = "jiff")]
use jiff::civil;

use super::Date;
use crate::error::DateRangeError;

impl From<Date> for time::Date {
    /// Converts a `Date` to a [`time::Date`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::{Date, time::macros::date};
    /// #
    /// assert_eq!(time::Date::from(Date::MIN), date!(1980-01-01));
    /// assert_eq!(time::Date::from(Date::MAX), date!(2107-12-31));
    /// ```
    fn from(date: Date) -> Self {
        let (year, month, day) = (date.year().into(), date.month(), date.day());
        Self::from_calendar_date(year, month, day)
            .expect("date should be in the range of `time::Date`")
    }
}

#[cfg(feature = "chrono")]
impl From<Date> for NaiveDate {
    /// Converts a `Date` to a [`NaiveDate`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::{Date, chrono::NaiveDate};
    /// #
    /// assert_eq!(
    ///     NaiveDate::from(Date::MIN),
    ///     "1980-01-01".parse::<NaiveDate>().unwrap()
    /// );
    /// assert_eq!(
    ///     NaiveDate::from(Date::MAX),
    ///     "2107-12-31".parse::<NaiveDate>().unwrap()
    /// );
    /// ```
    fn from(date: Date) -> Self {
        let (year, month, day) = (
            date.year().into(),
            u8::from(date.month()).into(),
            date.day().into(),
        );
        Self::from_ymd_opt(year, month, day).expect("date should be in the range of `NaiveDate`")
    }
}

#[cfg(feature = "jiff")]
impl From<Date> for civil::Date {
    /// Converts a `Date` to a [`civil::Date`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::{Date, jiff::civil};
    /// #
    /// assert_eq!(civil::Date::from(Date::MIN), civil::date(1980, 1, 1));
    /// assert_eq!(civil::Date::from(Date::MAX), civil::date(2107, 12, 31));
    /// ```
    fn from(date: Date) -> Self {
        let (year, month, day) = (
            date.year()
                .try_into()
                .expect("year should be in the range of `i16`"),
            u8::from(date.month())
                .try_into()
                .expect("month should be in the range of `i8`"),
            date.day()
                .try_into()
                .expect("day should be in the range of `i8`"),
        );
        civil::date(year, month, day)
    }
}

impl TryFrom<time::Date> for Date {
    type Error = DateRangeError;

    /// Converts a [`time::Date`] to a `Date`.
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if `date` is out of range for the MS-DOS date.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::{Date, time::macros::date};
    /// #
    /// assert_eq!(Date::try_from(date!(1980-01-01)), Ok(Date::MIN));
    /// assert_eq!(Date::try_from(date!(2107-12-31)), Ok(Date::MAX));
    ///
    /// // Before `1980-01-01`.
    /// assert!(Date::try_from(date!(1979-12-31)).is_err());
    /// // After `2107-12-31`.
    /// assert!(Date::try_from(date!(2108-01-01)).is_err());
    /// ```
    fn try_from(date: time::Date) -> Result<Self, Self::Error> {
        Self::from_date(date)
    }
}

#[cfg(feature = "chrono")]
impl TryFrom<NaiveDate> for Date {
    type Error = DateRangeError;

    /// Converts a [`NaiveDate`] to a `Date`.
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if `date` is out of range for the MS-DOS date.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::{Date, chrono::NaiveDate};
    /// #
    /// assert_eq!(
    ///     Date::try_from("1980-01-01".parse::<NaiveDate>().unwrap()),
    ///     Ok(Date::MIN)
    /// );
    /// assert_eq!(
    ///     Date::try_from("2107-12-31".parse::<NaiveDate>().unwrap()),
    ///     Ok(Date::MAX)
    /// );
    ///
    /// // Before `1980-01-01`.
    /// assert!(Date::try_from("1979-12-31".parse::<NaiveDate>().unwrap()).is_err());
    /// // After `2107-12-31`.
    /// assert!(Date::try_from("2108-01-01".parse::<NaiveDate>().unwrap()).is_err());
    /// ```
    fn try_from(date: NaiveDate) -> Result<Self, Self::Error> {
        let (year, month, day) = (
            date.year(),
            u8::try_from(date.month())
                .expect("month should be in the range of `u8`")
                .try_into()
                .expect("month should be in the range of `Month`"),
            date.day()
                .try_into()
                .expect("day should be in the range of `u8`"),
        );
        let date = time::Date::from_calendar_date(year, month, day)
            .expect("date should be in the range of `time::Date`");
        Self::from_date(date)
    }
}

#[cfg(feature = "jiff")]
impl TryFrom<civil::Date> for Date {
    type Error = DateRangeError;

    /// Converts a [`civil::Date`] to a `Date`.
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if `date` is out of range for the MS-DOS date.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::{Date, jiff::civil};
    /// #
    /// assert_eq!(Date::try_from(civil::date(1980, 1, 1)), Ok(Date::MIN));
    /// assert_eq!(Date::try_from(civil::date(2107, 12, 31)), Ok(Date::MAX));
    ///
    /// // Before `1980-01-01`.
    /// assert!(Date::try_from(civil::date(1979, 12, 31)).is_err());
    /// // After `2107-12-31`.
    /// assert!(Date::try_from(civil::date(2108, 1, 1)).is_err());
    /// ```
    fn try_from(date: civil::Date) -> Result<Self, Self::Error> {
        let (year, month, day) = (
            date.year().into(),
            u8::try_from(date.month())
                .expect("month should be in the range of `u8`")
                .try_into()
                .expect("month should be in the range of `Month`"),
            date.day()
                .try_into()
                .expect("day should be in the range of `u8`"),
        );
        let date = time::Date::from_calendar_date(year, month, day)
            .expect("date should be in the range of `time::Date`");
        Self::from_date(date)
    }
}

#[cfg(test)]
mod tests {
    use time::macros::date;

    use super::*;
    use crate::error::DateRangeErrorKind;

    #[test]
    fn from_date_to_time_date() {
        assert_eq!(time::Date::from(Date::MIN), date!(1980-01-01));
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            time::Date::from(Date::new(0b0010_1101_0111_1010).unwrap()),
            date!(2002-11-26)
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            time::Date::from(Date::new(0b0100_1101_0111_0001).unwrap()),
            date!(2018-11-17)
        );
        assert_eq!(time::Date::from(Date::MAX), date!(2107-12-31));
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn from_date_to_chrono_naive_date() {
        assert_eq!(
            NaiveDate::from(Date::MIN),
            "1980-01-01".parse::<NaiveDate>().unwrap()
        );
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            NaiveDate::from(Date::new(0b0010_1101_0111_1010).unwrap()),
            "2002-11-26".parse::<NaiveDate>().unwrap()
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            NaiveDate::from(Date::new(0b0100_1101_0111_0001).unwrap()),
            "2018-11-17".parse::<NaiveDate>().unwrap()
        );
        assert_eq!(
            NaiveDate::from(Date::MAX),
            "2107-12-31".parse::<NaiveDate>().unwrap()
        );
    }

    #[cfg(feature = "jiff")]
    #[test]
    fn from_date_to_jiff_civil_date() {
        assert_eq!(civil::Date::from(Date::MIN), civil::date(1980, 1, 1));
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            civil::Date::from(Date::new(0b0010_1101_0111_1010).unwrap()),
            civil::date(2002, 11, 26)
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            civil::Date::from(Date::new(0b0100_1101_0111_0001).unwrap()),
            civil::date(2018, 11, 17)
        );
        assert_eq!(civil::Date::from(Date::MAX), civil::date(2107, 12, 31));
    }

    #[test]
    fn try_from_time_date_to_date_before_dos_date_epoch() {
        assert_eq!(
            Date::try_from(date!(1979-12-31)).unwrap_err(),
            DateRangeErrorKind::Negative.into()
        );
        assert_eq!(
            Date::try_from(date!(1979-12-31)).unwrap_err(),
            DateRangeErrorKind::Negative.into()
        );
    }

    #[test]
    fn try_from_time_date_to_date() {
        assert_eq!(Date::try_from(date!(1980-01-01)).unwrap(), Date::MIN);
        assert_eq!(Date::try_from(date!(1980-01-01)).unwrap(), Date::MIN);
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            Date::try_from(date!(2002-11-26)).unwrap(),
            Date::new(0b0010_1101_0111_1010).unwrap()
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            Date::try_from(date!(2018-11-17)).unwrap(),
            Date::new(0b0100_1101_0111_0001).unwrap()
        );
        assert_eq!(Date::try_from(date!(2107-12-31)).unwrap(), Date::MAX);
        assert_eq!(Date::try_from(date!(2107-12-31)).unwrap(), Date::MAX);
    }

    #[test]
    fn try_from_time_date_to_date_with_too_big_date() {
        assert_eq!(
            Date::try_from(date!(2108-01-01)).unwrap_err(),
            DateRangeErrorKind::Overflow.into()
        );
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn try_from_chrono_naive_date_to_date_before_dos_date_epoch() {
        assert_eq!(
            Date::try_from("1979-12-31".parse::<NaiveDate>().unwrap()).unwrap_err(),
            DateRangeErrorKind::Negative.into()
        );
        assert_eq!(
            Date::try_from("1979-12-31".parse::<NaiveDate>().unwrap()).unwrap_err(),
            DateRangeErrorKind::Negative.into()
        );
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn try_from_chrono_naive_date_to_date() {
        assert_eq!(
            Date::try_from("1980-01-01".parse::<NaiveDate>().unwrap()).unwrap(),
            Date::MIN
        );
        assert_eq!(
            Date::try_from("1980-01-01".parse::<NaiveDate>().unwrap()).unwrap(),
            Date::MIN
        );
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            Date::try_from("2002-11-26".parse::<NaiveDate>().unwrap()).unwrap(),
            Date::new(0b0010_1101_0111_1010).unwrap()
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            Date::try_from("2018-11-17".parse::<NaiveDate>().unwrap()).unwrap(),
            Date::new(0b0100_1101_0111_0001).unwrap()
        );
        assert_eq!(
            Date::try_from("2107-12-31".parse::<NaiveDate>().unwrap()).unwrap(),
            Date::MAX
        );
        assert_eq!(
            Date::try_from("2107-12-31".parse::<NaiveDate>().unwrap()).unwrap(),
            Date::MAX
        );
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn try_from_chrono_naive_date_to_date_with_too_big_date() {
        assert_eq!(
            Date::try_from("2108-01-01".parse::<NaiveDate>().unwrap()).unwrap_err(),
            DateRangeErrorKind::Overflow.into()
        );
    }

    #[cfg(feature = "jiff")]
    #[test]
    fn try_from_jiff_civil_date_to_date_before_dos_date_epoch() {
        assert_eq!(
            Date::try_from(civil::date(1979, 12, 31)).unwrap_err(),
            DateRangeErrorKind::Negative.into()
        );
        assert_eq!(
            Date::try_from(civil::date(1979, 12, 31)).unwrap_err(),
            DateRangeErrorKind::Negative.into()
        );
    }

    #[cfg(feature = "jiff")]
    #[test]
    fn try_from_jiff_civil_date_to_date() {
        assert_eq!(Date::try_from(civil::date(1980, 1, 1)).unwrap(), Date::MIN);
        assert_eq!(Date::try_from(civil::date(1980, 1, 1)).unwrap(), Date::MIN);
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            Date::try_from(civil::date(2002, 11, 26)).unwrap(),
            Date::new(0b0010_1101_0111_1010).unwrap()
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            Date::try_from(civil::date(2018, 11, 17)).unwrap(),
            Date::new(0b0100_1101_0111_0001).unwrap()
        );
        assert_eq!(
            Date::try_from(civil::date(2107, 12, 31)).unwrap(),
            Date::MAX
        );
        assert_eq!(
            Date::try_from(civil::date(2107, 12, 31)).unwrap(),
            Date::MAX
        );
    }

    #[cfg(feature = "jiff")]
    #[test]
    fn try_from_jiff_civil_date_to_date_with_too_big_date() {
        assert_eq!(
            Date::try_from(civil::date(2108, 1, 1)).unwrap_err(),
            DateRangeErrorKind::Overflow.into()
        );
    }
}
