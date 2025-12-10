// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Utilities for formatting and printing [`DateTime`].

use core::fmt;

use super::DateTime;

impl fmt::Display for DateTime {
    /// Shows the value of this `DateTime` in the well-known [RFC 3339 format].
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::DateTime;
    /// #
    /// assert_eq!(format!("{}", DateTime::MIN), "1980-01-01 00:00:00");
    /// assert_eq!(format!("{}", DateTime::MAX), "2107-12-31 23:59:58");
    /// ```
    ///
    /// [RFC 3339 format]: https://datatracker.ietf.org/doc/html/rfc3339#section-5.6
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (year, month, day) = (self.year(), u8::from(self.month()), self.day());
        let (hour, minute, second) = (self.hour(), self.minute(), self.second());
        write!(
            f,
            "{year:04}-{month:02}-{day:02} {hour:02}:{minute:02}:{second:02}"
        )
    }
}

#[cfg(test)]
mod tests {
    use time::macros::datetime;

    use super::*;

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", DateTime::MIN),
            "DateTime { date: 33, time: 0 }"
        );
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            format!(
                "{:?}",
                DateTime::try_from(datetime!(2002-11-26 19:25:00)).unwrap()
            ),
            "DateTime { date: 11642, time: 39712 }"
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            format!(
                "{:?}",
                DateTime::try_from(datetime!(2018-11-17 10:38:30)).unwrap()
            ),
            "DateTime { date: 19825, time: 21711 }"
        );
        assert_eq!(
            format!("{:?}", DateTime::MAX),
            "DateTime { date: 65439, time: 49021 }"
        );
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", DateTime::MIN), "1980-01-01 00:00:00");
        // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
        assert_eq!(
            format!(
                "{}",
                DateTime::try_from(datetime!(2002-11-26 19:25:00)).unwrap()
            ),
            "2002-11-26 19:25:00"
        );
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        assert_eq!(
            format!(
                "{}",
                DateTime::try_from(datetime!(2018-11-17 10:38:30)).unwrap()
            ),
            "2018-11-17 10:38:30"
        );
        assert_eq!(format!("{}", DateTime::MAX), "2107-12-31 23:59:58");
    }
}
