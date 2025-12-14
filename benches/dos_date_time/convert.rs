// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[cfg(feature = "chrono")]
use chrono::NaiveDateTime;
use dos_date_time::{
    DateTime,
    time::{PrimitiveDateTime, macros::datetime},
};
#[cfg(feature = "jiff")]
use jiff::civil;
use test::Bencher;

#[bench]
fn try_from_date_time_to_primitive_date_time(b: &mut Bencher) {
    b.iter(|| PrimitiveDateTime::from(DateTime::MIN));
}

#[cfg(feature = "chrono")]
#[bench]
fn from_date_time_to_chrono_naive_date_time(b: &mut Bencher) {
    b.iter(|| NaiveDateTime::from(DateTime::MIN));
}

#[cfg(feature = "jiff")]
#[bench]
fn try_from_date_time_to_jiff_civil_date_time(b: &mut Bencher) {
    b.iter(|| civil::DateTime::from(DateTime::MIN));
}

#[bench]
fn try_from_primitive_date_time_to_date_time(b: &mut Bencher) {
    let dt = datetime!(1980-01-01 00:00:00);
    b.iter(|| DateTime::try_from(dt).unwrap());
}

#[cfg(feature = "chrono")]
#[bench]
fn try_from_chrono_naive_date_time_to_date_time(b: &mut Bencher) {
    let dt = "1980-01-01T00:00:00".parse::<NaiveDateTime>().unwrap();
    b.iter(|| DateTime::try_from(dt).unwrap());
}

#[cfg(feature = "jiff")]
#[bench]
fn try_from_jiff_civil_date_time_to_date_time(b: &mut Bencher) {
    let dt = civil::date(1980, 1, 1).at(0, 0, 0, 0);
    b.iter(|| DateTime::try_from(dt).unwrap());
}
