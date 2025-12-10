// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! An example of printing MS-DOS date and time in a human-readable format.

use anyhow::Context;
use clap::Parser;
use dos_date_time::DateTime;

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

    let dt = DateTime::new(opt.date, opt.time).context("could not convert date and time")?;
    println!("{dt}");
    Ok(())
}
