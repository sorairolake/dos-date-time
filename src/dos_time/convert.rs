// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Implementations of conversions between [`Time`] and other types.

#[cfg(feature = "chrono")]
use chrono::{NaiveTime, Timelike};
#[cfg(feature = "jiff")]
use jiff::civil;

use super::Time;

impl From<Time> for time::Time {
    /// Converts a `Time` to a [`time::Time`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::{
    /// #     Time,
    /// #     time::{self, macros::time},
    /// # };
    /// #
    /// assert_eq!(time::Time::from(Time::MIN), time::Time::MIDNIGHT);
    /// assert_eq!(time::Time::from(Time::MAX), time!(23:59:58));
    /// ```
    fn from(time: Time) -> Self {
        let (hour, minute, second) = (time.hour(), time.minute(), time.second());
        Self::from_hms(hour, minute, second).expect("time should be in the range of `time::Time`")
    }
}

#[cfg(feature = "chrono")]
impl From<Time> for NaiveTime {
    /// Converts a `Time` to a [`NaiveTime`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::{Time, chrono::NaiveTime};
    /// #
    /// assert_eq!(NaiveTime::from(Time::MIN), NaiveTime::MIN);
    /// assert_eq!(
    ///     NaiveTime::from(Time::MAX),
    ///     "23:59:58".parse::<NaiveTime>().unwrap()
    /// );
    /// ```
    fn from(time: Time) -> Self {
        let (hour, minute, second) = (
            time.hour().into(),
            time.minute().into(),
            time.second().into(),
        );
        Self::from_hms_opt(hour, minute, second)
            .expect("time should be in the range of `NaiveTime`")
    }
}

#[cfg(feature = "jiff")]
impl From<Time> for civil::Time {
    /// Converts a `Time` to a [`civil::Time`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::{Time, jiff::civil};
    /// #
    /// assert_eq!(civil::Time::from(Time::MIN), civil::Time::MIN);
    /// assert_eq!(civil::Time::from(Time::MAX), civil::time(23, 59, 58, 0));
    /// ```
    fn from(time: Time) -> Self {
        let (hour, minute, second) = (
            time.hour()
                .try_into()
                .expect("hour should be in the range of `i8`"),
            time.minute()
                .try_into()
                .expect("minute should be in the range of `i8`"),
            time.second()
                .try_into()
                .expect("second should be in the range of `i8`"),
        );
        civil::time(hour, minute, second, i32::default())
    }
}

impl From<time::Time> for Time {
    /// Converts a [`time::Time`] to a `Time`.
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
    /// assert_eq!(Time::from(time::Time::MIDNIGHT), Time::MIN);
    /// assert_eq!(Time::from(time!(23:59:58)), Time::MAX);
    /// ```
    fn from(time: time::Time) -> Self {
        Self::from_time(time)
    }
}

#[cfg(feature = "chrono")]
impl From<NaiveTime> for Time {
    /// Converts a [`NaiveTime`] to a `Time`.
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
    /// # use dos_date_time::{Time, chrono::NaiveTime};
    /// #
    /// assert_eq!(Time::from(NaiveTime::MIN), Time::MIN);
    /// assert_eq!(
    ///     Time::from("23:59:58".parse::<NaiveTime>().unwrap()),
    ///     Time::MAX
    /// );
    /// ```
    fn from(time: NaiveTime) -> Self {
        let (hour, minute, second) = (
            time.hour()
                .try_into()
                .expect("hour should be in the range of `u8`"),
            time.minute()
                .try_into()
                .expect("minute should be in the range of `u8`"),
            time.second()
                .try_into()
                .expect("second should be in the range of `u8`"),
        );
        let time = time::Time::from_hms(hour, minute, second)
            .expect("time should be in the range of `time::Time`");
        Self::from_time(time)
    }
}

#[cfg(feature = "jiff")]
impl From<civil::Time> for Time {
    /// Converts a [`civil::Time`] to a `Time`.
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
    /// # use dos_date_time::{Time, jiff::civil};
    /// #
    /// assert_eq!(Time::from(civil::Time::MIN), Time::MIN);
    /// assert_eq!(Time::from(civil::time(23, 59, 58, 0)), Time::MAX);
    /// ```
    fn from(time: civil::Time) -> Self {
        let (hour, minute, second) = (
            time.hour()
                .try_into()
                .expect("hour should be in the range of `u8`"),
            time.minute()
                .try_into()
                .expect("minute should be in the range of `u8`"),
            time.second()
                .try_into()
                .expect("second should be in the range of `u8`"),
        );
        let time = time::Time::from_hms(hour, minute, second)
            .expect("time should be in the range of `time::Time`");
        Self::from_time(time)
    }
}

#[cfg(test)]
mod tests {
    use time::macros::time;

    use super::*;

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
            "19:25:00".parse::<NaiveTime>().unwrap()
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            NaiveTime::from(Time::new(0b0101_0100_1100_1111).unwrap()),
            "10:38:30".parse::<NaiveTime>().unwrap()
        );
        assert_eq!(
            NaiveTime::from(Time::MAX),
            "23:59:58".parse::<NaiveTime>().unwrap()
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
            Time::from("00:00:01".parse::<NaiveTime>().unwrap()),
            Time::MIN
        );
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            Time::from("19:25:00".parse::<NaiveTime>().unwrap()),
            Time::new(0b1001_1011_0010_0000).unwrap()
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            Time::from("10:38:30".parse::<NaiveTime>().unwrap()),
            Time::new(0b0101_0100_1100_1111).unwrap()
        );
        assert_eq!(
            Time::from("23:59:58".parse::<NaiveTime>().unwrap()),
            Time::MAX
        );
        assert_eq!(
            Time::from("23:59:59".parse::<NaiveTime>().unwrap()),
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
