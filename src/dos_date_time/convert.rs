// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Implementations of conversions between [`DateTime`] and other types.

#[cfg(feature = "chrono")]
use chrono::NaiveDateTime;
#[cfg(feature = "jiff")]
use jiff::civil;
use time::PrimitiveDateTime;

use super::DateTime;
use crate::error::DateTimeRangeError;

impl From<DateTime> for PrimitiveDateTime {
    /// Converts a `DateTime` to a [`PrimitiveDateTime`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::{
    /// #     DateTime,
    /// #     time::{PrimitiveDateTime, macros::datetime},
    /// # };
    /// #
    /// assert_eq!(
    ///     PrimitiveDateTime::from(DateTime::MIN),
    ///     datetime!(1980-01-01 00:00:00)
    /// );
    /// assert_eq!(
    ///     PrimitiveDateTime::from(DateTime::MAX),
    ///     datetime!(2107-12-31 23:59:58)
    /// );
    /// ```
    fn from(dt: DateTime) -> Self {
        let (date, time) = (dt.date().into(), dt.time().into());
        Self::new(date, time)
    }
}

#[cfg(feature = "chrono")]
impl From<DateTime> for NaiveDateTime {
    /// Converts a `DateTime` to a [`NaiveDateTime`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::{DateTime, chrono::NaiveDateTime};
    /// #
    /// assert_eq!(
    ///     NaiveDateTime::from(DateTime::MIN),
    ///     "1980-01-01T00:00:00".parse::<NaiveDateTime>().unwrap()
    /// );
    /// assert_eq!(
    ///     NaiveDateTime::from(DateTime::MAX),
    ///     "2107-12-31T23:59:58".parse::<NaiveDateTime>().unwrap()
    /// );
    /// ```
    fn from(dt: DateTime) -> Self {
        let (date, time) = (dt.date().into(), dt.time().into());
        Self::new(date, time)
    }
}

#[cfg(feature = "jiff")]
impl From<DateTime> for civil::DateTime {
    /// Converts a `DateTime` to a [`civil::DateTime`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::{DateTime, jiff::civil};
    /// #
    /// assert_eq!(
    ///     civil::DateTime::from(DateTime::MIN),
    ///     civil::date(1980, 1, 1).at(0, 0, 0, 0)
    /// );
    /// assert_eq!(
    ///     civil::DateTime::from(DateTime::MAX),
    ///     civil::date(2107, 12, 31).at(23, 59, 58, 0)
    /// );
    /// ```
    fn from(dt: DateTime) -> Self {
        let (date, time) = (dt.date().into(), dt.time().into());
        Self::from_parts(date, time)
    }
}

impl TryFrom<PrimitiveDateTime> for DateTime {
    type Error = DateTimeRangeError;

    /// Converts a [`PrimitiveDateTime`] to a `DateTime`.
    ///
    /// <div class="warning">
    ///
    /// The resolution of MS-DOS date and time is 2 seconds. So this method
    /// rounds towards zero, truncating any fractional part of the exact result
    /// of dividing seconds by 2.
    ///
    /// </div>
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if `dt` is out of range for MS-DOS date and time.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::{DateTime, time::macros::datetime};
    /// #
    /// assert_eq!(
    ///     DateTime::try_from(datetime!(1980-01-01 00:00:00)),
    ///     Ok(DateTime::MIN)
    /// );
    /// assert_eq!(
    ///     DateTime::try_from(datetime!(2107-12-31 23:59:58)),
    ///     Ok(DateTime::MAX)
    /// );
    ///
    /// // Before `1980-01-01 00:00:00`.
    /// assert!(DateTime::try_from(datetime!(1979-12-31 23:59:59)).is_err());
    /// // After `2107-12-31 23:59:59`.
    /// assert!(DateTime::try_from(datetime!(2108-01-01 00:00:00)).is_err());
    /// ```
    fn try_from(dt: PrimitiveDateTime) -> Result<Self, Self::Error> {
        let (date, time) = (dt.date(), dt.time());
        Self::from_date_time(date, time)
    }
}

#[cfg(feature = "chrono")]
impl TryFrom<NaiveDateTime> for DateTime {
    type Error = DateTimeRangeError;

    /// Converts a [`NaiveDateTime`] to a `DateTime`.
    ///
    /// <div class="warning">
    ///
    /// The resolution of MS-DOS date and time is 2 seconds. So this method
    /// rounds towards zero, truncating any fractional part of the exact result
    /// of dividing seconds by 2.
    ///
    /// </div>
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if `dt` is out of range for MS-DOS date and time.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::{DateTime, chrono::NaiveDateTime};
    /// #
    /// assert_eq!(
    ///     DateTime::try_from("1980-01-01T00:00:00".parse::<NaiveDateTime>().unwrap()),
    ///     Ok(DateTime::MIN)
    /// );
    /// assert_eq!(
    ///     DateTime::try_from("2107-12-31T23:59:58".parse::<NaiveDateTime>().unwrap()),
    ///     Ok(DateTime::MAX)
    /// );
    ///
    /// // Before `1980-01-01 00:00:00`.
    /// assert!(DateTime::try_from("1979-12-31T23:59:59".parse::<NaiveDateTime>().unwrap()).is_err());
    /// // After `2107-12-31 23:59:59`.
    /// assert!(DateTime::try_from("2108-01-01T00:00:00".parse::<NaiveDateTime>().unwrap()).is_err());
    /// ```
    fn try_from(dt: NaiveDateTime) -> Result<Self, Self::Error> {
        let (date, time) = (dt.date().try_into()?, dt.time().into());
        let dt = Self::new(date, time);
        Ok(dt)
    }
}

#[cfg(feature = "jiff")]
impl TryFrom<civil::DateTime> for DateTime {
    type Error = DateTimeRangeError;

    /// Converts a [`civil::DateTime`] to a `DateTime`.
    ///
    /// <div class="warning">
    ///
    /// The resolution of MS-DOS date and time is 2 seconds. So this method
    /// rounds towards zero, truncating any fractional part of the exact result
    /// of dividing seconds by 2.
    ///
    /// </div>
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if `dt` is out of range for MS-DOS date and time.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::{DateTime, jiff::civil};
    /// #
    /// assert_eq!(
    ///     DateTime::try_from(civil::date(1980, 1, 1).at(0, 0, 0, 0)),
    ///     Ok(DateTime::MIN)
    /// );
    /// assert_eq!(
    ///     DateTime::try_from(civil::date(2107, 12, 31).at(23, 59, 58, 0)),
    ///     Ok(DateTime::MAX)
    /// );
    ///
    /// // Before `1980-01-01 00:00:00`.
    /// assert!(DateTime::try_from(civil::date(1979, 12, 31).at(23, 59, 59, 0)).is_err());
    /// // After `2107-12-31 23:59:59`.
    /// assert!(DateTime::try_from(civil::date(2108, 1, 1).at(0, 0, 0, 0)).is_err());
    /// ```
    fn try_from(dt: civil::DateTime) -> Result<Self, Self::Error> {
        let (date, time) = (dt.date().try_into()?, dt.time().into());
        let dt = Self::new(date, time);
        Ok(dt)
    }
}

#[cfg(test)]
mod tests {
    use time::macros::datetime;

    use super::*;
    use crate::{Date, Time, error::DateTimeRangeErrorKind};

    #[test]
    fn from_date_time_to_primitive_date_time() {
        assert_eq!(
            PrimitiveDateTime::from(DateTime::MIN),
            datetime!(1980-01-01 00:00:00)
        );
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            PrimitiveDateTime::from(DateTime::new(
                Date::new(0b0010_1101_0111_1010).unwrap(),
                Time::new(0b1001_1011_0010_0000).unwrap()
            )),
            datetime!(2002-11-26 19:25:00)
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            PrimitiveDateTime::from(DateTime::new(
                Date::new(0b0100_1101_0111_0001).unwrap(),
                Time::new(0b0101_0100_1100_1111).unwrap()
            )),
            datetime!(2018-11-17 10:38:30)
        );
        assert_eq!(
            PrimitiveDateTime::from(DateTime::MAX),
            datetime!(2107-12-31 23:59:58)
        );
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn from_date_time_to_chrono_naive_date_time() {
        assert_eq!(
            NaiveDateTime::from(DateTime::MIN),
            "1980-01-01T00:00:00".parse::<NaiveDateTime>().unwrap()
        );
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            NaiveDateTime::from(DateTime::new(
                Date::new(0b0010_1101_0111_1010).unwrap(),
                Time::new(0b1001_1011_0010_0000).unwrap()
            )),
            "2002-11-26T19:25:00".parse::<NaiveDateTime>().unwrap()
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            NaiveDateTime::from(DateTime::new(
                Date::new(0b0100_1101_0111_0001).unwrap(),
                Time::new(0b0101_0100_1100_1111).unwrap()
            )),
            "2018-11-17T10:38:30".parse::<NaiveDateTime>().unwrap()
        );
        assert_eq!(
            NaiveDateTime::from(DateTime::MAX),
            "2107-12-31T23:59:58".parse::<NaiveDateTime>().unwrap()
        );
    }

    #[cfg(feature = "jiff")]
    #[test]
    fn from_date_time_to_jiff_civil_date_time() {
        assert_eq!(
            civil::DateTime::from(DateTime::MIN),
            civil::date(1980, 1, 1).at(0, 0, 0, 0)
        );
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            civil::DateTime::from(DateTime::new(
                Date::new(0b0010_1101_0111_1010).unwrap(),
                Time::new(0b1001_1011_0010_0000).unwrap()
            )),
            civil::date(2002, 11, 26).at(19, 25, 0, 0)
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            civil::DateTime::from(DateTime::new(
                Date::new(0b0100_1101_0111_0001).unwrap(),
                Time::new(0b0101_0100_1100_1111).unwrap()
            )),
            civil::date(2018, 11, 17).at(10, 38, 30, 0)
        );
        assert_eq!(
            civil::DateTime::from(DateTime::MAX),
            civil::date(2107, 12, 31).at(23, 59, 58, 0)
        );
    }

    #[test]
    fn try_from_primitive_date_time_to_date_time_before_dos_date_time_epoch() {
        assert_eq!(
            DateTime::try_from(datetime!(1979-12-31 23:59:58)).unwrap_err(),
            DateTimeRangeErrorKind::Negative.into()
        );
        assert_eq!(
            DateTime::try_from(datetime!(1979-12-31 23:59:59)).unwrap_err(),
            DateTimeRangeErrorKind::Negative.into()
        );
    }

    #[test]
    fn try_from_primitive_date_time_to_date_time() {
        assert_eq!(
            DateTime::try_from(datetime!(1980-01-01 00:00:00)).unwrap(),
            DateTime::MIN
        );
        assert_eq!(
            DateTime::try_from(datetime!(1980-01-01 00:00:01)).unwrap(),
            DateTime::MIN
        );
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            DateTime::try_from(datetime!(2002-11-26 19:25:00)).unwrap(),
            DateTime::new(
                Date::new(0b0010_1101_0111_1010).unwrap(),
                Time::new(0b1001_1011_0010_0000).unwrap()
            )
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            DateTime::try_from(datetime!(2018-11-17 10:38:30)).unwrap(),
            DateTime::new(
                Date::new(0b0100_1101_0111_0001).unwrap(),
                Time::new(0b0101_0100_1100_1111).unwrap()
            )
        );
        assert_eq!(
            DateTime::try_from(datetime!(2107-12-31 23:59:58)).unwrap(),
            DateTime::MAX
        );
        assert_eq!(
            DateTime::try_from(datetime!(2107-12-31 23:59:59)).unwrap(),
            DateTime::MAX
        );
    }

    #[test]
    fn try_from_primitive_date_time_to_date_time_with_too_big_date_time() {
        assert_eq!(
            DateTime::try_from(datetime!(2108-01-01 00:00:00)).unwrap_err(),
            DateTimeRangeErrorKind::Overflow.into()
        );
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn try_from_chrono_naive_date_time_to_date_time_before_dos_date_time_epoch() {
        assert_eq!(
            DateTime::try_from("1979-12-31T23:59:58".parse::<NaiveDateTime>().unwrap())
                .unwrap_err(),
            DateTimeRangeErrorKind::Negative.into()
        );
        assert_eq!(
            DateTime::try_from("1979-12-31T23:59:59".parse::<NaiveDateTime>().unwrap())
                .unwrap_err(),
            DateTimeRangeErrorKind::Negative.into()
        );
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn try_from_chrono_naive_date_time_to_date_time() {
        assert_eq!(
            DateTime::try_from("1980-01-01T00:00:00".parse::<NaiveDateTime>().unwrap()).unwrap(),
            DateTime::MIN
        );
        assert_eq!(
            DateTime::try_from("1980-01-01T00:00:01".parse::<NaiveDateTime>().unwrap()).unwrap(),
            DateTime::MIN
        );
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            DateTime::try_from("2002-11-26T19:25:00".parse::<NaiveDateTime>().unwrap()).unwrap(),
            DateTime::new(
                Date::new(0b0010_1101_0111_1010).unwrap(),
                Time::new(0b1001_1011_0010_0000).unwrap()
            )
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            DateTime::try_from("2018-11-17T10:38:30".parse::<NaiveDateTime>().unwrap()).unwrap(),
            DateTime::new(
                Date::new(0b0100_1101_0111_0001).unwrap(),
                Time::new(0b0101_0100_1100_1111).unwrap()
            )
        );
        assert_eq!(
            DateTime::try_from("2107-12-31T23:59:58".parse::<NaiveDateTime>().unwrap()).unwrap(),
            DateTime::MAX
        );
        assert_eq!(
            DateTime::try_from("2107-12-31T23:59:59".parse::<NaiveDateTime>().unwrap()).unwrap(),
            DateTime::MAX
        );
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn try_from_chrono_naive_date_time_to_date_time_with_too_big_date_time() {
        assert_eq!(
            DateTime::try_from("2108-01-01T00:00:00".parse::<NaiveDateTime>().unwrap())
                .unwrap_err(),
            DateTimeRangeErrorKind::Overflow.into()
        );
    }

    #[cfg(feature = "jiff")]
    #[test]
    fn try_from_jiff_civil_date_time_to_date_time_before_dos_date_time_epoch() {
        assert_eq!(
            DateTime::try_from(civil::date(1979, 12, 31).at(23, 59, 58, 0)).unwrap_err(),
            DateTimeRangeErrorKind::Negative.into()
        );
        assert_eq!(
            DateTime::try_from(civil::date(1979, 12, 31).at(23, 59, 59, 0)).unwrap_err(),
            DateTimeRangeErrorKind::Negative.into()
        );
    }

    #[cfg(feature = "jiff")]
    #[test]
    fn try_from_jiff_civil_date_time_to_date_time() {
        assert_eq!(
            DateTime::try_from(civil::date(1980, 1, 1).at(0, 0, 0, 0)).unwrap(),
            DateTime::MIN
        );
        assert_eq!(
            DateTime::try_from(civil::date(1980, 1, 1).at(0, 0, 1, 0)).unwrap(),
            DateTime::MIN
        );
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            DateTime::try_from(civil::date(2002, 11, 26).at(19, 25, 0, 0)).unwrap(),
            DateTime::new(
                Date::new(0b0010_1101_0111_1010).unwrap(),
                Time::new(0b1001_1011_0010_0000).unwrap()
            )
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            DateTime::try_from(civil::date(2018, 11, 17).at(10, 38, 30, 0)).unwrap(),
            DateTime::new(
                Date::new(0b0100_1101_0111_0001).unwrap(),
                Time::new(0b0101_0100_1100_1111).unwrap()
            )
        );
        assert_eq!(
            DateTime::try_from(civil::date(2107, 12, 31).at(23, 59, 58, 0)).unwrap(),
            DateTime::MAX
        );
        assert_eq!(
            DateTime::try_from(civil::date(2107, 12, 31).at(23, 59, 59, 0)).unwrap(),
            DateTime::MAX
        );
    }

    #[cfg(feature = "jiff")]
    #[test]
    fn try_from_jiff_civil_date_time_to_date_time_with_too_big_date_time() {
        assert_eq!(
            DateTime::try_from(civil::date(2108, 1, 1).at(0, 0, 0, 0)).unwrap_err(),
            DateTimeRangeErrorKind::Overflow.into()
        );
    }
}
