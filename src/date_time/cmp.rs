// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Utilities for comparing and ordering values.

use core::cmp::Ordering;

use super::DateTime;

impl Ord for DateTime {
    fn cmp(&self, other: &Self) -> Ordering {
        if let ordering @ (Ordering::Less | Ordering::Greater) = self.year().cmp(&other.year()) {
            return ordering;
        }
        if let ordering @ (Ordering::Less | Ordering::Greater) = self.month().cmp(&other.month()) {
            return ordering;
        }
        if let ordering @ (Ordering::Less | Ordering::Greater) = self.day().cmp(&other.day()) {
            return ordering;
        }
        if let ordering @ (Ordering::Less | Ordering::Greater) = self.hour().cmp(&other.hour()) {
            return ordering;
        }
        if let ordering @ (Ordering::Less | Ordering::Greater) = self.minute().cmp(&other.minute())
        {
            return ordering;
        }
        self.second().cmp(&other.second())
    }
}

impl PartialOrd for DateTime {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use time::macros::datetime;

    use super::*;

    #[test]
    fn equality() {
        assert_eq!(DateTime::MIN, DateTime::MIN);
        assert_ne!(DateTime::MIN, DateTime::MAX);
        assert_ne!(DateTime::MAX, DateTime::MIN);
        assert_eq!(DateTime::MAX, DateTime::MAX);
    }

    #[test]
    fn order() {
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        let dt = DateTime::try_from(datetime!(2018-11-17 10:38:30)).unwrap();

        assert_eq!(
            dt.cmp(&DateTime::try_from(datetime!(2018-11-17 10:38:30)).unwrap()),
            Ordering::Equal
        );

        // Tests the order of years.
        assert!(dt < DateTime::try_from(datetime!(2019-11-17 10:38:30)).unwrap());
        assert!(dt > DateTime::try_from(datetime!(2017-11-17 10:38:30)).unwrap());

        // Tests the order of months.
        assert!(dt < DateTime::try_from(datetime!(2018-12-17 10:38:30)).unwrap());
        assert!(dt > DateTime::try_from(datetime!(2018-10-17 10:38:30)).unwrap());

        // Tests the order of days.
        assert!(dt < DateTime::try_from(datetime!(2018-11-18 10:38:30)).unwrap());
        assert!(dt > DateTime::try_from(datetime!(2018-11-16 10:38:30)).unwrap());

        // Tests the order of hours.
        assert!(dt < DateTime::try_from(datetime!(2018-11-17 11:38:30)).unwrap());
        assert!(dt > DateTime::try_from(datetime!(2018-11-17 09:38:30)).unwrap());

        // Tests the order of minutes.
        assert!(dt < DateTime::try_from(datetime!(2018-11-17 10:39:30)).unwrap());
        assert!(dt > DateTime::try_from(datetime!(2018-11-17 10:37:30)).unwrap());

        // Tests the order of seconds.
        assert!(dt < DateTime::try_from(datetime!(2018-11-17 10:38:32)).unwrap());
        assert_eq!(
            dt.cmp(&DateTime::try_from(datetime!(2018-11-17 10:38:31)).unwrap()),
            Ordering::Equal
        );
        assert!(dt > DateTime::try_from(datetime!(2018-11-17 10:38:29)).unwrap());
        assert!(dt > DateTime::try_from(datetime!(2018-11-17 10:38:28)).unwrap());
    }
}
