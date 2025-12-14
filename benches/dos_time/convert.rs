// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[cfg(feature = "chrono")]
use chrono::NaiveTime;
use dos_date_time::Time;
#[cfg(feature = "jiff")]
use jiff::civil;
use test::Bencher;

#[bench]
fn from_time_to_time_time(b: &mut Bencher) {
    b.iter(|| time::Time::from(Time::MIN));
}

#[cfg(feature = "chrono")]
#[bench]
fn from_time_to_chrono_naive_time(b: &mut Bencher) {
    b.iter(|| NaiveTime::from(Time::MIN));
}

#[cfg(feature = "jiff")]
#[bench]
fn from_time_to_jiff_civil_time(b: &mut Bencher) {
    b.iter(|| civil::Time::from(Time::MIN));
}

#[bench]
fn from_time_time_to_time(b: &mut Bencher) {
    b.iter(|| Time::from(time::Time::MIDNIGHT));
}

#[cfg(feature = "chrono")]
#[bench]
fn from_chrono_naive_time_to_time(b: &mut Bencher) {
    b.iter(|| Time::from(NaiveTime::MIN));
}

#[cfg(feature = "jiff")]
#[bench]
fn from_jiff_civil_time_to_time(b: &mut Bencher) {
    b.iter(|| Time::from(civil::Time::MIN));
}
