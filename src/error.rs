//! A custom error type

use std::{error::Error, fmt, io};


/// A custom error that should be used when a function/method can return multiple error types, 
/// or when a more specific error type does not already exist for a given situation.
#[derive(Debug)]
pub enum ImpulsePhmError {
    /// Errors related to I/O operations.
    IoError(io::Error),

    /// Errors that could occur when using, or attempting to use, one of the databases.
    DatabaseError(rusqlite::Error),

    /// Errors when a user database does not meet the standard of being the application file format.
    /// 
    /// Because the schema of a user database defines the application file format, a user database 
    /// can be associated with this error variant (such as using an incorrect file extension) 
    /// as well as a [`ImpulsePhmError::DatabaseError`] (such as executing an invalid SQL query). 
    AppFileFormatError(String),

    /// Errors that could occur when parsing values used for semantic versioning
    SemVerParseError(semver::Error)
}

impl fmt::Display for ImpulsePhmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImpulsePhmError::IoError(e) => write!(f, "{e}"),
            ImpulsePhmError::DatabaseError(e) => write!(f, "{e}"),
            ImpulsePhmError::AppFileFormatError(message) => write!(f, "{message}"),
            ImpulsePhmError::SemVerParseError(e) => write!(f, "{e}")
        }
    }
}

impl Error for ImpulsePhmError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ImpulsePhmError::IoError(e) => Some(e),
            ImpulsePhmError::DatabaseError(e) => Some(e),
            ImpulsePhmError::SemVerParseError(e) => Some(e),
            // For error variants that don't have a source error (meaning they are the source error)
            _ => None
        }
    }
}

impl From<io::Error> for ImpulsePhmError {
    fn from(error: io::Error) -> Self {
        ImpulsePhmError::IoError(error)
    }
}

impl From<rusqlite::Error> for ImpulsePhmError {
    fn from(error: rusqlite::Error) -> Self {
        ImpulsePhmError::DatabaseError(error)
    }
}

impl From<semver::Error> for ImpulsePhmError {
    fn from(error: semver::Error) -> Self {
        ImpulsePhmError::SemVerParseError(error)
    }
}
