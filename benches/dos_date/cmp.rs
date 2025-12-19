// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use dos_date_time::Date;
use test::Bencher;

#[bench]
fn equality(b: &mut Bencher) {
    b.iter(|| Date::MIN == Date::MIN);
}

#[bench]
fn order(b: &mut Bencher) {
    b.iter(|| Date::MAX > Date::MIN);
}
