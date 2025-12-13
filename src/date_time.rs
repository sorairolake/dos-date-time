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

use time::{Date, Month, Time};

use crate::error::{DateTimeRangeError, DateTimeRangeErrorKind};

/// `DateTime` is a type that represents [MS-DOS date and time].
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
    date: u16,
    time: u16,
}

impl DateTime {
    #[allow(clippy::missing_panics_doc)]
    /// Creates a new `DateTime` with the given MS-DOS date and time.
    ///
    /// Returns [`None`] if the given MS-DOS date and time are not valid MS-DOS
    /// date and time.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::DateTime;
    /// #
    /// assert_eq!(
    ///     DateTime::new(0b0000_0000_0010_0001, u16::MIN),
    ///     Some(DateTime::MIN)
    /// );
    /// assert_eq!(
    ///     DateTime::new(0b1111_1111_1001_1111, 0b1011_1111_0111_1101),
    ///     Some(DateTime::MAX)
    /// );
    ///
    /// // The Day field is 0.
    /// assert_eq!(DateTime::new(0b0000_0000_0010_0000, u16::MIN), None);
    /// // The DoubleSeconds field is 30.
    /// assert_eq!(
    ///     DateTime::new(0b0000_0000_0010_0001, 0b0000_0000_0001_1110),
    ///     None
    /// );
    /// ```
    #[must_use]
    #[inline]
    pub fn new(date: u16, time: u16) -> Option<Self> {
        let (year, month, day) = (
            (1980 + (date >> 9)).into(),
            u8::try_from((date >> 5) & 0x0f)
                .expect("month should be in the range of `u8`")
                .try_into()
                .ok()?,
            (date & 0x1f)
                .try_into()
                .expect("day should be in the range of `u8`"),
        );
        let date = Date::from_calendar_date(year, month, day).ok()?;

        let (hour, minute, second) = (
            (time >> 11)
                .try_into()
                .expect("hour should be in the range of `u8`"),
            ((time >> 5) & 0x3f)
                .try_into()
                .expect("minute should be in the range of `u8`"),
            ((time & 0x1f) * 2)
                .try_into()
                .expect("second should be in the range of `u8`"),
        );
        let time = Time::from_hms(hour, minute, second).ok()?;

        Self::from_date_time(date, time).ok()
    }

    /// Creates a new `DateTime` with the given MS-DOS date and time.
    ///
    /// # Safety
    ///
    /// The given MS-DOS date and time must be valid MS-DOS date and time.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::DateTime;
    /// #
    /// assert_eq!(
    ///     unsafe { DateTime::new_unchecked(0b0000_0000_0010_0001, u16::MIN) },
    ///     DateTime::MIN
    /// );
    /// assert_eq!(
    ///     unsafe { DateTime::new_unchecked(0b1111_1111_1001_1111, 0b1011_1111_0111_1101) },
    ///     DateTime::MAX
    /// );
    /// ```
    #[must_use]
    #[inline]
    pub const unsafe fn new_unchecked(date: u16, time: u16) -> Self {
        Self { date, time }
    }

    #[allow(clippy::missing_panics_doc)]
    /// Creates a new `DateTime` with the given date and time.
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
    pub fn from_date_time(date: Date, time: Time) -> Result<Self, DateTimeRangeError> {
        match date.year() {
            ..=1979 => Err(DateTimeRangeErrorKind::Negative.into()),
            2108.. => Err(DateTimeRangeErrorKind::Overflow.into()),
            year => {
                let (year, month, day) = (
                    u16::try_from(year - 1980).expect("year should be in the range of `u16`"),
                    u16::from(u8::from(date.month())),
                    u16::from(date.day()),
                );
                let date = (year << 9) | (month << 5) | day;

                let (hour, minute, second) = (
                    u16::from(time.hour()),
                    u16::from(time.minute()),
                    u16::from(time.second() / 2),
                );
                // <https://learn.microsoft.com/en-us/windows/win32/fileio/exfat-specification#7481-doubleseconds-field>.
                let second = second.min(29);
                let time = (hour << 11) | (minute << 5) | second;

                // SAFETY: `date` and `time` are valid as MS-DOS date and time.
                let dt = unsafe { Self::new_unchecked(date, time) };
                Ok(dt)
            }
        }
    }

    /// Gets the MS-DOS date of this `DateTime`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::DateTime;
    /// #
    /// assert_eq!(DateTime::MIN.date(), 0b0000_0000_0010_0001);
    /// assert_eq!(DateTime::MAX.date(), 0b1111_1111_1001_1111);
    /// ```
    #[must_use]
    #[inline]
    pub const fn date(self) -> u16 {
        self.date
    }

    /// Gets the MS-DOS time of this `DateTime`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::DateTime;
    /// #
    /// assert_eq!(DateTime::MIN.time(), u16::MIN);
    /// assert_eq!(DateTime::MAX.time(), 0b1011_1111_0111_1101);
    /// ```
    #[must_use]
    #[inline]
    pub const fn time(self) -> u16 {
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
        1980 + (self.date() >> 9)
    }

    #[allow(clippy::missing_panics_doc)]
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
        u8::try_from((self.date() >> 5) & 0x0f)
            .expect("month should be in the range of `u8`")
            .try_into()
            .expect("month should be in the range of `Month`")
    }

    #[allow(clippy::missing_panics_doc)]
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
        (self.date() & 0x1f)
            .try_into()
            .expect("day should be in the range of `u8`")
    }

    #[allow(clippy::missing_panics_doc)]
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
        (self.time() >> 11)
            .try_into()
            .expect("hour should be in the range of `u8`")
    }

    #[allow(clippy::missing_panics_doc)]
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
        ((self.time() >> 5) & 0x3f)
            .try_into()
            .expect("minute should be in the range of `u8`")
    }

    #[allow(clippy::missing_panics_doc)]
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
        ((self.time() & 0x1f) * 2)
            .try_into()
            .expect("second should be in the range of `u8`")
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
        assert_eq!(
            DateTime::new(0b0000_0000_0010_0001, u16::MIN).unwrap(),
            DateTime::MIN
        );
        assert_eq!(
            DateTime::new(0b1111_1111_1001_1111, 0b1011_1111_0111_1101).unwrap(),
            DateTime::MAX
        );
    }

    #[test]
    fn new_with_invalid_date_time() {
        // The Day field is 0.
        assert!(DateTime::new(0b0000_0000_0010_0000, u16::MIN).is_none());
        // The Day field is 30, which is after the last day of February.
        assert!(DateTime::new(0b0000_0000_0101_1110, u16::MIN).is_none());
        // The Month field is 0.
        assert!(DateTime::new(0b0000_0000_0000_0001, u16::MIN).is_none());
        // The Month field is 13.
        assert!(DateTime::new(0b0000_0001_1010_0001, u16::MIN).is_none());

        // The DoubleSeconds field is 30.
        assert!(DateTime::new(0b0000_0000_0010_0001, 0b0000_0000_0001_1110).is_none());
        // The Minute field is 60.
        assert!(DateTime::new(0b0000_0000_0010_0001, 0b0000_0111_1000_0000).is_none());
        // The Hour field is 24.
        assert!(DateTime::new(0b0000_0000_0010_0001, 0b1100_0000_0000_0000).is_none());
    }

    #[test]
    fn new_unchecked() {
        assert_eq!(
            unsafe { DateTime::new_unchecked(0b0000_0000_0010_0001, u16::MIN) },
            DateTime::MIN
        );
        assert_eq!(
            unsafe { DateTime::new_unchecked(0b1111_1111_1001_1111, 0b1011_1111_0111_1101) },
            DateTime::MAX
        );
    }

    #[test]
    const fn new_unchecked_is_const_fn() {
        const _: DateTime = unsafe { DateTime::new_unchecked(0b0000_0000_0010_0001, u16::MIN) };
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
            DateTime::from_date_time(date!(1980-01-01), Time::MIDNIGHT).unwrap(),
            DateTime::MIN
        );
        assert_eq!(
            DateTime::from_date_time(date!(1980-01-01), time!(00:00:01)).unwrap(),
            DateTime::MIN
        );
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            DateTime::from_date_time(date!(2002-11-26), time!(19:25:00)).unwrap(),
            DateTime::new(0b0010_1101_0111_1010, 0b1001_1011_0010_0000).unwrap()
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            DateTime::from_date_time(date!(2018-11-17), time!(10:38:30)).unwrap(),
            DateTime::new(0b0100_1101_0111_0001, 0b0101_0100_1100_1111).unwrap()
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
            DateTime::from_date_time(date!(2108-01-01), Time::MIDNIGHT).unwrap_err(),
            DateTimeRangeErrorKind::Overflow.into()
        );
    }

    #[test]
    fn date() {
        assert_eq!(DateTime::MIN.date(), 0b0000_0000_0010_0001);
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            DateTime::new(0b0010_1101_0111_1010, 0b1001_1011_0010_0000)
                .unwrap()
                .date(),
            0b0010_1101_0111_1010
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            DateTime::new(0b0100_1101_0111_0001, 0b0101_0100_1100_1111)
                .unwrap()
                .date(),
            0b0100_1101_0111_0001
        );
        assert_eq!(DateTime::MAX.date(), 0b1111_1111_1001_1111);
    }

    #[test]
    const fn date_is_const_fn() {
        const _: u16 = DateTime::MIN.date();
    }

    #[test]
    fn time() {
        assert_eq!(DateTime::MIN.time(), u16::MIN);
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            DateTime::new(0b0010_1101_0111_1010, 0b1001_1011_0010_0000)
                .unwrap()
                .time(),
            0b1001_1011_0010_0000
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            DateTime::new(0b0100_1101_0111_0001, 0b0101_0100_1100_1111)
                .unwrap()
                .time(),
            0b0101_0100_1100_1111
        );
        assert_eq!(DateTime::MAX.time(), 0b1011_1111_0111_1101);
    }

    #[test]
    const fn time_is_const_fn() {
        const _: u16 = DateTime::MIN.time();
    }

    #[test]
    fn year() {
        assert_eq!(DateTime::MIN.year(), 1980);
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            DateTime::new(0b0010_1101_0111_1010, 0b1001_1011_0010_0000)
                .unwrap()
                .year(),
            2002
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            DateTime::new(0b0100_1101_0111_0001, 0b0101_0100_1100_1111)
                .unwrap()
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
            DateTime::new(0b0010_1101_0111_1010, 0b1001_1011_0010_0000)
                .unwrap()
                .month(),
            Month::November
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            DateTime::new(0b0100_1101_0111_0001, 0b0101_0100_1100_1111)
                .unwrap()
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
            DateTime::new(0b0010_1101_0111_1010, 0b1001_1011_0010_0000)
                .unwrap()
                .day(),
            26
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            DateTime::new(0b0100_1101_0111_0001, 0b0101_0100_1100_1111)
                .unwrap()
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
            DateTime::new(0b0010_1101_0111_1010, 0b1001_1011_0010_0000)
                .unwrap()
                .hour(),
            19
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            DateTime::new(0b0100_1101_0111_0001, 0b0101_0100_1100_1111)
                .unwrap()
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
            DateTime::new(0b0010_1101_0111_1010, 0b1001_1011_0010_0000)
                .unwrap()
                .minute(),
            25
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            DateTime::new(0b0100_1101_0111_0001, 0b0101_0100_1100_1111)
                .unwrap()
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
            DateTime::new(0b0010_1101_0111_1010, 0b1001_1011_0010_0000)
                .unwrap()
                .second(),
            u8::MIN
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            DateTime::new(0b0100_1101_0111_0001, 0b0101_0100_1100_1111)
                .unwrap()
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
