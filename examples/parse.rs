// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! An example of printing a human-readable date and time as MS-DOS date and
//! time.

use std::{ops::Deref, str::FromStr};

use anyhow::Context;
use clap::Parser;
use dos_date_time::time::{
    PrimitiveDateTime,
    error::Parse,
    format_description::well_known::{Iso8601, Rfc2822, Rfc3339},
};

#[derive(Debug, Parser)]
#[command(version, about)]
struct Opt {
    /// Date and time to print.
    ///
    /// <DATE> is a string representing a date and time in either ISO 8601, RFC
    /// 2822, or RFC 3339 format.
    date: DateTime,
}

#[derive(Clone, Debug)]
struct DateTime(PrimitiveDateTime);

impl Deref for DateTime {
    type Target = PrimitiveDateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for DateTime {
    type Err = Parse;

    fn from_str(dt: &str) -> Result<Self, Self::Err> {
        PrimitiveDateTime::parse(dt, &Iso8601::DEFAULT)
            .or_else(|_| PrimitiveDateTime::parse(dt, &Rfc2822))
            .or_else(|_| PrimitiveDateTime::parse(dt, &Rfc3339))
            .map(Self)
    }
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::parse();

    let dt =
        dos_date_time::DateTime::try_from(*opt.date).context("could not convert date and time")?;
    let dt = (dt.date().to_raw(), dt.time().to_raw());
    println!("{dt:?}");
    Ok(())
}
