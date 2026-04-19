// SPDX-FileCopyrightText: 2026 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Implementations of [`rand`] for [`Time`].

use rand::{
    Rng, RngExt,
    distr::{Distribution, StandardUniform},
};
use time::Duration;

use super::Time;

impl Distribution<Time> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Time {
        let offset = Duration::seconds(rng.random_range(0..43200) * 2);
        let time = time::Time::MIDNIGHT + offset;
        Time::from_time(time)
    }
}

#[cfg(test)]
mod tests {
    use rand_pcg::{Pcg64Mcg, rand_core::SeedableRng};
    use time::macros::time;

    use super::*;

    #[test]
    fn sample() {
        let mut rng = Pcg64Mcg::from_seed(Default::default());
        let buf: [Time; 8] = rng.random();
        assert_eq!(
            buf,
            [
                Time::from_time(time!(21:07:44)),
                Time::from_time(time!(03:57:08)),
                Time::from_time(time!(20:59:26)),
                Time::from_time(time!(03:57:56)),
                Time::from_time(time!(07:30:18)),
                Time::from_time(time!(12:35:52)),
                Time::from_time(time!(21:49:58)),
                Time::from_time(time!(11:45:22))
            ]
        );
    }

    #[test]
    fn is_valid() {
        let rng = Pcg64Mcg::from_seed(Default::default());
        for time in rng.random_iter::<Time>().take(1 << 14) {
            assert!(time.is_valid());
        }
    }
}
