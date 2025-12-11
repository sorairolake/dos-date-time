// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Constants for [`DateTime`].

use super::DateTime;

impl DateTime {
    /// The smallest value that can be represented by the file time.
    ///
    /// This is "1980-01-01 00:00:00".
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::{
    /// #     DateTime,
    /// #     time::{Time, macros::date},
    /// # };
    /// #
    /// assert_eq!(
    ///     DateTime::MIN,
    ///     DateTime::from_date_time(date!(1980 - 01 - 01), Time::MIDNIGHT).unwrap()
    /// );
    /// ```
    // SAFETY: the given MS-DOS date and time are valid as the smallest MS-DOS date
    // and time.
    pub const MIN: Self = unsafe { Self::new_unchecked(0b0000_0000_0010_0001, u16::MIN) };

    /// The largest value that can be represented by the file time.
    ///
    /// This is "2107-12-31 23:59:58".
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::{
    /// #     DateTime,
    /// #     time::macros::{date, time},
    /// # };
    /// #
    /// assert_eq!(
    ///     DateTime::MAX,
    ///     DateTime::from_date_time(date!(2107 - 12 - 31), time!(23:59:58)).unwrap()
    /// );
    /// ```
    // SAFETY: the given MS-DOS date and time are valid as the largest MS-DOS date
    // and time.
    pub const MAX: Self =
        unsafe { Self::new_unchecked(0b1111_1111_1001_1111, 0b1011_1111_0111_1101) };
}

#[cfg(test)]
mod tests {
    use time::{
        Time,
        macros::{date, time},
    };

    use super::*;

    #[test]
    fn min() {
        assert_eq!(
            DateTime::MIN,
            DateTime::from_date_time(date!(1980 - 01 - 01), Time::MIDNIGHT).unwrap()
        );
    }

    #[test]
    fn max() {
        assert_eq!(
            DateTime::MAX,
            DateTime::from_date_time(date!(2107 - 12 - 31), time!(23:59:58)).unwrap()
        );
    }
}
