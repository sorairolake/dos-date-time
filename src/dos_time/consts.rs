// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Constants for [`Time`].

use super::Time;

impl Time {
    /// The smallest value that can be represented by the MS-DOS time.
    ///
    /// This is "00:00:00".
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::{Time, time};
    /// #
    /// assert_eq!(Time::MIN, Time::from_time(time::Time::MIDNIGHT));
    /// ```
    // SAFETY: the given MS-DOS time is valid as the smallest MS-DOS time.
    pub const MIN: Self = unsafe { Self::new_unchecked(u16::MIN) };

    /// The largest value that can be represented by the MS-DOS time.
    ///
    /// This is "23:59:58".
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::{Time, time::macros::time};
    /// #
    /// assert_eq!(Time::MAX, Time::from_time(time!(23:59:58)));
    /// ```
    // SAFETY: the given MS-DOS time is valid as the largest MS-DOS time.
    pub const MAX: Self = unsafe { Self::new_unchecked(0b1011_1111_0111_1101) };
}

#[cfg(test)]
mod tests {
    use time::macros::time;

    use super::*;

    #[test]
    fn min() {
        assert_eq!(Time::MIN, Time::from_time(time::Time::MIDNIGHT));
    }

    #[test]
    fn max() {
        assert_eq!(Time::MAX, Time::from_time(time!(23:59:58)));
    }
}
