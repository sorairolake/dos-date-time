// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Implementations of conversions between [`Time`] and other types.

#[cfg(feature = "chrono")]
use chrono::{NaiveTime, Timelike};
#[cfg(feature = "jiff")]
use jiff::civil;

use super::Time;

impl From<Time> for u16 {
    fn from(time: Time) -> Self {
        time.to_raw()
    }
}

#[expect(clippy::fallible_impl_from)]
impl From<Time> for time::Time {
    /// Converts a `Time` to a [`time::Time`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::{Time, time::macros::time};
    /// #
    /// assert_eq!(time::Time::from(Time::MIN), time!(00:00:00));
    /// assert_eq!(time::Time::from(Time::MAX), time!(23:59:58));
    /// ```
    fn from(time: Time) -> Self {
        let (hour, minute, second) = (time.hour(), time.minute(), time.second());
        Self::from_hms(hour, minute, second).unwrap()
    }
}

#[cfg(feature = "chrono")]
#[expect(clippy::fallible_impl_from)]
impl From<Time> for NaiveTime {
    /// Converts a `Time` to a [`NaiveTime`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::{Time, chrono::NaiveTime};
    /// #
    /// assert_eq!(NaiveTime::from(Time::MIN), "00:00:00".parse().unwrap());
    /// assert_eq!(NaiveTime::from(Time::MAX), "23:59:58".parse().unwrap());
    /// ```
    fn from(time: Time) -> Self {
        let (hour, minute, second) = (
            time.hour().into(),
            time.minute().into(),
            time.second().into(),
        );
        Self::from_hms_opt(hour, minute, second).unwrap()
    }
}

#[cfg(feature = "jiff")]
#[expect(clippy::fallible_impl_from)]
impl From<Time> for civil::Time {
    /// Converts a `Time` to a [`civil::Time`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::{Time, jiff::civil};
    /// #
    /// assert_eq!(civil::Time::from(Time::MIN), "00:00:00".parse().unwrap());
    /// assert_eq!(civil::Time::from(Time::MAX), "23:59:58".parse().unwrap());
    /// ```
    fn from(time: Time) -> Self {
        let (hour, minute, second) = (
            time.hour().try_into().unwrap(),
            time.minute().try_into().unwrap(),
            time.second().try_into().unwrap(),
        );
        civil::time(hour, minute, second, i32::default())
    }
}

impl From<time::Time> for Time {
    /// Converts a [`time::Time`] to a `Time`.
    ///
    /// <div class="warning">
    ///
    /// This method may round towards zero, truncating more precise times that a
    /// `Time` cannot store.
    ///
    /// </div>
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::{Time, time::macros::time};
    /// #
    /// assert_eq!(Time::from(time!(00:00:00)), Time::MIN);
    /// assert_eq!(Time::from(time!(23:59:58)), Time::MAX);
    /// ```
    fn from(time: time::Time) -> Self {
        Self::from_time(time)
    }
}

#[cfg(feature = "chrono")]
#[expect(clippy::fallible_impl_from)]
impl From<NaiveTime> for Time {
    /// Converts a [`NaiveTime`] to a `Time`.
    ///
    /// <div class="warning">
    ///
    /// This method may round towards zero, truncating more precise times that a
    /// `Time` cannot store.
    ///
    /// </div>
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::{Time, chrono::NaiveTime};
    /// #
    /// let time: NaiveTime = "00:00:00".parse().unwrap();
    /// assert_eq!(Time::from(time), Time::MIN);
    /// let time: NaiveTime = "23:59:58".parse().unwrap();
    /// assert_eq!(Time::from(time), Time::MAX);
    /// ```
    fn from(time: NaiveTime) -> Self {
        let (hour, minute, second) = (
            time.hour().try_into().unwrap(),
            time.minute().try_into().unwrap(),
            time.second().try_into().unwrap(),
        );
        let time = time::Time::from_hms(hour, minute, second).unwrap();
        Self::from_time(time)
    }
}

#[cfg(feature = "jiff")]
#[expect(clippy::fallible_impl_from)]
impl From<civil::Time> for Time {
    /// Converts a [`civil::Time`] to a `Time`.
    ///
    /// <div class="warning">
    ///
    /// This method may round towards zero, truncating more precise times that a
    /// `Time` cannot store.
    ///
    /// </div>
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::{Time, jiff::civil};
    /// #
    /// let time: civil::Time = "00:00:00".parse().unwrap();
    /// assert_eq!(Time::from(time), Time::MIN);
    /// let time: civil::Time = "23:59:58".parse().unwrap();
    /// assert_eq!(Time::from(time), Time::MAX);
    /// ```
    fn from(time: civil::Time) -> Self {
        let (hour, minute, second) = (
            time.hour().try_into().unwrap(),
            time.minute().try_into().unwrap(),
            time.second().try_into().unwrap(),
        );
        let time = time::Time::from_hms(hour, minute, second).unwrap();
        Self::from_time(time)
    }
}

#[cfg(test)]
mod tests {
    use time::macros::time;

    use super::*;

    #[test]
    fn from_time_to_u16() {
        assert_eq!(u16::from(Time::MIN), u16::MIN);
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            u16::from(Time::new(0b1001_1011_0010_0000).unwrap()),
            0b1001_1011_0010_0000
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            u16::from(Time::new(0b0101_0100_1100_1111).unwrap()),
            0b0101_0100_1100_1111
        );
        assert_eq!(u16::from(Time::MAX), 0b1011_1111_0111_1101);
    }

    #[test]
    fn from_time_to_time_time() {
        assert_eq!(time::Time::from(Time::MIN), time::Time::MIDNIGHT);
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            time::Time::from(Time::new(0b1001_1011_0010_0000).unwrap()),
            time!(19:25:00)
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            time::Time::from(Time::new(0b0101_0100_1100_1111).unwrap()),
            time!(10:38:30)
        );
        assert_eq!(time::Time::from(Time::MAX), time!(23:59:58));
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn from_time_to_chrono_naive_time() {
        assert_eq!(NaiveTime::from(Time::MIN), NaiveTime::MIN);
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            NaiveTime::from(Time::new(0b1001_1011_0010_0000).unwrap()),
            NaiveTime::from_hms_opt(19, 25, 0).unwrap()
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            NaiveTime::from(Time::new(0b0101_0100_1100_1111).unwrap()),
            NaiveTime::from_hms_opt(10, 38, 30).unwrap()
        );
        assert_eq!(
            NaiveTime::from(Time::MAX),
            NaiveTime::from_hms_opt(23, 59, 58).unwrap()
        );
    }

    #[cfg(feature = "jiff")]
    #[test]
    fn from_time_to_jiff_civil_time() {
        assert_eq!(civil::Time::from(Time::MIN), civil::Time::MIN);
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            civil::Time::from(Time::new(0b1001_1011_0010_0000).unwrap()),
            civil::time(19, 25, 0, 0)
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            civil::Time::from(Time::new(0b0101_0100_1100_1111).unwrap()),
            civil::time(10, 38, 30, 0)
        );
        assert_eq!(civil::Time::from(Time::MAX), civil::time(23, 59, 58, 0));
    }

    #[test]
    fn from_time_time_to_time() {
        assert_eq!(Time::from(time::Time::MIDNIGHT), Time::MIN);
        assert_eq!(Time::from(time!(00:00:01)), Time::MIN);
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            Time::from(time!(19:25:00)),
            Time::new(0b1001_1011_0010_0000).unwrap()
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            Time::from(time!(10:38:30)),
            Time::new(0b0101_0100_1100_1111).unwrap()
        );
        assert_eq!(Time::from(time!(23:59:58)), Time::MAX);
        assert_eq!(Time::from(time!(23:59:59)), Time::MAX);
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn from_chrono_naive_time_to_time() {
        assert_eq!(Time::from(NaiveTime::MIN), Time::MIN);
        assert_eq!(
            Time::from(NaiveTime::from_hms_opt(0, 0, 1).unwrap()),
            Time::MIN
        );
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            Time::from(NaiveTime::from_hms_opt(19, 25, 0).unwrap()),
            Time::new(0b1001_1011_0010_0000).unwrap()
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            Time::from(NaiveTime::from_hms_opt(10, 38, 30).unwrap()),
            Time::new(0b0101_0100_1100_1111).unwrap()
        );
        assert_eq!(
            Time::from(NaiveTime::from_hms_opt(23, 59, 58).unwrap()),
            Time::MAX
        );
        assert_eq!(
            Time::from(NaiveTime::from_hms_opt(23, 59, 59).unwrap()),
            Time::MAX
        );
    }

    #[cfg(feature = "jiff")]
    #[test]
    fn from_jiff_civil_time_to_time() {
        assert_eq!(Time::from(civil::Time::MIN), Time::MIN);
        assert_eq!(Time::from(civil::time(0, 0, 1, 0)), Time::MIN);
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            Time::from(civil::time(19, 25, 0, 0)),
            Time::new(0b1001_1011_0010_0000).unwrap()
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            Time::from(civil::time(10, 38, 30, 0)),
            Time::new(0b0101_0100_1100_1111).unwrap()
        );
        assert_eq!(Time::from(civil::time(23, 59, 58, 0)), Time::MAX);
        assert_eq!(Time::from(civil::time(23, 59, 59, 0)), Time::MAX);
    }
}
