// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![feature(test)]

extern crate test;

mod cmp;
mod convert;

use dos_date_time::{Date, time::macros::date};
use test::Bencher;

#[bench]
fn new(b: &mut Bencher) {
    b.iter(|| Date::new(0b0000_0000_0010_0001).unwrap());
}

#[bench]
fn new_unchecked(b: &mut Bencher) {
    b.iter(|| unsafe { Date::new_unchecked(0b0000_0000_0010_0001) });
}

#[bench]
fn from_date(b: &mut Bencher) {
    b.iter(|| Date::from_date(date!(1980-01-01)).unwrap());
}

#[bench]
fn is_valid(b: &mut Bencher) {
    b.iter(|| Date::MIN.is_valid());
}

#[bench]
fn to_raw(b: &mut Bencher) {
    b.iter(|| Date::MIN.to_raw());
}

#[bench]
fn year(b: &mut Bencher) {
    b.iter(|| Date::MIN.year());
}

#[bench]
fn month(b: &mut Bencher) {
    b.iter(|| Date::MIN.month());
}

#[bench]
fn day(b: &mut Bencher) {
    b.iter(|| Date::MIN.day());
}

#[bench]
fn default(b: &mut Bencher) {
    b.iter(Date::default);
}
