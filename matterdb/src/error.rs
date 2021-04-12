//! An implementation of `Error` type.

use thiserror::Error;

/// The error type for I/O operations with the `Database`.
///
/// Application code in most cases should consider these errors as fatal. At the same time,
/// it may be possible to recover from an error after manual intervention (e.g., by restarting
/// the process or freeing up more disc space).
#[derive(Debug, Clone, Error)]
#[error("{}", message)]
pub struct Error {
    message: String,
}

impl Error {
    /// Creates a new storage error with an information message about the reason.
    pub fn new<T: Into<String>>(message: T) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl From<rocksdb::Error> for Error {
    fn from(err: rocksdb::Error) -> Self {
        Self::new(err.to_string())
    }
}
