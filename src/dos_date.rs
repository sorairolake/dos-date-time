// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! The [MS-DOS date].
//!
//! [MS-DOS date]: https://learn.microsoft.com/en-us/windows/win32/sysinfo/ms-dos-date-and-time

mod cmp;
mod consts;
mod convert;
mod fmt;

use time::Month;

use crate::error::{DateRangeError, DateRangeErrorKind};

/// `Date` is a type that represents the [MS-DOS date].
///
/// This is a packed 16-bit unsigned integer value.
///
/// See the [format specification] for [Kaitai Struct] for more details on the
/// structure of the MS-DOS date.
///
/// [MS-DOS date]: https://learn.microsoft.com/en-us/windows/win32/sysinfo/ms-dos-date-and-time
/// [format specification]: https://formats.kaitai.io/dos_datetime/
/// [Kaitai Struct]: https://kaitai.io/
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Date(u16);

impl Date {
    #[allow(clippy::missing_panics_doc)]
    /// Creates a new `Date` with the given MS-DOS date.
    ///
    /// Returns [`None`] if the given MS-DOS date is not a valid MS-DOS date.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::Date;
    /// #
    /// assert_eq!(Date::new(0b0000_0000_0010_0001), Some(Date::MIN));
    /// assert_eq!(Date::new(0b1111_1111_1001_1111), Some(Date::MAX));
    ///
    /// // The Day field is 0.
    /// assert_eq!(Date::new(0b0000_0000_0010_0000), None);
    /// ```
    #[must_use]
    pub fn new(date: u16) -> Option<Self> {
        let (year, month, day) = (
            (1980 + (date >> 9)).into(),
            u8::try_from((date >> 5) & 0x0F)
                .expect("month should be in the range of `u8`")
                .try_into()
                .ok()?,
            (date & 0x1F)
                .try_into()
                .expect("day should be in the range of `u8`"),
        );
        let date = time::Date::from_calendar_date(year, month, day).ok()?;
        Self::from_date(date).ok()
    }

    /// Creates a new `Date` with the given MS-DOS date.
    ///
    /// # Safety
    ///
    /// The given MS-DOS date must be a valid MS-DOS date.
    #[must_use]
    pub const unsafe fn new_unchecked(date: u16) -> Self {
        Self(date)
    }

    #[allow(clippy::missing_panics_doc)]
    /// Creates a new `Date` with the given [`time::Date`].
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if `date` is an invalid as the MS-DOS date.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::{Date, time::macros::date};
    /// #
    /// assert_eq!(Date::from_date(date!(1980-01-01)), Ok(Date::MIN));
    /// assert_eq!(Date::from_date(date!(2107-12-31)), Ok(Date::MAX));
    ///
    /// // Before `1980-01-01`.
    /// assert!(Date::from_date(date!(1979-12-31)).is_err());
    /// // After `2107-12-31`.
    /// assert!(Date::from_date(date!(2108-01-01)).is_err());
    /// ```
    pub fn from_date(date: time::Date) -> Result<Self, DateRangeError> {
        match date.year() {
            ..=1979 => Err(DateRangeErrorKind::Negative.into()),
            2108.. => Err(DateRangeErrorKind::Overflow.into()),
            year => {
                let (year, month, day) = (
                    u16::try_from(year - 1980).expect("year should be in the range of `u16`"),
                    u16::from(u8::from(date.month())),
                    u16::from(date.day()),
                );
                let date = (year << 9) | (month << 5) | day;
                // SAFETY: `date` is a valid as the MS-DOS date.
                let date = unsafe { Self::new_unchecked(date) };
                Ok(date)
            }
        }
    }

    /// Returns [`true`] if `self` is a valid MS-DOS date, and [`false`]
    /// otherwise.
    #[must_use]
    pub fn is_valid(self) -> bool {
        Self::new(self.to_raw()).is_some()
    }

    /// Returns the MS-DOS date of this `Date` as the underlying [`u16`] value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::Date;
    /// #
    /// assert_eq!(Date::MIN.to_raw(), 0b0000_0000_0010_0001);
    /// assert_eq!(Date::MAX.to_raw(), 0b1111_1111_1001_1111);
    /// ```
    #[must_use]
    pub const fn to_raw(self) -> u16 {
        self.0
    }

    /// Gets the year of this `Date`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::Date;
    /// #
    /// assert_eq!(Date::MIN.year(), 1980);
    /// assert_eq!(Date::MAX.year(), 2107);
    /// ```
    #[must_use]
    pub const fn year(self) -> u16 {
        1980 + (self.to_raw() >> 9)
    }

    #[allow(clippy::missing_panics_doc)]
    /// Gets the month of this `Date`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::{Date, time::Month};
    /// #
    /// assert_eq!(Date::MIN.month(), Month::January);
    /// assert_eq!(Date::MAX.month(), Month::December);
    /// ```
    #[must_use]
    pub fn month(self) -> Month {
        u8::try_from((self.to_raw() >> 5) & 0x0F)
            .expect("month should be in the range of `u8`")
            .try_into()
            .expect("month should be in the range of `Month`")
    }

    #[allow(clippy::missing_panics_doc)]
    /// Gets the day of this `Date`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::Date;
    /// #
    /// assert_eq!(Date::MIN.day(), 1);
    /// assert_eq!(Date::MAX.day(), 31);
    /// ```
    #[must_use]
    pub fn day(self) -> u8 {
        (self.to_raw() & 0x1F)
            .try_into()
            .expect("day should be in the range of `u8`")
    }
}

impl Default for Date {
    /// Returns the default value of "1980-01-01".
    ///
    /// Equivalent to [`Date::MIN`] except that it is not callable in const
    /// contexts.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::Date;
    /// #
    /// assert_eq!(Date::default(), Date::MIN);
    /// ```
    fn default() -> Self {
        Self::MIN
    }
}

#[cfg(test)]
mod tests {
    use core::mem;
    #[cfg(feature = "std")]
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    use time::macros::date;

    use super::*;

    #[test]
    fn size_of() {
        assert_eq!(mem::size_of::<Date>(), mem::size_of::<u16>());
    }

    #[test]
    fn align_of() {
        assert_eq!(mem::align_of::<Date>(), mem::align_of::<u16>());
    }

    #[test]
    fn clone() {
        assert_eq!(Date::MIN.clone(), Date::MIN);
    }

    #[test]
    fn copy() {
        let a = Date::MIN;
        let b = a;
        assert_eq!(a, b);
    }

    #[cfg(feature = "std")]
    #[test]
    fn hash() {
        assert_ne!(
            {
                let mut hasher = DefaultHasher::new();
                Date::MIN.hash(&mut hasher);
                hasher.finish()
            },
            {
                let mut hasher = DefaultHasher::new();
                Date::MAX.hash(&mut hasher);
                hasher.finish()
            }
        );
    }

    #[test]
    fn new() {
        assert_eq!(Date::new(0b0000_0000_0010_0001).unwrap(), Date::MIN);
        assert_eq!(Date::new(0b1111_1111_1001_1111).unwrap(), Date::MAX);
    }

    #[test]
    fn new_with_invalid_date() {
        // The Day field is 0.
        assert!(Date::new(0b0000_0000_0010_0000).is_none());
        // The Day field is 30, which is after the last day of February.
        assert!(Date::new(0b0000_0000_0101_1110).is_none());
        // The Month field is 0.
        assert!(Date::new(0b0000_0000_0000_0001).is_none());
        // The Month field is 13.
        assert!(Date::new(0b0000_0001_1010_0001).is_none());
    }

    #[test]
    fn new_unchecked() {
        assert_eq!(
            unsafe { Date::new_unchecked(0b0000_0000_0010_0001) },
            Date::MIN
        );
        assert_eq!(
            unsafe { Date::new_unchecked(0b1111_1111_1001_1111) },
            Date::MAX
        );
    }

    #[test]
    const fn new_unchecked_is_const_fn() {
        const _: Date = unsafe { Date::new_unchecked(0b0000_0000_0010_0001) };
    }

    #[test]
    fn from_date_before_dos_date_epoch() {
        assert_eq!(
            Date::from_date(date!(1979-12-31)).unwrap_err(),
            DateRangeErrorKind::Negative.into()
        );
        assert_eq!(
            Date::from_date(date!(1979-12-31)).unwrap_err(),
            DateRangeErrorKind::Negative.into()
        );
    }

    #[test]
    fn from_date() {
        assert_eq!(Date::from_date(date!(1980-01-01)).unwrap(), Date::MIN);
        assert_eq!(Date::from_date(date!(1980-01-01)).unwrap(), Date::MIN);
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            Date::from_date(date!(2002-11-26)).unwrap(),
            Date::new(0b0010_1101_0111_1010).unwrap()
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            Date::from_date(date!(2018-11-17)).unwrap(),
            Date::new(0b0100_1101_0111_0001).unwrap()
        );
        assert_eq!(Date::from_date(date!(2107-12-31)).unwrap(), Date::MAX);
        assert_eq!(Date::from_date(date!(2107-12-31)).unwrap(), Date::MAX);
    }

    #[test]
    fn from_date_with_too_big_date() {
        assert_eq!(
            Date::from_date(date!(2108-01-01)).unwrap_err(),
            DateRangeErrorKind::Overflow.into()
        );
    }

    #[test]
    fn is_valid() {
        assert!(Date::MIN.is_valid());
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert!(Date::new(0b0010_1101_0111_1010).unwrap().is_valid());
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert!(Date::new(0b0100_1101_0111_0001).unwrap().is_valid());
        assert!(Date::MAX.is_valid());
    }

    #[test]
    fn is_valid_with_invalid_date() {
        // The Day field is 0.
        assert!(!unsafe { Date::new_unchecked(0b0000_0000_0010_0000) }.is_valid());
        // The Day field is 30, which is after the last day of February.
        assert!(!unsafe { Date::new_unchecked(0b0000_0000_0101_1110) }.is_valid());
        // The Month field is 0.
        assert!(!unsafe { Date::new_unchecked(0b0000_0000_0000_0001) }.is_valid());
        // The Month field is 13.
        assert!(!unsafe { Date::new_unchecked(0b0000_0001_1010_0001) }.is_valid());
    }

    #[test]
    fn to_raw() {
        assert_eq!(Date::MIN.to_raw(), 0b0000_0000_0010_0001);
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            Date::new(0b0010_1101_0111_1010).unwrap().to_raw(),
            0b0010_1101_0111_1010
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            Date::new(0b0100_1101_0111_0001).unwrap().to_raw(),
            0b0100_1101_0111_0001
        );
        assert_eq!(Date::MAX.to_raw(), 0b1111_1111_1001_1111);
    }

    #[test]
    const fn to_raw_is_const_fn() {
        const _: u16 = Date::MIN.to_raw();
    }

    #[test]
    fn year() {
        assert_eq!(Date::MIN.year(), 1980);
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(Date::new(0b0010_1101_0111_1010).unwrap().year(), 2002);
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(Date::new(0b0100_1101_0111_0001).unwrap().year(), 2018);
        assert_eq!(Date::MAX.year(), 2107);
    }

    #[test]
    const fn year_is_const_fn() {
        const _: u16 = Date::MIN.year();
    }

    #[test]
    fn month() {
        assert_eq!(Date::MIN.month(), Month::January);
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            Date::new(0b0010_1101_0111_1010).unwrap().month(),
            Month::November
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            Date::new(0b0100_1101_0111_0001).unwrap().month(),
            Month::November
        );
        assert_eq!(Date::MAX.month(), Month::December);
    }

    #[test]
    fn day() {
        assert_eq!(Date::MIN.day(), 1);
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(Date::new(0b0010_1101_0111_1010).unwrap().day(), 26);
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(Date::new(0b0100_1101_0111_0001).unwrap().day(), 17);
        assert_eq!(Date::MAX.day(), 31);
    }

    #[test]
    fn default() {
        assert_eq!(Date::default(), Date::MIN);
    }
}
