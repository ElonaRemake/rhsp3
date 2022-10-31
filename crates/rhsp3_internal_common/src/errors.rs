//! This crate implements the error type used in the rhsp3 project.
//!
//! This is publicly reexported in every subcrate that uses it, but this crate itself is not
//! public API.

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

    /// An error type that only exists for testing.
    #[error("[Testing error]")]
    TestingError,
    /// A wrapped external error.
    #[error("External error encountered: {0}")]
    WrappedError(#[from] Box<dyn StdError + 'static>),
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
}
impl Error {
    #[inline(never)]
    fn new(kind: ErrorKind) -> Error {
        Error(Box::new(ErrorData { kind, backtrace: None, cause: None }))
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

        // If the cause is an `ErrorWrapping` containing a backtrace, we move it to this type.
        if let Some(err) = source.downcast_mut::<ErrorWrapper>() {
            if let Some(backtrace) = err.as_inner_mut().0.backtrace.take() {
                self.0.backtrace = Some(backtrace);
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
