// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Error types for [`Date`](crate::Date).

use core::{error::Error, fmt};

/// The error type indicating that a [`Date`](crate::Date) was out of range.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DateRangeError(DateRangeErrorKind);

impl DateRangeError {
    #[inline]
    pub(crate) const fn new(kind: DateRangeErrorKind) -> Self {
        Self(kind)
    }

    /// Returns the corresponding [`DateRangeErrorKind`] for this error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dos_date_time::{Date, error::DateRangeErrorKind, time::macros::date};
    /// #
    /// let err = Date::from_date(date!(1979-12-31)).unwrap_err();
    /// assert_eq!(err.kind(), DateRangeErrorKind::Negative);
    ///
    /// let err = Date::from_date(date!(2108-01-01)).unwrap_err();
    /// assert_eq!(err.kind(), DateRangeErrorKind::Overflow);
    /// ```
    #[must_use]
    #[inline]
    pub const fn kind(&self) -> DateRangeErrorKind {
        self.0
    }
}

impl fmt::Display for DateRangeError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind().fmt(f)
    }
}

impl Error for DateRangeError {}

impl From<DateRangeErrorKind> for DateRangeError {
    #[inline]
    fn from(kind: DateRangeErrorKind) -> Self {
        Self::new(kind)
    }
}

/// Details of the error that caused a [`DateRangeError`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DateRangeErrorKind {
    /// Value was negative.
    ///
    /// This means the MS-DOS date was before "1980-01-01".
    Negative,

    /// Value was too big to be represented as [`Date`](crate::Date).
    ///
    /// This means the MS-DOS date was after "2107-12-31".
    Overflow,
}

impl fmt::Display for DateRangeErrorKind {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Negative => write!(f, "MS-DOS date is before `1980-01-01`"),
            Self::Overflow => write!(f, "MS-DOS date is after `2107-12-31`"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clone_date_range_error() {
        assert_eq!(
            DateRangeError::new(DateRangeErrorKind::Negative).clone(),
            DateRangeError::new(DateRangeErrorKind::Negative)
        );
        assert_eq!(
            DateRangeError::new(DateRangeErrorKind::Overflow).clone(),
            DateRangeError::new(DateRangeErrorKind::Overflow)
        );
    }

    #[test]
    fn copy_date_range_error() {
        {
            let a = DateRangeError::new(DateRangeErrorKind::Negative);
            let b = a;
            assert_eq!(a, b);
        }

        {
            let a = DateRangeError::new(DateRangeErrorKind::Overflow);
            let b = a;
            assert_eq!(a, b);
        }
    }

    #[test]
    fn debug_date_range_error() {
        assert_eq!(
            format!("{:?}", DateRangeError::new(DateRangeErrorKind::Negative)),
            "DateRangeError(Negative)"
        );
        assert_eq!(
            format!("{:?}", DateRangeError::new(DateRangeErrorKind::Overflow)),
            "DateRangeError(Overflow)"
        );
    }

    #[test]
    fn date_range_error_equality() {
        assert_eq!(
            DateRangeError::new(DateRangeErrorKind::Negative),
            DateRangeError::new(DateRangeErrorKind::Negative)
        );
        assert_ne!(
            DateRangeError::new(DateRangeErrorKind::Negative),
            DateRangeError::new(DateRangeErrorKind::Overflow)
        );
        assert_ne!(
            DateRangeError::new(DateRangeErrorKind::Overflow),
            DateRangeError::new(DateRangeErrorKind::Negative)
        );
        assert_eq!(
            DateRangeError::new(DateRangeErrorKind::Overflow),
            DateRangeError::new(DateRangeErrorKind::Overflow)
        );
    }

    #[test]
    fn kind_date_range_error() {
        assert_eq!(
            DateRangeError::new(DateRangeErrorKind::Negative).kind(),
            DateRangeErrorKind::Negative
        );
        assert_eq!(
            DateRangeError::new(DateRangeErrorKind::Overflow).kind(),
            DateRangeErrorKind::Overflow
        );
    }

    #[test]
    const fn kind_date_range_error_is_const_fn() {
        const _: DateRangeErrorKind = DateRangeError::new(DateRangeErrorKind::Negative).kind();
    }

    #[test]
    fn display_date_range_error() {
        assert_eq!(
            format!("{}", DateRangeError::new(DateRangeErrorKind::Negative)),
            "MS-DOS date is before `1980-01-01`"
        );
        assert_eq!(
            format!("{}", DateRangeError::new(DateRangeErrorKind::Overflow)),
            "MS-DOS date is after `2107-12-31`"
        );
    }

    #[test]
    fn source_date_range_error() {
        assert!(
            DateRangeError::new(DateRangeErrorKind::Negative)
                .source()
                .is_none()
        );
        assert!(
            DateRangeError::new(DateRangeErrorKind::Overflow)
                .source()
                .is_none()
        );
    }

    #[test]
    fn from_date_range_error_kind_to_date_range_error() {
        assert_eq!(
            DateRangeError::from(DateRangeErrorKind::Negative),
            DateRangeError::new(DateRangeErrorKind::Negative)
        );
        assert_eq!(
            DateRangeError::from(DateRangeErrorKind::Overflow),
            DateRangeError::new(DateRangeErrorKind::Overflow)
        );
    }

    #[test]
    fn clone_date_range_error_kind() {
        assert_eq!(
            DateRangeErrorKind::Negative.clone(),
            DateRangeErrorKind::Negative
        );
        assert_eq!(
            DateRangeErrorKind::Overflow.clone(),
            DateRangeErrorKind::Overflow
        );
    }

    #[test]
    fn copy_date_range_error_kind() {
        {
            let a = DateRangeErrorKind::Negative;
            let b = a;
            assert_eq!(a, b);
        }

        {
            let a = DateRangeErrorKind::Overflow;
            let b = a;
            assert_eq!(a, b);
        }
    }

    #[test]
    fn debug_date_range_error_kind() {
        assert_eq!(format!("{:?}", DateRangeErrorKind::Negative), "Negative");
        assert_eq!(format!("{:?}", DateRangeErrorKind::Overflow), "Overflow");
    }

    #[test]
    fn date_range_error_kind_equality() {
        assert_eq!(DateRangeErrorKind::Negative, DateRangeErrorKind::Negative);
        assert_ne!(DateRangeErrorKind::Negative, DateRangeErrorKind::Overflow);
        assert_ne!(DateRangeErrorKind::Overflow, DateRangeErrorKind::Negative);
        assert_eq!(DateRangeErrorKind::Overflow, DateRangeErrorKind::Overflow);
    }

    #[test]
    fn display_date_range_error_kind() {
        assert_eq!(
            format!("{}", DateRangeErrorKind::Negative),
            "MS-DOS date is before `1980-01-01`"
        );
        assert_eq!(
            format!("{}", DateRangeErrorKind::Overflow),
            "MS-DOS date is after `2107-12-31`"
        );
    }
}
