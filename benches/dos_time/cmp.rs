// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use dos_date_time::Time;
use test::Bencher;

#[bench]
fn equality(b: &mut Bencher) {
    b.iter(|| Time::MIN == Time::MIN);
}

#[bench]
fn order(b: &mut Bencher) {
    b.iter(|| Time::MAX > Time::MIN);
}
