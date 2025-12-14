// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! [MS-DOS date and time].
//!
//! [MS-DOS date and time]: https://learn.microsoft.com/en-us/windows/win32/sysinfo/ms-dos-date-and-time

mod cmp;
mod consts;
mod convert;
mod fmt;

use time::Month;

use crate::{Date, Time, error::DateTimeRangeError};

/// `DateTime` is a type that combines a [`Date`] and a [`Time`] and represents
/// [MS-DOS date and time].
///
/// These are packed 16-bit unsigned integer values that specify the date and
/// time an MS-DOS file was last written to, and are used as timestamps such as
/// [FAT] or [ZIP] file format.
///
/// <div class="warning">
///
/// The resolution of MS-DOS date and time is 2 seconds.
///
/// </div>
///
/// See the [format specification] for [Kaitai Struct] for more details on the
/// structure of MS-DOS date and time.
///
/// [MS-DOS date and time]: https://learn.microsoft.com/en-us/windows/win32/sysinfo/ms-dos-date-and-time
/// [FAT]: https://en.wikipedia.org/wiki/File_Allocation_Table
/// [ZIP]: https://en.wikipedia.org/wiki/ZIP_(file_format)
/// [format specification]: https://formats.kaitai.io/dos_datetime/
/// [Kaitai Struct]: https://kaitai.io/
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DateTime {
    date: Date,
    time: Time,
}

impl DateTime {
    /// Creates a new `DateTime` with the given [`Date`] and [`Time`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::{Date, DateTime, Time};
    /// #
    /// assert_eq!(DateTime::new(Date::MIN, Time::MIN), DateTime::MIN);
    /// assert_eq!(DateTime::new(Date::MAX, Time::MAX), DateTime::MAX);
    /// ```
    #[must_use]
    #[inline]
    pub const fn new(date: Date, time: Time) -> Self {
        Self { date, time }
    }

    /// Creates a new `DateTime` with the given [`time::Date`] and
    /// [`time::Time`].
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
    /// Returns [`Err`] if `date` or `time` are invalid as MS-DOS date and time.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::{
    /// #     DateTime,
    /// #     time::{
    /// #         Time,
    /// #         macros::{date, time},
    /// #     },
    /// # };
    /// #
    /// assert_eq!(
    ///     DateTime::from_date_time(date!(1980-01-01), Time::MIDNIGHT),
    ///     Ok(DateTime::MIN)
    /// );
    /// assert_eq!(
    ///     DateTime::from_date_time(date!(2107-12-31), time!(23:59:58)),
    ///     Ok(DateTime::MAX)
    /// );
    ///
    /// // Before `1980-01-01 00:00:00`.
    /// assert!(DateTime::from_date_time(date!(1979-12-31), time!(23:59:59)).is_err());
    /// // After `2107-12-31 23:59:59`.
    /// assert!(DateTime::from_date_time(date!(2108-01-01), Time::MIDNIGHT).is_err());
    /// ```
    pub fn from_date_time(date: time::Date, time: time::Time) -> Result<Self, DateTimeRangeError> {
        let (date, time) = (Date::from_date(date)?, Time::from_time(time));
        let dt = Self::new(date, time);
        Ok(dt)
    }

    /// Gets the [`Date`] of this `DateTime`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::{Date, DateTime};
    /// #
    /// assert_eq!(DateTime::MIN.date(), Date::MIN);
    /// assert_eq!(DateTime::MAX.date(), Date::MAX);
    /// ```
    #[must_use]
    #[inline]
    pub const fn date(self) -> Date {
        self.date
    }

    /// Gets the [`Time`] of this `DateTime`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::{DateTime, Time};
    /// #
    /// assert_eq!(DateTime::MIN.time(), Time::MIN);
    /// assert_eq!(DateTime::MAX.time(), Time::MAX);
    /// ```
    #[must_use]
    #[inline]
    pub const fn time(self) -> Time {
        self.time
    }

    /// Gets the year of this `DateTime`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::DateTime;
    /// #
    /// assert_eq!(DateTime::MIN.year(), 1980);
    /// assert_eq!(DateTime::MAX.year(), 2107);
    /// ```
    #[must_use]
    #[inline]
    pub const fn year(self) -> u16 {
        self.date().year()
    }

    /// Gets the month of this `DateTime`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::{DateTime, time::Month};
    /// #
    /// assert_eq!(DateTime::MIN.month(), Month::January);
    /// assert_eq!(DateTime::MAX.month(), Month::December);
    /// ```
    #[must_use]
    #[inline]
    pub fn month(self) -> Month {
        self.date().month()
    }

    /// Gets the day of this `DateTime`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::DateTime;
    /// #
    /// assert_eq!(DateTime::MIN.day(), 1);
    /// assert_eq!(DateTime::MAX.day(), 31);
    /// ```
    #[must_use]
    #[inline]
    pub fn day(self) -> u8 {
        self.date().day()
    }

    /// Gets the hour of this `DateTime`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::DateTime;
    /// #
    /// assert_eq!(DateTime::MIN.hour(), 0);
    /// assert_eq!(DateTime::MAX.hour(), 23);
    /// ```
    #[must_use]
    #[inline]
    pub fn hour(self) -> u8 {
        self.time().hour()
    }

    /// Gets the minute of this `DateTime`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::DateTime;
    /// #
    /// assert_eq!(DateTime::MIN.minute(), 0);
    /// assert_eq!(DateTime::MAX.minute(), 59);
    /// ```
    #[must_use]
    #[inline]
    pub fn minute(self) -> u8 {
        self.time().minute()
    }

    /// Gets the second of this `DateTime`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::DateTime;
    /// #
    /// assert_eq!(DateTime::MIN.second(), 0);
    /// assert_eq!(DateTime::MAX.second(), 58);
    /// ```
    #[must_use]
    #[inline]
    pub fn second(self) -> u8 {
        self.time().second()
    }
}

impl Default for DateTime {
    /// Returns the default value of "1980-01-01 00:00:00".
    ///
    /// Equivalent to [`DateTime::MIN`] except that it is not callable in const
    /// contexts.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::DateTime;
    /// #
    /// assert_eq!(DateTime::default(), DateTime::MIN);
    /// ```
    #[inline]
    fn default() -> Self {
        Self::MIN
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "std")]
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    use time::macros::{date, time};

    use super::*;
    use crate::error::DateTimeRangeErrorKind;

    #[test]
    fn clone() {
        assert_eq!(DateTime::MIN.clone(), DateTime::MIN);
    }

    #[test]
    fn copy() {
        let a = DateTime::MIN;
        let b = a;
        assert_eq!(a, b);
    }

    #[cfg(feature = "std")]
    #[test]
    fn hash() {
        assert_ne!(
            {
                let mut hasher = DefaultHasher::new();
                DateTime::MIN.hash(&mut hasher);
                hasher.finish()
            },
            {
                let mut hasher = DefaultHasher::new();
                DateTime::MAX.hash(&mut hasher);
                hasher.finish()
            }
        );
    }

    #[test]
    fn new() {
        assert_eq!(DateTime::new(Date::MIN, Time::MIN), DateTime::MIN);
        assert_eq!(DateTime::new(Date::MAX, Time::MAX), DateTime::MAX);
    }

    #[test]
    const fn new_is_const_fn() {
        const _: DateTime = DateTime::new(Date::MIN, Time::MIN);
    }

    #[test]
    fn from_date_time_before_dos_date_time_epoch() {
        assert_eq!(
            DateTime::from_date_time(date!(1979-12-31), time!(23:59:58)).unwrap_err(),
            DateTimeRangeErrorKind::Negative.into()
        );
        assert_eq!(
            DateTime::from_date_time(date!(1979-12-31), time!(23:59:59)).unwrap_err(),
            DateTimeRangeErrorKind::Negative.into()
        );
    }

    #[test]
    fn from_date_time() {
        assert_eq!(
            DateTime::from_date_time(date!(1980-01-01), time::Time::MIDNIGHT).unwrap(),
            DateTime::MIN
        );
        assert_eq!(
            DateTime::from_date_time(date!(1980-01-01), time!(00:00:01)).unwrap(),
            DateTime::MIN
        );
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            DateTime::from_date_time(date!(2002-11-26), time!(19:25:00)).unwrap(),
            DateTime::new(
                Date::new(0b0010_1101_0111_1010).unwrap(),
                Time::new(0b1001_1011_0010_0000).unwrap()
            )
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            DateTime::from_date_time(date!(2018-11-17), time!(10:38:30)).unwrap(),
            DateTime::new(
                Date::new(0b0100_1101_0111_0001).unwrap(),
                Time::new(0b0101_0100_1100_1111).unwrap()
            )
        );
        assert_eq!(
            DateTime::from_date_time(date!(2107-12-31), time!(23:59:58)).unwrap(),
            DateTime::MAX
        );
        assert_eq!(
            DateTime::from_date_time(date!(2107-12-31), time!(23:59:59)).unwrap(),
            DateTime::MAX
        );
    }

    #[test]
    fn from_date_time_with_too_big_date_time() {
        assert_eq!(
            DateTime::from_date_time(date!(2108-01-01), time::Time::MIDNIGHT).unwrap_err(),
            DateTimeRangeErrorKind::Overflow.into()
        );
    }

    #[test]
    fn date() {
        assert_eq!(DateTime::MIN.date(), Date::MIN);
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            DateTime::new(
                Date::new(0b0010_1101_0111_1010).unwrap(),
                Time::new(0b1001_1011_0010_0000).unwrap()
            )
            .date(),
            Date::new(0b0010_1101_0111_1010).unwrap()
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            DateTime::new(
                Date::new(0b0100_1101_0111_0001).unwrap(),
                Time::new(0b0101_0100_1100_1111).unwrap()
            )
            .date(),
            Date::new(0b0100_1101_0111_0001).unwrap()
        );
        assert_eq!(DateTime::MAX.date(), Date::MAX);
    }

    #[test]
    const fn date_is_const_fn() {
        const _: Date = DateTime::MIN.date();
    }

    #[test]
    fn time() {
        assert_eq!(DateTime::MIN.time(), Time::MIN);
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            DateTime::new(
                Date::new(0b0010_1101_0111_1010).unwrap(),
                Time::new(0b1001_1011_0010_0000).unwrap()
            )
            .time(),
            Time::new(0b1001_1011_0010_0000).unwrap()
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            DateTime::new(
                Date::new(0b0100_1101_0111_0001).unwrap(),
                Time::new(0b0101_0100_1100_1111).unwrap()
            )
            .time(),
            Time::new(0b0101_0100_1100_1111).unwrap()
        );
        assert_eq!(DateTime::MAX.time(), Time::MAX);
    }

    #[test]
    const fn time_is_const_fn() {
        const _: Time = DateTime::MIN.time();
    }

    #[test]
    fn year() {
        assert_eq!(DateTime::MIN.year(), 1980);
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            DateTime::new(
                Date::new(0b0010_1101_0111_1010).unwrap(),
                Time::new(0b1001_1011_0010_0000).unwrap()
            )
            .year(),
            2002
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            DateTime::new(
                Date::new(0b0100_1101_0111_0001).unwrap(),
                Time::new(0b0101_0100_1100_1111).unwrap()
            )
            .year(),
            2018
        );
        assert_eq!(DateTime::MAX.year(), 2107);
    }

    #[test]
    const fn year_is_const_fn() {
        const _: u16 = DateTime::MIN.year();
    }

    #[test]
    fn month() {
        assert_eq!(DateTime::MIN.month(), Month::January);
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            DateTime::new(
                Date::new(0b0010_1101_0111_1010).unwrap(),
                Time::new(0b1001_1011_0010_0000).unwrap()
            )
            .month(),
            Month::November
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            DateTime::new(
                Date::new(0b0100_1101_0111_0001).unwrap(),
                Time::new(0b0101_0100_1100_1111).unwrap()
            )
            .month(),
            Month::November
        );
        assert_eq!(DateTime::MAX.month(), Month::December);
    }

    #[test]
    fn day() {
        assert_eq!(DateTime::MIN.day(), 1);
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            DateTime::new(
                Date::new(0b0010_1101_0111_1010).unwrap(),
                Time::new(0b1001_1011_0010_0000).unwrap()
            )
            .day(),
            26
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            DateTime::new(
                Date::new(0b0100_1101_0111_0001).unwrap(),
                Time::new(0b0101_0100_1100_1111).unwrap()
            )
            .day(),
            17
        );
        assert_eq!(DateTime::MAX.day(), 31);
    }

    #[test]
    fn hour() {
        assert_eq!(DateTime::MIN.hour(), u8::MIN);
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            DateTime::new(
                Date::new(0b0010_1101_0111_1010).unwrap(),
                Time::new(0b1001_1011_0010_0000).unwrap()
            )
            .hour(),
            19
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            DateTime::new(
                Date::new(0b0100_1101_0111_0001).unwrap(),
                Time::new(0b0101_0100_1100_1111).unwrap()
            )
            .hour(),
            10
        );
        assert_eq!(DateTime::MAX.hour(), 23);
    }

    #[test]
    fn minute() {
        assert_eq!(DateTime::MIN.minute(), u8::MIN);
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            DateTime::new(
                Date::new(0b0010_1101_0111_1010).unwrap(),
                Time::new(0b1001_1011_0010_0000).unwrap()
            )
            .minute(),
            25
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            DateTime::new(
                Date::new(0b0100_1101_0111_0001).unwrap(),
                Time::new(0b0101_0100_1100_1111).unwrap()
            )
            .minute(),
            38
        );
        assert_eq!(DateTime::MAX.minute(), 59);
    }

    #[test]
    fn second() {
        assert_eq!(DateTime::MIN.second(), u8::MIN);
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            DateTime::new(
                Date::new(0b0010_1101_0111_1010).unwrap(),
                Time::new(0b1001_1011_0010_0000).unwrap()
            )
            .second(),
            u8::MIN
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            DateTime::new(
                Date::new(0b0100_1101_0111_0001).unwrap(),
                Time::new(0b0101_0100_1100_1111).unwrap()
            )
            .second(),
            30
        );
        assert_eq!(DateTime::MAX.second(), 58);
    }

    #[test]
    fn default() {
        assert_eq!(DateTime::default(), DateTime::MIN);
    }
}
