// SPDX-FileCopyrightText: 2026 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use dos_date_time::{Date, rand::RngExt};
use rand_pcg::{Pcg64Mcg, rand_core::SeedableRng};
use test::Bencher;

#[bench]
fn sample(b: &mut Bencher) {
    let mut rng = Pcg64Mcg::from_seed(Default::default());
    b.iter(|| rng.random::<Date>());
}
