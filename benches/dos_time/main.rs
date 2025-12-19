// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![feature(test)]

extern crate test;

mod cmp;
mod convert;

use dos_date_time::Time;
use test::Bencher;

#[bench]
fn new(b: &mut Bencher) {
    b.iter(|| Time::new(u16::MIN).unwrap());
}

#[bench]
fn from_time(b: &mut Bencher) {
    b.iter(|| Time::from_time(time::Time::MIDNIGHT));
}

#[bench]
fn is_valid(b: &mut Bencher) {
    b.iter(|| Time::MIN.is_valid());
}

#[bench]
fn to_raw(b: &mut Bencher) {
    b.iter(|| Time::MIN.to_raw());
}

#[bench]
fn hour(b: &mut Bencher) {
    b.iter(|| Time::MIN.hour());
}

#[bench]
fn minute(b: &mut Bencher) {
    b.iter(|| Time::MIN.minute());
}

#[bench]
fn second(b: &mut Bencher) {
    b.iter(|| Time::MIN.second());
}

#[bench]
fn default(b: &mut Bencher) {
    b.iter(Time::default);
}
