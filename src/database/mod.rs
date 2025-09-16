//! Everything needed to manage the databases

pub mod core;
pub mod user;

use std::{fs, path::Path};

use rusqlite::Connection;

use crate::error::ImpulsePhmError;


/// Allows for the creation and querying of a database
pub trait Query {
    /// Create a connection to the database
    /// 
    /// # Parameters:
    /// `path`: The database's file path
    /// 
    /// Call [`Query::connect_default`] here if only needing a standard connection with no extra 
    /// logic. If used, then the documentation for it applies here as well (including which errors 
    /// can be returned).
    fn connect(path: &Path) -> Result<Connection, ImpulsePhmError>;

    /// A default implementation for creating a connection to the database
    /// 
    /// This should be called from [`Query::connect`] if no extra logic is needed. Additionally, 
    /// the database file will be created automatically if it doesn't already exist.
    /// 
    /// # Parameters:
    /// `path`: The database's file path
    /// 
    /// # Returns:
    /// A connection to the database
    /// 
    /// # Errors:
    /// 1. Returns an [`ImpulsePhmError::DatabaseError`] should the connection fail to be created
    /// 2. Returns an [`ImpulsePhmError::AppFileFormatError`] if an attempt is made to connect to 
    /// a file that does not meet the rules of the application file format (only applies when trying 
    /// to connect to [`crate::database::user::UserDatabase`]).
    fn connect_default(path: &Path) -> Result<Connection, ImpulsePhmError> {
        let connection = Connection::open(path)?;
        Ok(connection)
    }

    /// Get the path to the database file
    fn get_database_path(&self) -> &Path;

    /// Get the current connection
    fn get_connection(&self) -> &Connection;

    /// Configure the connection (this should be executed right after creating a connection)
    fn configure_connection(&self) -> Result<(), rusqlite::Error>;

    /// A default implementation to enable foreign key enforcement for actual data integrity
    fn enable_foreign_key_enforcement(&self) -> Result<(), rusqlite::Error> {
        self.get_connection().pragma_update(None, "foreign_keys", 1)?;
        Ok(())
    }

    /// A default implementation to enable secure delete
    fn enable_secure_delete(&self) -> Result<(), rusqlite::Error> {
        self.get_connection().pragma_update(None, "secure_delete", 1)?;
        Ok(())
    }

    /// Create the schema of the database based on a file that contains the SQL commands
    /// 
    /// Concrete types are expected to provide the file path and pass it to 
    /// [`Query::execute_sql_from_file`].
    fn create_schema(&self) -> Result<(), ImpulsePhmError>;


    /// Insert initial values into the database based on a file that contains the SQL commands
    /// 
    /// Concrete types are expected to provide the file path and pass it to 
    /// [`Query::execute_sql_from_file`].
    fn initialize(&self) -> Result<(), ImpulsePhmError>;

    /// A default implementation to bulk/batch execute SQL commands from a file
    /// 
    /// The primary use-case here is to execute the commands specifically to help setup the 
    /// local environment of the application.
    /// 
    /// # Parameters:
    /// `path`: The path to the file contains the SQL commands
    /// 
    /// # Errors:
    /// 1. Returns an [`ImpulsePhmError::IoError`] if the file cannot be read
    /// 2. Returns an [`ImpulsePhmError::DatabaseError`] if the SQL commands could not be executed
    fn execute_sql_from_file(&self, path: &Path) -> Result<(), ImpulsePhmError> {
        let sql = fs::read_to_string(path)?;
        self.get_connection().execute_batch(sql.as_str())?;
        Ok(())
    }
}
