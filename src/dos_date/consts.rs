// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Constants for [`Date`].

use super::Date;

impl Date {
    /// The smallest value that can be represented by the MS-DOS date.
    ///
    /// This is "1980-01-01".
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::{Date, time::macros::date};
    /// #
    /// assert_eq!(Date::MIN, Date::from_date(date!(1980-01-01)).unwrap());
    /// ```
    // SAFETY: the given MS-DOS date is valid as the smallest MS-DOS date.
    pub const MIN: Self = unsafe { Self::new_unchecked(0b0000_0000_0010_0001) };

    /// The largest value that can be represented by the MS-DOS date.
    ///
    /// This is "2107-12-31".
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::{Date, time::macros::date};
    /// #
    /// assert_eq!(Date::MAX, Date::from_date(date!(2107-12-31)).unwrap());
    /// ```
    // SAFETY: the given MS-DOS date is valid as the largest MS-DOS date.
    pub const MAX: Self = unsafe { Self::new_unchecked(0b1111_1111_1001_1111) };
}

#[cfg(test)]
mod tests {
    use time::macros::date;

    use super::*;

    #[test]
    fn min() {
        assert_eq!(Date::MIN, Date::from_date(date!(1980-01-01)).unwrap());
    }

    #[test]
    fn max() {
        assert_eq!(Date::MAX, Date::from_date(date!(2107-12-31)).unwrap());
    }
}
