// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! The [MS-DOS time].
//!
//! [MS-DOS time]: https://learn.microsoft.com/en-us/windows/win32/sysinfo/ms-dos-date-and-time

mod cmp;
mod consts;
mod convert;
mod fmt;

/// `Time` is a type that represents the [MS-DOS time].
///
/// This is a packed 16-bit unsigned integer value that specify the time an
/// MS-DOS file was last written to, and is used as timestamps such as [FAT] or
/// [ZIP] file format.
///
/// <div class="warning">
///
/// The resolution of MS-DOS time is 2 seconds.
///
/// </div>
///
/// See the [format specification] for [Kaitai Struct] for more details on the
/// structure of the MS-DOS time.
///
/// [MS-DOS time]: https://learn.microsoft.com/en-us/windows/win32/sysinfo/ms-dos-date-and-time
/// [FAT]: https://en.wikipedia.org/wiki/File_Allocation_Table
/// [ZIP]: https://en.wikipedia.org/wiki/ZIP_(file_format)
/// [format specification]: https://formats.kaitai.io/dos_datetime/
/// [Kaitai Struct]: https://kaitai.io/
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Time(u16);

impl Time {
    #[allow(clippy::missing_panics_doc)]
    /// Creates a new `Time` with the given MS-DOS time.
    ///
    /// Returns [`None`] if the given MS-DOS time is not a valid MS-DOS time.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::Time;
    /// #
    /// assert_eq!(Time::new(u16::MIN), Some(Time::MIN));
    /// assert_eq!(Time::new(0b1011_1111_0111_1101), Some(Time::MAX));
    ///
    /// // The DoubleSeconds field is 30.
    /// assert_eq!(Time::new(0b0000_0000_0001_1110), None);
    /// ```
    #[must_use]
    pub fn new(time: u16) -> Option<Self> {
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
        let time = time::Time::from_hms(hour, minute, second).ok()?;
        let time = Self::from_time(time);
        Some(time)
    }

    /// Creates a new `Time` with the given MS-DOS time.
    ///
    /// # Safety
    ///
    /// The given MS-DOS time must be a valid MS-DOS time.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::Time;
    /// #
    /// assert_eq!(unsafe { Time::new_unchecked(u16::MIN) }, Time::MIN);
    /// assert_eq!(
    ///     unsafe { Time::new_unchecked(0b1011_1111_0111_1101) },
    ///     Time::MAX
    /// );
    /// ```
    #[must_use]
    pub const unsafe fn new_unchecked(time: u16) -> Self {
        Self(time)
    }

    /// Creates a new `Time` with the given [`time::Time`].
    ///
    /// <div class="warning">
    ///
    /// The resolution of MS-DOS time is 2 seconds. So this method rounds
    /// towards zero, truncating any fractional part of the exact result of
    /// dividing seconds by 2.
    ///
    /// </div>
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::{
    /// #     Time,
    /// #     time::{self, macros::time},
    /// # };
    /// #
    /// assert_eq!(Time::from_time(time::Time::MIDNIGHT), Time::MIN);
    /// assert_eq!(Time::from_time(time!(23:59:58)), Time::MAX);
    /// ```
    #[must_use]
    pub fn from_time(time: time::Time) -> Self {
        let (hour, minute, second) = (
            u16::from(time.hour()),
            u16::from(time.minute()),
            u16::from(time.second() / 2),
        );
        // <https://learn.microsoft.com/en-us/windows/win32/fileio/exfat-specification#7481-doubleseconds-field>.
        let second = second.min(29);
        let time = (hour << 11) | (minute << 5) | second;
        // SAFETY: `time` is a valid as the MS-DOS time.
        unsafe { Self::new_unchecked(time) }
    }

    /// Returns the MS-DOS time of this `Time` as the underlying [`u16`] value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::Time;
    /// #
    /// assert_eq!(Time::MIN.to_raw(), u16::MIN);
    /// assert_eq!(Time::MAX.to_raw(), 0b1011_1111_0111_1101);
    /// ```
    #[must_use]
    pub const fn to_raw(self) -> u16 {
        self.0
    }

    #[allow(clippy::missing_panics_doc)]
    /// Gets the hour of this `Time`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::Time;
    /// #
    /// assert_eq!(Time::MIN.hour(), 0);
    /// assert_eq!(Time::MAX.hour(), 23);
    /// ```
    #[must_use]
    pub fn hour(self) -> u8 {
        (self.to_raw() >> 11)
            .try_into()
            .expect("hour should be in the range of `u8`")
    }

    #[allow(clippy::missing_panics_doc)]
    /// Gets the minute of this `Time`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::Time;
    /// #
    /// assert_eq!(Time::MIN.minute(), 0);
    /// assert_eq!(Time::MAX.minute(), 59);
    /// ```
    #[must_use]
    pub fn minute(self) -> u8 {
        ((self.to_raw() >> 5) & 0x3f)
            .try_into()
            .expect("minute should be in the range of `u8`")
    }

    #[allow(clippy::missing_panics_doc)]
    /// Gets the second of this `Time`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::Time;
    /// #
    /// assert_eq!(Time::MIN.second(), 0);
    /// assert_eq!(Time::MAX.second(), 58);
    /// ```
    #[must_use]
    pub fn second(self) -> u8 {
        ((self.to_raw() & 0x1f) * 2)
            .try_into()
            .expect("second should be in the range of `u8`")
    }
}

impl Default for Time {
    /// Returns the default value of "00:00:00".
    ///
    /// Equivalent to [`Time::MIN`] except that it is not callable in const
    /// contexts.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::Time;
    /// #
    /// assert_eq!(Time::default(), Time::MIN);
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

    use time::macros::time;

    use super::*;

    #[test]
    fn size_of() {
        assert_eq!(mem::size_of::<Time>(), mem::size_of::<u16>());
    }

    #[test]
    fn align_of() {
        assert_eq!(mem::align_of::<Time>(), mem::align_of::<u16>());
    }

    #[test]
    fn clone() {
        assert_eq!(Time::MIN.clone(), Time::MIN);
    }

    #[test]
    fn copy() {
        let a = Time::MIN;
        let b = a;
        assert_eq!(a, b);
    }

    #[cfg(feature = "std")]
    #[test]
    fn hash() {
        assert_ne!(
            {
                let mut hasher = DefaultHasher::new();
                Time::MIN.hash(&mut hasher);
                hasher.finish()
            },
            {
                let mut hasher = DefaultHasher::new();
                Time::MAX.hash(&mut hasher);
                hasher.finish()
            }
        );
    }

    #[test]
    fn new() {
        assert_eq!(Time::new(u16::MIN).unwrap(), Time::MIN);
        assert_eq!(Time::new(0b1011_1111_0111_1101).unwrap(), Time::MAX);
    }

    #[test]
    fn new_with_invalid_time() {
        // The DoubleSeconds field is 30.
        assert!(Time::new(0b0000_0000_0001_1110).is_none());
        // The Minute field is 60.
        assert!(Time::new(0b0000_0111_1000_0000).is_none());
        // The Hour field is 24.
        assert!(Time::new(0b1100_0000_0000_0000).is_none());
    }

    #[test]
    fn new_unchecked() {
        assert_eq!(unsafe { Time::new_unchecked(u16::MIN) }, Time::MIN);
        assert_eq!(
            unsafe { Time::new_unchecked(0b1011_1111_0111_1101) },
            Time::MAX
        );
    }

    #[test]
    const fn new_unchecked_is_const_fn() {
        const _: Time = unsafe { Time::new_unchecked(u16::MIN) };
    }

    #[test]
    fn from_time() {
        assert_eq!(Time::from_time(time::Time::MIDNIGHT), Time::MIN);
        assert_eq!(Time::from_time(time!(00:00:01)), Time::MIN);
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            Time::from_time(time!(19:25:00)),
            Time::new(0b1001_1011_0010_0000).unwrap()
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            Time::from_time(time!(10:38:30)),
            Time::new(0b0101_0100_1100_1111).unwrap()
        );
        assert_eq!(Time::from_time(time!(23:59:58)), Time::MAX);
        assert_eq!(Time::from_time(time!(23:59:59)), Time::MAX);
    }

    #[test]
    fn to_raw() {
        assert_eq!(Time::MIN.to_raw(), u16::MIN);
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            Time::new(0b1001_1011_0010_0000).unwrap().to_raw(),
            0b1001_1011_0010_0000
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            Time::new(0b0101_0100_1100_1111).unwrap().to_raw(),
            0b0101_0100_1100_1111
        );
        assert_eq!(Time::MAX.to_raw(), 0b1011_1111_0111_1101);
    }

    #[test]
    const fn to_raw_is_const_fn() {
        const _: u16 = Time::MIN.to_raw();
    }

    #[test]
    fn hour() {
        assert_eq!(Time::MIN.hour(), u8::MIN);
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(Time::new(0b1001_1011_0010_0000).unwrap().hour(), 19);
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(Time::new(0b0101_0100_1100_1111).unwrap().hour(), 10);
        assert_eq!(Time::MAX.hour(), 23);
    }

    #[test]
    fn minute() {
        assert_eq!(Time::MIN.minute(), u8::MIN);
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(Time::new(0b1001_1011_0010_0000).unwrap().minute(), 25);
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(Time::new(0b0101_0100_1100_1111).unwrap().minute(), 38);
        assert_eq!(Time::MAX.minute(), 59);
    }

    #[test]
    fn second() {
        assert_eq!(Time::MIN.second(), u8::MIN);
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(Time::new(0b1001_1011_0010_0000).unwrap().second(), u8::MIN);
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(Time::new(0b0101_0100_1100_1111).unwrap().second(), 30);
        assert_eq!(Time::MAX.second(), 58);
    }

    #[test]
    fn default() {
        assert_eq!(Time::default(), Time::MIN);
    }
}
