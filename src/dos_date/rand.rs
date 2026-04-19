// SPDX-FileCopyrightText: 2026 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Implementations of [`rand`] for [`Date`].

use rand::{
    Rng, RngExt,
    distr::{Distribution, StandardUniform},
};
use time::{Duration, Month};

use super::Date;

impl Distribution<Date> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Date {
        let offset = Duration::days(rng.random_range(0..46751));
        let date = time::Date::from_calendar_date(1980, Month::January, 1).unwrap() + offset;
        Date::from_date(date).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use rand_pcg::{Pcg64Mcg, rand_core::SeedableRng};
    use time::macros::date;

    use super::*;

    #[test]
    fn sample() {
        let mut rng = Pcg64Mcg::from_seed(Default::default());
        let buf: [Date; 8] = rng.random();
        assert_eq!(
            buf,
            [
                Date::from_date(date!(2092-09-07)).unwrap(),
                Date::from_date(date!(2001-01-29)).unwrap(),
                Date::from_date(date!(2091-12-13)).unwrap(),
                Date::from_date(date!(2001-02-23)).unwrap(),
                Date::from_date(date!(2020-01-11)).unwrap(),
                Date::from_date(date!(2047-03-09)).unwrap(),
                Date::from_date(date!(2096-06-10)).unwrap(),
                Date::from_date(date!(2042-09-13)).unwrap()
            ]
        );
    }

    #[test]
    fn is_valid() {
        let rng = Pcg64Mcg::from_seed(Default::default());
        for date in rng.random_iter::<Date>().take(1 << 14) {
            assert!(date.is_valid());
        }
    }
}
