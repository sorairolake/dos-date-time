<!--
SPDX-FileCopyrightText: 2025 Shun Sakai

SPDX-License-Identifier: CC-BY-4.0
-->

# dos-date-time

[![CI][ci-badge]][ci-url]
[![Version][version-badge]][version-url]
![MSRV][msrv-badge]
[![Docs][docs-badge]][docs-url]
![License][license-badge]

**dos-date-time** is an [MS-DOS date and time] library for [Rust].

MS-DOS date and time are packed 16-bit unsigned integer values that specify the
date and time an MS-DOS file was last written to, and are used as timestamps
such as [FAT] or [ZIP] file format.

## Usage

Run the following command in your project directory:

```sh
cargo add dos-date-time
```

### Crate features

#### `chrono`

Enables the [`chrono`] crate.

#### `jiff`

Enables the [`jiff`] crate.

#### `std`

Enables features that depend on the standard library. This is enabled by
default.

### `no_std` support

This supports `no_std` mode. Disables the `default` feature to enable this.

### Documentation

See the [documentation][docs-url] for more details.

## Minimum supported Rust version

The minimum supported Rust version (MSRV) of this library is v1.85.0.

## Source code

The upstream repository is available at
<https://github.com/sorairolake/dos-date-time.git>.

## Changelog

Please see [CHANGELOG.adoc].

## Contributing

Please see [CONTRIBUTING.adoc].

## License

Copyright (C) 2025 Shun Sakai (see [AUTHORS.adoc])

This library is distributed under the terms of either the _Apache License 2.0_
or the _MIT License_.

This project is compliant with version 3.3 of the [_REUSE Specification_]. See
copyright notices of individual files for more details on copyright and
licensing information.

[ci-badge]: https://img.shields.io/github/actions/workflow/status/sorairolake/dos-date-time/CI.yaml?branch=develop&style=for-the-badge&logo=github&label=CI
[ci-url]: https://github.com/sorairolake/dos-date-time/actions?query=branch%3Adevelop+workflow%3ACI++
[version-badge]: https://img.shields.io/crates/v/dos-date-time?style=for-the-badge&logo=rust
[version-url]: https://crates.io/crates/dos-date-time
[msrv-badge]: https://img.shields.io/crates/msrv/dos-date-time?style=for-the-badge&logo=rust
[docs-badge]: https://img.shields.io/docsrs/dos-date-time?style=for-the-badge&logo=docsdotrs&label=Docs.rs
[docs-url]: https://docs.rs/dos-date-time
[license-badge]: https://img.shields.io/crates/l/dos-date-time?style=for-the-badge
[MS-DOS date and time]: https://learn.microsoft.com/en-us/windows/win32/sysinfo/ms-dos-date-and-time
[Rust]: https://www.rust-lang.org/
[FAT]: https://en.wikipedia.org/wiki/File_Allocation_Table
[ZIP]: https://en.wikipedia.org/wiki/ZIP_(file_format)
[`time`]: https://crates.io/crates/time
[`chrono`]: https://crates.io/crates/chrono
[`jiff`]: https://crates.io/crates/jiff
[CHANGELOG.adoc]: CHANGELOG.adoc
[CONTRIBUTING.adoc]: CONTRIBUTING.adoc
[AUTHORS.adoc]: AUTHORS.adoc
[_REUSE Specification_]: https://reuse.software/spec-3.3/
