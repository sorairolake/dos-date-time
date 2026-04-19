// SPDX-FileCopyrightText: 2026 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Implementations of [`rand`] for [`DateTime`].

use rand::{
    Rng, RngExt,
    distr::{Distribution, StandardUniform},
};

use super::DateTime;

impl Distribution<DateTime> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DateTime {
        DateTime::new(rng.random(), rng.random())
    }
}

#[cfg(test)]
mod tests {
    use rand_pcg::{Pcg64Mcg, rand_core::SeedableRng};
    use time::macros::{date, time};

    use super::*;

    #[test]
    fn sample() {
        let mut rng = Pcg64Mcg::from_seed(Default::default());
        let buf: [DateTime; 8] = rng.random();
        assert_eq!(
            buf,
            [
                DateTime::from_date_time(date!(2092-09-07), time!(03:57:08)).unwrap(),
                DateTime::from_date_time(date!(2091-12-13), time!(03:57:56)).unwrap(),
                DateTime::from_date_time(date!(2020-01-11), time!(12:35:52)).unwrap(),
                DateTime::from_date_time(date!(2096-06-10), time!(11:45:22)).unwrap(),
                DateTime::from_date_time(date!(2006-02-11), time!(16:41:18)).unwrap(),
                DateTime::from_date_time(date!(2077-09-23), time!(19:52:04)).unwrap(),
                DateTime::from_date_time(date!(2031-05-11), time!(07:14:48)).unwrap(),
                DateTime::from_date_time(date!(2010-08-07), time!(10:58:28)).unwrap()
            ]
        );
    }

    #[test]
    fn is_valid() {
        let rng = Pcg64Mcg::from_seed(Default::default());
        for dt in rng.random_iter::<DateTime>().take(1 << 14) {
            assert!(dt.is_valid());
        }
    }
}
