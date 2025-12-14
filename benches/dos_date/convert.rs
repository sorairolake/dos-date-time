// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[cfg(feature = "chrono")]
use chrono::NaiveDate;
use dos_date_time::{Date, time::macros::date};
#[cfg(feature = "jiff")]
use jiff::civil;
use test::Bencher;

#[bench]
fn from_date_to_time_date(b: &mut Bencher) {
    b.iter(|| time::Date::from(Date::MIN));
}

#[cfg(feature = "chrono")]
#[bench]
fn from_date_to_chrono_naive_date(b: &mut Bencher) {
    b.iter(|| NaiveDate::from(Date::MIN));
}

#[cfg(feature = "jiff")]
#[bench]
fn from_date_to_jiff_civil_date(b: &mut Bencher) {
    b.iter(|| civil::Date::from(Date::MIN));
}

#[bench]
fn try_from_time_date_to_date(b: &mut Bencher) {
    let date = date!(1980-01-01);
    b.iter(|| Date::try_from(date).unwrap());
}

#[cfg(feature = "chrono")]
#[bench]
fn try_from_chrono_naive_date_to_date(b: &mut Bencher) {
    let date = "1980-01-01".parse::<NaiveDate>().unwrap();
    b.iter(|| Date::try_from(date).unwrap());
}

#[cfg(feature = "jiff")]
#[bench]
fn try_from_jiff_civil_date_to_date(b: &mut Bencher) {
    let date = civil::date(1980, 1, 1);
    b.iter(|| Date::try_from(date).unwrap());
}
