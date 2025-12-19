// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use dos_date_time::DateTime;
use test::Bencher;

#[bench]
fn equality(b: &mut Bencher) {
    b.iter(|| DateTime::MIN == DateTime::MIN);
}

#[bench]
fn order(b: &mut Bencher) {
    b.iter(|| DateTime::MAX > DateTime::MIN);
}
