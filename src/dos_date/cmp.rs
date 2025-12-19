// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Utilities for comparing and ordering values.

#[cfg(test)]
mod tests {
    use core::cmp::Ordering;

    use time::macros::date;

    use super::super::Date;

    #[test]
    fn equality() {
        assert_eq!(Date::MIN, Date::MIN);
        assert_ne!(Date::MIN, Date::MAX);
        assert_ne!(Date::MAX, Date::MIN);
        assert_eq!(Date::MAX, Date::MAX);
    }

    #[test]
    fn order() {
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        let date = Date::from_date(date!(2018-11-17)).unwrap();

        assert_eq!(
            date.cmp(&Date::from_date(date!(2018-11-17)).unwrap()),
            Ordering::Equal
        );

        // Tests the order of years.
        assert!(date < Date::from_date(date!(2019-11-17)).unwrap());
        assert!(date > Date::from_date(date!(2017-11-17)).unwrap());

        // Tests the order of months.
        assert!(date < Date::from_date(date!(2018-12-17)).unwrap());
        assert!(date > Date::from_date(date!(2018-10-17)).unwrap());

        // Tests the order of days.
        assert!(date < Date::from_date(date!(2018-11-18)).unwrap());
        assert!(date > Date::from_date(date!(2018-11-16)).unwrap());
    }
}
