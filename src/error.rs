//! A custom error type

use std::{error::Error, fmt, io};


/// A custom error that should be used when a function/method can return multiple error types, 
/// or when a more specific error type does not already exist for a given situation.
#[derive(Debug)]
pub enum ImpulsePhmError {
    /// Errors related to I/O operations
    Io(io::Error),

    /// Errors that could occur when using, or attempting to use, one of the databases.
    Database(rusqlite::Error),

    /// Occurs specifically when a table is missing
    MissingTable(String),

    /// Occurs specifically when a PRAGMA value is not the expected value in one of the databases
    InvalidPragma(String),

    /// Occurs when a database has an invalid schema version
    InvalidSchemaVersion(String),

    /// Occurs when a user database does not meet the standard of being the application file format
    /// 
    /// Because the schema of a user database defines the application file format, a user database 
    /// can be associated with this error variant (such as using an incorrect file extension) 
    /// as well as a [`ImpulsePhmError::Database`] (such as executing an invalid SQL query). 
    AppFileFormat(String),

    /// Occurs when there's an issue parsing values used for semantic versioning
    SemVerParse(semver::Error),

    /// Occurs when a required value is missing
    /// 
    /// This type of error will only occur in cases where the interface doesn't explicitly require 
    /// a value, but the value is actually required (determined by the business logic). For example, 
    /// when using a "builder" interface it's possible for this error to be returned.
    MissingValue(String),

    /// Occurs when a value is the correct data type but still causes a logic error in the code
    InvalidValue(String),
}

impl fmt::Display for ImpulsePhmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "{e}"),
            Self::Database(e) => write!(f, "{e}"),
            Self::MissingTable(message) => write!(f, "{message}"),
            Self::InvalidPragma(message) => write!(f, "{message}"),
            Self::InvalidSchemaVersion(message) => write!(f, "{message}"),
            Self::AppFileFormat(message) => write!(f, "{message}"),
            Self::SemVerParse(e) => write!(f, "{e}"),
            Self::MissingValue(message) => write!(f, "{message}"),
            Self::InvalidValue(message) => write!(f, "{message}"),
        }
    }
}

impl Error for ImpulsePhmError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            Self::Database(e) => Some(e),
            Self::SemVerParse(e) => Some(e),
            // For error variants that don't have a source error (meaning they are the source error)
            _ => None
        }
    }
}

impl From<io::Error> for ImpulsePhmError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<rusqlite::Error> for ImpulsePhmError {
    fn from(error: rusqlite::Error) -> Self {
        Self::Database(error)
    }
}

impl From<semver::Error> for ImpulsePhmError {
    fn from(error: semver::Error) -> Self {
        Self::SemVerParse(error)
    }
}
