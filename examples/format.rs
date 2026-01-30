// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! An example of printing MS-DOS date and time in a human-readable format.

use anyhow::Context;
use clap::Parser;
use dos_date_time::{Date, DateTime, Time};

#[derive(Debug, Parser)]
#[command(version, about)]
struct Opt {
    /// MS-DOS date to print.
    date: u16,

    /// MS-DOS time to print.
    time: u16,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::parse();

    let (date, time) = (
        Date::new(opt.date).context("could not convert MS-DOS date")?,
        Time::new(opt.time).context("could not convert MS-DOS time")?,
    );
    let dt = DateTime::new(date, time);
    println!("{dt}");
    Ok(())
}
