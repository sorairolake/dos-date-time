// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Error types for this crate.

mod dos_date;
mod dos_date_time;

pub use self::{
    dos_date::{DateRangeError, DateRangeErrorKind},
    dos_date_time::{DateTimeRangeError, DateTimeRangeErrorKind},
};
