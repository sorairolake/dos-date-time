// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Utilities for formatting and printing [`Date`].

use core::fmt;

use super::Date;

impl fmt::Display for Date {
    /// Shows the value of this `Date` in the well-known [RFC 3339 format].
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::Date;
    /// #
    /// assert_eq!(format!("{}", Date::MIN), "1980-01-01");
    /// assert_eq!(format!("{}", Date::MAX), "2107-12-31");
    /// ```
    ///
    /// [RFC 3339 format]: https://datatracker.ietf.org/doc/html/rfc3339#section-5.6
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (year, month, day) = (self.year(), u8::from(self.month()), self.day());
        write!(f, "{year:04}-{month:02}-{day:02}")
    }
}

#[cfg(test)]
mod tests {
    use time::macros::date;

    use super::*;

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", Date::MIN), "Date(33)");
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            format!("{:?}", Date::from_date(date!(2002-11-26)).unwrap()),
            "Date(11642)"
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            format!("{:?}", Date::from_date(date!(2018-11-17)).unwrap()),
            "Date(19825)"
        );
        assert_eq!(format!("{:?}", Date::MAX), "Date(65439)");
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", Date::MIN), "1980-01-01");
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            format!("{}", Date::from_date(date!(2002-11-26)).unwrap()),
            "2002-11-26"
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            format!("{}", Date::from_date(date!(2018-11-17)).unwrap()),
            "2018-11-17"
        );
        assert_eq!(format!("{}", Date::MAX), "2107-12-31");
    }
}
