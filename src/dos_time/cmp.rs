// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Utilities for comparing and ordering values.

#[cfg(test)]
mod tests {
    use core::cmp::Ordering;

    use time::macros::time;

    use super::super::Time;

    #[test]
    fn equality() {
        assert_eq!(Time::MIN, Time::MIN);
        assert_ne!(Time::MIN, Time::MAX);
        assert_ne!(Time::MAX, Time::MIN);
        assert_eq!(Time::MAX, Time::MAX);
    }

    #[test]
    fn order() {
        // <https://github.com/zip-rs/zip/blob/v0.6.4/src/types.rs#L553-L569>.
        let time = Time::from_time(time!(10:38:30));

        assert_eq!(time.cmp(&Time::from_time(time!(10:38:30))), Ordering::Equal);

        // Tests the order of hours.
        assert!(time < Time::from_time(time!(11:38:30)));
        assert!(time > Time::from_time(time!(09:38:30)));

        // Tests the order of minutes.
        assert!(time < Time::from_time(time!(10:39:30)));
        assert!(time > Time::from_time(time!(10:37:30)));

        // Tests the order of seconds.
        assert!(time < Time::from_time(time!(10:38:32)));
        assert_eq!(time.cmp(&Time::from_time(time!(10:38:31))), Ordering::Equal);
        assert!(time > Time::from_time(time!(10:38:29)));
        assert!(time > Time::from_time(time!(10:38:28)));
    }
}
