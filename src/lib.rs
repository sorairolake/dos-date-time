// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! The `dos-date-time` crate is an [MS-DOS date and time] library.
//!
//! The [`DateTime`] is a type that represents MS-DOS date and time, which are
//! packed 16-bit unsigned integer values that specify the date and time an
//! MS-DOS file was last written to, and are used as timestamps such as [FAT] or
//! [ZIP] file format.
//!
//! # Examples
//!
//! ## Basic usage
//!
//! [`DateTime`] can be converted from and to a type which represents time such
//! as [`time::PrimitiveDateTime`].
//!
//! ```
//! use core::time::Duration;
//!
//! use dos_date_time::{
//!     DateTime,
//!     time::{
//!         PrimitiveDateTime,
//!         macros::{date, datetime, time},
//!     },
//! };
//!
//! let dt = DateTime::MIN;
//! let dt = PrimitiveDateTime::from(dt);
//! assert_eq!(dt, datetime!(1980-01-01 00:00:00));
//!
//! // <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>.
//! let dt = dt + Duration::from_secs(722_805_900);
//! assert_eq!(dt, datetime!(2002-11-26 19:25:00));
//! let dt = DateTime::try_from(dt).unwrap();
//! assert_eq!(
//!     (dt.date(), dt.time()),
//!     (0b0010_1101_0111_1010, 0b1001_1011_0010_0000)
//! );
//!
//! // The largest MS-DOS date and time.
//! assert_eq!(
//!     DateTime::from_date_time(date!(2107-12-31), time!(23:59:58)),
//!     Ok(DateTime::MAX)
//! );
//! ```
//!
//! ## Formatting and printing MS-DOS date and time
//!
//! The [`Display`](core::fmt::Display) trait for [`DateTime`] is implemented to
//! show the value in the well-known [RFC 3339 format]. If you need a different
//! date and time format, convert [`DateTime`] to a type which represents time
//! such as [`time::PrimitiveDateTime`].
//!
//! ```
//! use dos_date_time::{
//!     DateTime,
//!     time::{PrimitiveDateTime, format_description::well_known::Rfc2822},
//! };
//!
//! let dt = DateTime::MIN;
//! assert_eq!(format!("{dt}"), "1980-01-01 00:00:00");
//!
//! let dt = PrimitiveDateTime::from(dt)
//!     .as_utc()
//!     .format(&Rfc2822)
//!     .unwrap();
//! assert_eq!(format!("{dt}"), "Tue, 01 Jan 1980 00:00:00 +0000");
//! ```
//!
//! [MS-DOS date and time]: https://learn.microsoft.com/en-us/windows/win32/sysinfo/ms-dos-date-and-time
//! [FAT]: https://en.wikipedia.org/wiki/File_Allocation_Table
//! [ZIP]: https://en.wikipedia.org/wiki/ZIP_(file_format)
//! [RFC 3339 format]: https://datatracker.ietf.org/doc/html/rfc3339#section-5.6

#![doc(html_root_url = "https://docs.rs/dos-date-time/0.1.0/")]
#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
// Lint levels of rustc.
#![deny(missing_docs)]

#[cfg(test)]
#[macro_use]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

mod date_time;
pub mod error;

#[cfg(feature = "chrono")]
pub use chrono;
#[cfg(feature = "jiff")]
pub use jiff;
pub use time;

pub use crate::date_time::DateTime;
