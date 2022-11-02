//! This crate implements the error type used in the rhsp3 project.
//!
//! This is publicly reexported in every subcrate that uses it, but this crate itself is not
//! public API.

use crate::{ctx::HspType, hsp_errors::ErrorCode};
use backtrace::Backtrace;
use std::{
    fmt::{Debug, Display, Formatter},
    thread::ThreadId,
};
use thiserror::Error;

pub use std::{error::Error as StdError, result::Result as StdResult};

pub type Result<T> = StdResult<T, Error>;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ErrorKind {
    #[error("Plugin SDK called from wrong thread: {0:?} != {1:?}")]
    ExecutionOnWrongThread(ThreadId, ThreadId),
    #[error("Integer value cannot fit into an {0}: {1}")]
    IntegerNotInRange(&'static str, i64),
    #[error("HSP value has incorrect type. (expected {0:?}, got {1:?})")]
    HspTypeError(HspType, HspType),
    #[error("HSP version is too old for function `{0}`. (required 0x{1:04x}, got 0x{2:04x})")]
    HspVersionError(&'static str, u16, u16),
    #[error("HSP function `{1}` does not exist for type {0:?}.")]
    FunctionNotFoundForType(HspType, &'static str),

    /// An error type that only exists for testing.
    #[error("[Testing error]")]
    TestingError,
    /// A wrapped external error.
    #[error("External error encountered: {0}")]
    WrappedError(#[from] Box<dyn StdError + 'static>),
    /// An internal error occurred.
    #[error("Internal error {0}")]
    StaticInternalError(&'static str),
    /// An internal error occurred.
    #[error("Internal error: {0}")]
    InternalError(String),
    /// A message that should be shown directly to the user.
    #[error("{0}")]
    Message(String),
}

/// The error type used by rhsp3.
///
/// This type does not implement [`std::error::Error`] in order to allow a blanket `From`
/// implementation from any standard error type.
///
/// # Optimizations
///
/// Most functions in this type are marked `#[inline(never)]` to optimize binary size and code
/// cache usage, and this type is itself only one pointer in size to avoid excessively expanding
/// the size of function return values.
#[derive(Debug)]
pub struct Error(Box<ErrorData>);
#[derive(Debug)]
struct ErrorData {
    kind: ErrorKind,
    backtrace: Option<Backtrace>,
    cause: Option<Box<dyn StdError + 'static>>,
    error_code: Option<ErrorCode>,
}
impl Error {
    /// Creates an new error containing a message to show directly to the user.
    #[inline(never)]
    pub fn message(msg: impl Display) -> Self {
        Error::new(ErrorKind::Message(msg.to_string()))
    }

    /// Creates an new error containing an internal error.
    #[inline(never)]
    pub fn internal_error(msg: impl Display) -> Self {
        Error::new(ErrorKind::InternalError(msg.to_string())).with_backtrace()
    }

    #[inline(never)]
    fn new(kind: ErrorKind) -> Error {
        Error(Box::new(ErrorData { kind, backtrace: None, cause: None, error_code: None }))
    }

    #[inline(never)]
    fn new_with_cause(kind: ErrorKind, source: impl StdError + 'static) -> Error {
        Error::new(kind).with_cause(Box::new(source))
    }

    /// Sets the cause of this error to a given source.
    #[inline(never)]
    fn with_cause(mut self, mut source: Box<dyn StdError + 'static>) -> Self {
        // Make sure we don't get into tangles of error chains.
        assert!(self.0.cause.is_none());

        if let Some(err) = source.downcast_mut::<ErrorWrapper>() {
            // If the cause is an `ErrorWrapping` containing a backtrace, we move it to this type.
            if let Some(backtrace) = err.as_inner_mut().0.backtrace.take() {
                self.0.backtrace = Some(backtrace);
            }

            // If the cause contains an HSP error code, we move it to this type.
            if let Some(hsp_code) = err.as_inner().0.error_code {
                if self.0.error_code.is_none() {
                    self.0.error_code = Some(hsp_code);
                }
            }
        }

        // Actually set the cause.
        self.0.cause = Some(source);
        self
    }

    /// Attaches a backtrace if one does not already exist.
    #[inline(never)]
    fn with_backtrace(mut self) -> Self {
        if self.0.backtrace.is_none() {
            self.0.backtrace = Some(Backtrace::new_unresolved());
        }
        self
    }

    #[inline(never)]
    fn context(mut self, kind: ErrorKind) -> Self {
        if let ErrorKind::WrappedError(_) = &self.0.kind {
            // Assumption: `ErrorKind::WrappedError` is never used with `cause.is_some()`
            let kind = std::mem::replace(&mut self.0.kind, kind);
            self.0.cause = Some(match kind {
                ErrorKind::WrappedError(err) => err,
                _ => unreachable!(),
            });
            self
        } else {
            Error::new(kind).with_cause(Box::new(self.into_error()))
        }
    }

    /// Sets the error code returned to HSP.
    pub fn with_error_code(mut self, code: ErrorCode) -> Self {
        self.0.error_code = Some(code);
        self
    }

    /// Returns the source of this error if one exists.
    pub fn source(&self) -> Option<&(dyn StdError + 'static)> {
        if let Some(x) = self.0.cause.as_ref() {
            Some(x.as_ref())
        } else {
            StdError::source(&self.0.kind)
        }
    }

    /// Returns the backtrace stored as part of this error, if one exists.
    pub fn backtrace(&self) -> Option<&Backtrace> {
        self.0.backtrace.as_ref()
    }

    /// Returns the error code this error will return to HSP code.
    pub fn error_code(&self) -> ErrorCode {
        self.0.error_code.unwrap_or(ErrorCode::GenericError)
    }

    /// Converts this error into a type that implements [`std::error::Error`].
    pub fn into_error(self) -> ErrorWrapper {
        ErrorWrapper(self)
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0.kind, f)
    }
}
impl<E: StdError + 'static> From<E> for Error {
    #[inline(never)]
    fn from(value: E) -> Self {
        Error::new(ErrorKind::WrappedError(Box::new(value))).with_backtrace()
    }
}

/// An [`Error`] wrapped so that it may be used as an [`std::error::Error`].
///
/// This type is publicly exposed to allow downcasts to work.
#[derive(Debug)]
pub struct ErrorWrapper(Error);
impl ErrorWrapper {
    /// Returns the [`Error`] wrapped into this type as a reference.
    pub fn as_inner(&self) -> &Error {
        &self.0
    }

    /// Returns the [`Error`] wrapped into this type as a mutable reference.
    pub fn as_inner_mut(&mut self) -> &mut Error {
        &mut self.0
    }

    /// Returns the [`Error`] wrapped into this type.
    pub fn into_inner(self) -> Error {
        self.0
    }
}
impl Display for ErrorWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
impl StdError for ErrorWrapper {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.0.source()
    }
}

/// Contains the implementation of [`Result`] extensions
pub trait ResultPrivate<T> {
    /// Tags an error (if one occurs) with an [`ErrorKind`]
    fn context(self, kind: ErrorKind) -> StdResult<T, Error>;
}
impl<T, E: StdError + 'static> ResultPrivate<T> for StdResult<T, E> {
    fn context(self, kind: ErrorKind) -> StdResult<T, Error> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::new_with_cause(kind, e)),
        }
    }
}
impl<T> ResultPrivate<T> for StdResult<T, Error> {
    fn context(self, kind: ErrorKind) -> StdResult<T, Error> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(e.context(kind)),
        }
    }
}

/// Creates a new error wrapping a given [`ErrorKind`].
#[inline(never)]
pub fn error_new(kind: ErrorKind) -> Error {
    if let ErrorKind::WrappedError(_) = &kind {
        panic!("Cannot use `ErrorKind::WrappedErrors` through `Error::new`!")
    }
    Error::new(kind)
}

/// Creates a new error wrapping a given [`ErrorKind`].
#[inline(never)]
pub fn error_new_str(str: &'static str) -> Error {
    Error::new(ErrorKind::StaticInternalError(str)).with_backtrace()
}

/// Contains the implementation of functions of [`Error`] that are private to the `rhsp3` project.
pub trait ErrorPrivate {
    /// Attaches a backtrace to this error.
    fn with_backtrace(self) -> Self;
}
impl ErrorPrivate for Error {
    #[inline(never)]
    fn with_backtrace(self) -> Self {
        self.with_backtrace()
    }
}

/// Not public API.
#[doc(hidden)]
pub mod macro_hidden {
    use super::*;

    pub use std::result::Result::Err;

    #[inline(never)]
    pub fn message(msg: impl Display) -> Error {
        Error::message(msg)
    }

    #[inline(never)]
    pub fn internal_error(msg: impl Display) -> Error {
        Error::internal_error(msg.to_string())
    }

    #[inline(never)]
    pub fn message_with_code(code: ErrorCode, msg: impl Display) -> Error {
        Error::message(msg).with_error_code(code)
    }

    #[inline(never)]
    pub fn internal_error_lit_with_code(code: ErrorCode, msg: &'static str) -> Error {
        error_new_str(msg).with_error_code(code)
    }
}

/// Returns from an rhsp3 function with an internal error.
///
/// This has the same parameters as [`panic!`], but it returns an [`Result`] from the current
/// function instead.
///
/// In addition, you may pass an additional first parameter like `code: TypeMismatch` containing
/// an variant of [`ErrorCode`] to override the error code returned to HSP code.
#[macro_export]
macro_rules! bail {
    (code: $code:ident, $($rest:tt)*) => {
        return $crate::errors::macro_hidden::Err(
            $crate::errors::macro_hidden::internal_error_with_code(
                $crate::hsp_errors::$code, format_args!($($rest)*),
            ),
        )
    };
    (code: $code:expr, $($rest:tt)*) => {
        return $crate::errors::macro_hidden::Err(
            $crate::errors::macro_hidden::internal_error_with_code(
                $code, format_args!($($rest)*),
            ),
        )
    };
    ($($rest:tt)*) => {
        return $crate::errors::macro_hidden::Err(
            $crate::errors::macro_hidden::internal_error(format_args!($($rest)*)),
        )
    }
}

/// Returns from an rhsp3 function with an internal error if a condition is not met.
///
/// This has the same parameters as [`assert!`], but it returns an [`Result`] from the current
/// function instead.
///
/// In addition, you may pass an additional first parameter like `code: TypeMismatch` containing
/// an variant of [`ErrorCode`] to override the error code returned to HSP code.
#[macro_export]
macro_rules! ensure {
    (code: $code:ident, $test:expr $(,)*) => {
        if !$test {
            $crate::bail_lit!(code: $code, stringify!($test));
        }
    };
    (code: $code:expr, $test:expr $(,)*) => {
        if !$test {
            $crate::bail_lit!(code: $code, stringify!($test));
        }
    };
    ($test:expr $(,)*) => {
        if !$test {
            $crate::bail_lit!(stringify!($test));
        }
    };
    (code: $code:ident, $test:expr, $($rest:tt)*) => {
        if !$test {
            $crate::bail!(code: $code, $($rest)*);
        }
    };
    (code: $code:expr, $test:expr, $($rest:tt)*) => {
        if !$test {
            $crate::bail!(code: $code, $($rest)*);
        }
    };
    ($test:expr, $($rest:tt)*) => {
        if !$test {
            $crate::bail!($($rest)*);
        }
    };
}

/// Returns from an rhsp3 function with an internal error if a condition is not met.
///
/// This has the same parameters as [`bail!`], but it does not parse format args, and instead
/// simply uses a single literal string.
#[macro_export]
macro_rules! bail_lit {
    (code: $code:ident, $lit:expr $(,)?) => {
        return $crate::errors::macro_hidden::Err(
            $crate::errors::macro_hidden::internal_error_lit_with_code(
                $crate::hsp_errors::$code,
                $lit,
            ),
        )
    };
    (code: $code:expr, $lit:expr $(,)?) => {
        return $crate::errors::macro_hidden::Err(
            $crate::errors::macro_hidden::internal_error_lit_with_code($code, $lit),
        )
    };
    ($lit:expr $(,)?) => {
        return $crate::errors::macro_hidden::Err($crate::errors::error_new_str($lit))
    };
}

/// Returns from an rhsp3 function with an internal error if a condition is not met.
///
/// This has the same parameters as [`ensure!`], but it does not parse format args, and instead
/// simply uses a single literal string.
#[macro_export]
macro_rules! ensure_lit {
    (code: $code:ident, $test:expr $(,)*) => {
        if !$test {
            $crate::bail_lit!(code: $code, stringify!($test));
        }
    };
    (code: $code:expr, $test:expr $(,)*) => {
        if !$test {
            $crate::bail_lit!(code: $code, stringify!($test));
        }
    };
    ($test:expr $(,)*) => {
        if !$test {
            $crate::bail_lit!(stringify!($test));
        }
    };
    (code: $code:ident, $test:expr, $lit:expr $(,)?) => {
        if !$test {
            $crate::bail_lit!(code: $code, $lit);
        }
    };
    (code: $code:expr, $test:expr, $lit:expr $(,)?) => {
        if !$test {
            $crate::bail_lit!(code: $code, $lit);
        }
    };
    ($test:expr, $lit:expr $(,)?) => {
        if !$test {
            $crate::bail_lit!($lit);
        }
    };
}
