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
//! <div class="warning">
//!
//! The resolution of MS-DOS date and time is 2 seconds.
//!
//! </div>
//!
//! [MS-DOS date and time]: https://learn.microsoft.com/en-us/windows/win32/sysinfo/ms-dos-date-and-time
//! [FAT]: https://en.wikipedia.org/wiki/File_Allocation_Table
//! [ZIP]: https://en.wikipedia.org/wiki/ZIP_(file_format)

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

#[must_use]
pub const fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
