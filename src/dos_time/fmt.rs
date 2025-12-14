// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Utilities for formatting and printing [`Time`].

use core::fmt;

use super::Time;

impl fmt::Display for Time {
    /// Shows the value of this `Time` in the well-known [RFC 3339 format].
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::Time;
    /// #
    /// assert_eq!(format!("{}", Time::MIN), "00:00:00");
    /// assert_eq!(format!("{}", Time::MAX), "23:59:58");
    /// ```
    ///
    /// [RFC 3339 format]: https://datatracker.ietf.org/doc/html/rfc3339#section-5.6
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (hour, minute, second) = (self.hour(), self.minute(), self.second());
        write!(f, "{hour:02}:{minute:02}:{second:02}")
    }
}

#[cfg(test)]
mod tests {
    use time::macros::time;

    use super::*;

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", Time::MIN), "Time(0)");
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            format!("{:?}", Time::from_time(time!(19:25:00))),
            "Time(39712)"
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            format!("{:?}", Time::from_time(time!(10:38:30))),
            "Time(21711)"
        );
        assert_eq!(format!("{:?}", Time::MAX), "Time(49021)");
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", Time::MIN), "00:00:00");
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(format!("{}", Time::from_time(time!(19:25:00))), "19:25:00");
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(format!("{}", Time::from_time(time!(10:38:30))), "10:38:30");
        assert_eq!(format!("{}", Time::MAX), "23:59:58");
    }
}
