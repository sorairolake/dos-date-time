// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![feature(test)]

extern crate test;

mod cmp;
mod convert;

use dos_date_time::{Date, DateTime, Time, time::macros::date};
use test::Bencher;

#[bench]
fn new(b: &mut Bencher) {
    b.iter(|| DateTime::new(Date::MIN, Time::MIN));
}

#[bench]
fn from_date_time(b: &mut Bencher) {
    b.iter(|| DateTime::from_date_time(date!(1980-01-01), time::Time::MIDNIGHT).unwrap());
}

#[bench]
fn is_valid(b: &mut Bencher) {
    b.iter(|| DateTime::MIN.is_valid());
}

#[bench]
fn date(b: &mut Bencher) {
    b.iter(|| DateTime::MIN.date());
}

#[bench]
fn time(b: &mut Bencher) {
    b.iter(|| DateTime::MIN.time());
}

#[bench]
fn year(b: &mut Bencher) {
    b.iter(|| DateTime::MIN.year());
}

#[bench]
fn month(b: &mut Bencher) {
    b.iter(|| DateTime::MIN.month());
}

#[bench]
fn day(b: &mut Bencher) {
    b.iter(|| DateTime::MIN.day());
}

#[bench]
fn hour(b: &mut Bencher) {
    b.iter(|| DateTime::MIN.hour());
}

#[bench]
fn minute(b: &mut Bencher) {
    b.iter(|| DateTime::MIN.minute());
}

#[bench]
fn second(b: &mut Bencher) {
    b.iter(|| DateTime::MIN.second());
}

#[bench]
fn default(b: &mut Bencher) {
    b.iter(DateTime::default);
}
