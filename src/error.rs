// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Error types for this crate.

mod date_time;

pub use self::date_time::{DateTimeRangeError, DateTimeRangeErrorKind};
