// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![feature(test)]

extern crate test;

use dos_date_time::{
    DateTime,
    time::{Time, macros::date},
};
use test::Bencher;

#[bench]
fn new(b: &mut Bencher) {
    b.iter(|| DateTime::new(0b0000_0000_0010_0001, u16::MIN).unwrap());
}

#[bench]
fn new_unchecked(b: &mut Bencher) {
    b.iter(|| unsafe { DateTime::new_unchecked(0b0000_0000_0010_0001, u16::MIN) });
}

#[bench]
fn from_date_time(b: &mut Bencher) {
    b.iter(|| DateTime::from_date_time(date!(1980-01-01), Time::MIDNIGHT).unwrap());
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
