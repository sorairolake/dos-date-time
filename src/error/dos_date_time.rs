// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Error types for [`DateTime`](crate::DateTime).

use core::{error::Error, fmt};

use crate::error::{DateRangeError, DateRangeErrorKind};

/// The error type indicating that a [`DateTime`](crate::DateTime) was out of
/// range.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DateTimeRangeError(DateTimeRangeErrorKind);

impl DateTimeRangeError {
    pub(crate) const fn new(kind: DateTimeRangeErrorKind) -> Self {
        Self(kind)
    }

    /// Returns the corresponding [`DateTimeRangeErrorKind`] for this error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::{
    /// #     DateTime,
    /// #     error::DateTimeRangeErrorKind,
    /// #     time::{
    /// #         Time,
    /// #         macros::{date, time},
    /// #     },
    /// # };
    /// #
    /// let err = DateTime::from_date_time(date!(1979-12-31), time!(23:59:59)).unwrap_err();
    /// assert_eq!(err.kind(), DateTimeRangeErrorKind::Negative);
    ///
    /// let err = DateTime::from_date_time(date!(2108-01-01), Time::MIDNIGHT).unwrap_err();
    /// assert_eq!(err.kind(), DateTimeRangeErrorKind::Overflow);
    /// ```
    #[must_use]
    pub const fn kind(&self) -> DateTimeRangeErrorKind {
        self.0
    }
}

impl fmt::Display for DateTimeRangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind().fmt(f)
    }
}

impl Error for DateTimeRangeError {}

impl From<DateTimeRangeErrorKind> for DateTimeRangeError {
    fn from(kind: DateTimeRangeErrorKind) -> Self {
        Self::new(kind)
    }
}

impl From<DateRangeError> for DateTimeRangeError {
    fn from(err: DateRangeError) -> Self {
        match err.kind() {
            DateRangeErrorKind::Negative => Self::new(DateTimeRangeErrorKind::Negative),
            DateRangeErrorKind::Overflow => Self::new(DateTimeRangeErrorKind::Overflow),
        }
    }
}

/// Details of the error that caused a [`DateTimeRangeError`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DateTimeRangeErrorKind {
    /// Value was negative.
    ///
    /// This means MS-DOS date and time were before "1980-01-01 00:00:00".
    Negative,

    /// Value was too big to be represented as [`DateTime`](crate::DateTime).
    ///
    /// This means MS-DOS date and time were after "2107-12-31 23:59:58".
    Overflow,
}

impl fmt::Display for DateTimeRangeErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Negative => write!(f, "MS-DOS date and time are before `1980-01-01 00:00:00`"),
            Self::Overflow => write!(f, "MS-DOS date and time are after `2107-12-31 23:59:58`"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clone_date_time_range_error() {
        assert_eq!(
            DateTimeRangeError::new(DateTimeRangeErrorKind::Negative).clone(),
            DateTimeRangeError::new(DateTimeRangeErrorKind::Negative)
        );
        assert_eq!(
            DateTimeRangeError::new(DateTimeRangeErrorKind::Overflow).clone(),
            DateTimeRangeError::new(DateTimeRangeErrorKind::Overflow)
        );
    }

    #[test]
    fn copy_date_time_range_error() {
        {
            let a = DateTimeRangeError::new(DateTimeRangeErrorKind::Negative);
            let b = a;
            assert_eq!(a, b);
        }

        {
            let a = DateTimeRangeError::new(DateTimeRangeErrorKind::Overflow);
            let b = a;
            assert_eq!(a, b);
        }
    }

    #[test]
    fn debug_date_time_range_error() {
        assert_eq!(
            format!(
                "{:?}",
                DateTimeRangeError::new(DateTimeRangeErrorKind::Negative)
            ),
            "DateTimeRangeError(Negative)"
        );
        assert_eq!(
            format!(
                "{:?}",
                DateTimeRangeError::new(DateTimeRangeErrorKind::Overflow)
            ),
            "DateTimeRangeError(Overflow)"
        );
    }

    #[test]
    fn date_time_range_error_equality() {
        assert_eq!(
            DateTimeRangeError::new(DateTimeRangeErrorKind::Negative),
            DateTimeRangeError::new(DateTimeRangeErrorKind::Negative)
        );
        assert_ne!(
            DateTimeRangeError::new(DateTimeRangeErrorKind::Negative),
            DateTimeRangeError::new(DateTimeRangeErrorKind::Overflow)
        );
        assert_ne!(
            DateTimeRangeError::new(DateTimeRangeErrorKind::Overflow),
            DateTimeRangeError::new(DateTimeRangeErrorKind::Negative)
        );
        assert_eq!(
            DateTimeRangeError::new(DateTimeRangeErrorKind::Overflow),
            DateTimeRangeError::new(DateTimeRangeErrorKind::Overflow)
        );
    }

    #[test]
    fn kind_date_time_range_error() {
        assert_eq!(
            DateTimeRangeError::new(DateTimeRangeErrorKind::Negative).kind(),
            DateTimeRangeErrorKind::Negative
        );
        assert_eq!(
            DateTimeRangeError::new(DateTimeRangeErrorKind::Overflow).kind(),
            DateTimeRangeErrorKind::Overflow
        );
    }

    #[test]
    const fn kind_date_time_range_error_is_const_fn() {
        const _: DateTimeRangeErrorKind =
            DateTimeRangeError::new(DateTimeRangeErrorKind::Negative).kind();
    }

    #[test]
    fn display_date_time_range_error() {
        assert_eq!(
            format!(
                "{}",
                DateTimeRangeError::new(DateTimeRangeErrorKind::Negative)
            ),
            "MS-DOS date and time are before `1980-01-01 00:00:00`"
        );
        assert_eq!(
            format!(
                "{}",
                DateTimeRangeError::new(DateTimeRangeErrorKind::Overflow)
            ),
            "MS-DOS date and time are after `2107-12-31 23:59:58`"
        );
    }

    #[test]
    fn source_date_time_range_error() {
        assert!(
            DateTimeRangeError::new(DateTimeRangeErrorKind::Negative)
                .source()
                .is_none()
        );
        assert!(
            DateTimeRangeError::new(DateTimeRangeErrorKind::Overflow)
                .source()
                .is_none()
        );
    }

    #[test]
    fn from_date_time_range_error_kind_to_date_time_range_error() {
        assert_eq!(
            DateTimeRangeError::from(DateTimeRangeErrorKind::Negative),
            DateTimeRangeError::new(DateTimeRangeErrorKind::Negative)
        );
        assert_eq!(
            DateTimeRangeError::from(DateTimeRangeErrorKind::Overflow),
            DateTimeRangeError::new(DateTimeRangeErrorKind::Overflow)
        );
    }

    #[test]
    fn clone_date_time_range_error_kind() {
        assert_eq!(
            DateTimeRangeErrorKind::Negative.clone(),
            DateTimeRangeErrorKind::Negative
        );
        assert_eq!(
            DateTimeRangeErrorKind::Overflow.clone(),
            DateTimeRangeErrorKind::Overflow
        );
    }

    #[test]
    fn copy_date_time_range_error_kind() {
        {
            let a = DateTimeRangeErrorKind::Negative;
            let b = a;
            assert_eq!(a, b);
        }

        {
            let a = DateTimeRangeErrorKind::Overflow;
            let b = a;
            assert_eq!(a, b);
        }
    }

    #[test]
    fn debug_date_time_range_error_kind() {
        assert_eq!(
            format!("{:?}", DateTimeRangeErrorKind::Negative),
            "Negative"
        );
        assert_eq!(
            format!("{:?}", DateTimeRangeErrorKind::Overflow),
            "Overflow"
        );
    }

    #[test]
    fn date_time_range_error_kind_equality() {
        assert_eq!(
            DateTimeRangeErrorKind::Negative,
            DateTimeRangeErrorKind::Negative
        );
        assert_ne!(
            DateTimeRangeErrorKind::Negative,
            DateTimeRangeErrorKind::Overflow
        );
        assert_ne!(
            DateTimeRangeErrorKind::Overflow,
            DateTimeRangeErrorKind::Negative
        );
        assert_eq!(
            DateTimeRangeErrorKind::Overflow,
            DateTimeRangeErrorKind::Overflow
        );
    }

    #[test]
    fn display_date_time_range_error_kind() {
        assert_eq!(
            format!("{}", DateTimeRangeErrorKind::Negative),
            "MS-DOS date and time are before `1980-01-01 00:00:00`"
        );
        assert_eq!(
            format!("{}", DateTimeRangeErrorKind::Overflow),
            "MS-DOS date and time are after `2107-12-31 23:59:58`"
        );
    }
}
