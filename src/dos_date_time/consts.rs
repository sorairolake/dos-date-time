// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Constants for [`DateTime`].

use super::DateTime;
use crate::{Date, Time};

impl DateTime {
    /// The smallest value that can be represented by MS-DOS date and time.
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
    ///     DateTime::from_date_time(date!(1980-01-01), Time::MIDNIGHT).unwrap()
    /// );
    /// ```
    pub const MIN: Self = Self::new(Date::MIN, Time::MIN);

    /// The largest value that can be represented by MS-DOS date and time.
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
    ///     DateTime::from_date_time(date!(2107-12-31), time!(23:59:58)).unwrap()
    /// );
    /// ```
    pub const MAX: Self = Self::new(Date::MAX, Time::MAX);
}

#[cfg(test)]
mod tests {
    use time::macros::{date, time};

    use super::*;

    #[test]
    fn min() {
        assert_eq!(
            DateTime::MIN,
            DateTime::from_date_time(date!(1980-01-01), time::Time::MIDNIGHT).unwrap()
        );
    }

    #[test]
    fn max() {
        assert_eq!(
            DateTime::MAX,
            DateTime::from_date_time(date!(2107-12-31), time!(23:59:58)).unwrap()
        );
    }
}
