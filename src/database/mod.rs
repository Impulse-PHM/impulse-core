//! Functionality to manage the databases

pub mod core;
pub mod user;

use std::{fs, path::Path};

use rusqlite::Connection;

use crate::error::ImpulsePhmError;


/// Allows for the creation and querying of a database
pub trait Query {    
    /// Get the path to the database file
    fn get_database_path(&self) -> &Path;

    /// Get the current connection
    fn get_connection(&self) -> &Connection;

    /// A default implementation to configure the connection 
    /// 
    /// Ideally, this should be executed right after creating a connection.
    fn configure_connection(&self) -> Result<(), rusqlite::Error> {
        self.enable_foreign_keys_enforcement()?;
        self.enable_secure_delete()
    }

    /// A default implementation to enable the foreign_keys PRAGMA for actual data integrity
    fn enable_foreign_keys_enforcement(&self) -> Result<(), rusqlite::Error> {
        self
            .get_connection()
            .pragma_update(None, "foreign_keys", 1)?;
        
        Ok(())
    }

    /// A default implementation to enable secure delete
    fn enable_secure_delete(&self) -> Result<(), rusqlite::Error> {
        self
            .get_connection()
            .pragma_update(None, "secure_delete", 1)?;
        
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
    /// 1. Returns an [`ImpulsePhmError::Io`] if the file cannot be read
    /// 2. Returns an [`ImpulsePhmError::Database`] if the SQL commands could not be executed
    fn execute_sql_from_file(&self, path: &Path) -> Result<(), ImpulsePhmError> {
        let sql = fs::read_to_string(path)?;
        self.get_connection().execute_batch(sql.as_str())?;
        Ok(())
    }

    /// A default implementation that checks whether a database has custom tables or not
    /// 
    /// "Custom" tables refers to any tables created by this application rather than the ones 
    /// SQLite automatically creates. This method is useful to determine whether a database 
    /// needs to be setup or not for an end-user.
    /// 
    /// # Returns:
    /// Whether there are custom tables or not
    /// 
    /// # Errors:
    /// Returns a [`rusqlite::Error`] if there was an issue querying the database
    fn has_tables(&self) -> Result<bool, rusqlite::Error> {
        let count_result = self
            .get_connection()
            .query_one(
            "SELECT COUNT(*) 
            FROM sqlite_master 
            WHERE type = 'table' 
            AND name NOT LIKE 'sqlite_%';",
            [],
            |row| row.get(0),
        );

        let count: u64 = match count_result {
            Ok(value) => value,
            Err(e) => {
                log::error!("An error occurred when checking if the database has tables: {e}");
                return Err(e);
            },
        };

        if count > 0 {
            Ok(true)
        }
        else {
            Ok(false)
        }
    }
}

/// Allows a database to be validated at any point in time
/// 
/// These methods are useful in the following situations:
/// 1. Right after setting up a new database.
/// 2. Before upgrading an existing database.
/// 3. After upgrading an existing database.
/// 4. At any point in time the application suspects a database is the potential source of an issue.
pub trait Validate: Query {
    /// A default implementation that checks if the foreign_keys PRAGMA is enabled
    /// 
    /// # Errors:
    /// 1. Returns a [`ImpulsePhmError::InvalidPragma`] if the pragma is not enabled
    /// 2. Returns an [`ImpulsePhmError::Database`] for any other database or SQL problems 
    fn check_foreign_keys_pragma(&self) -> Result<(), ImpulsePhmError> {
        let pragma_result: Result<u8, rusqlite::Error> = self
            .get_connection()
            .query_one(
            "SELECT * FROM pragma_foreign_keys;",
            [],
            |row| row.get(0) 
            );
        
        match pragma_result {
            Ok(1) => Ok(()),
            Ok(_) => {
                log::error!("The foreign_keys PRAGMA is not enabled");
                return Err(
                    ImpulsePhmError::InvalidPragma("The foreign_keys PRAGMA is not \
                    enabled".to_owned())
                );
            },
            Err(e) => {
                log::error!("An error occurred when checking the foreign_keys PRAGMA: {e}");
                return Err(ImpulsePhmError::Database(e));
            }
        }
    }

    /// A default implementation that checks if the secure_delete PRAGMA is enabled
    /// 
    /// # Errors:
    /// 1. Returns a [`ImpulsePhmError::InvalidPragma`] if the pragma is not enabled
    /// 2. Returns an [`ImpulsePhmError::Database`] for any other database or SQL problems 
    fn check_secure_delete_pragma(&self) -> Result<(), ImpulsePhmError> {
        let pragma_result: Result<u8, rusqlite::Error> = self
            .get_connection()
            .query_one(
            "SELECT * FROM pragma_secure_delete;",
            [],
            |row| row.get(0) 
            );
        
        match pragma_result {
            Ok(1) => Ok(()),
            Ok(_) => {
                log::error!("The secure_delete PRAGMA is not enabled");
                return Err(
                    ImpulsePhmError::InvalidPragma("The secure_delete PRAGMA is not \
                    enabled".to_owned())
                );
            },
            Err(e) => {
                log::error!("An error occurred when checking the secure_delete PRAGMA: {e}");
                return Err(ImpulsePhmError::Database(e));
            }
        }
    }

    /// Check if the given file's properties are valid
    /// 
    /// # Errors:
    /// Returns an [`ImpulsePhmError::Io`] if the file cannot be found.
    fn check_file_properties(&self) -> Result<(), ImpulsePhmError>;

    /// Check if the given database has all of the required tables
    /// 
    /// # Errors:
    /// 1. Returns an [`ImpulsePhmError::MissingTable`] specifically when a table is imssing
    /// 2. Returns an [`ImpulsePhmError::Database`] for any other database or SQL problems
    fn check_tables(&self) -> Result<(), ImpulsePhmError>;

    /// Check that the given database's schema version is valid
    /// 
    /// # Parameters:
    /// `database`: The database to check
    /// 
    /// # Errors:
    /// 1. Returns an [`ImpulsePhmError::SemVerParse`] if there was an issue parsing the version
    /// 1. Returns an [`ImpulsePhmError::InvalidSchemaVersion`] if the version is an empty string
    /// 1. Returns an [`ImpulsePhmError::InvalidSchemaVersion`] if the version is not in semantic versioning format
    /// 1. Returns an [`ImpulsePhmError::InvalidSchemaVersion`] if the schema version is not >= 0.1.0
    /// 1. Returns an [`ImpulsePhmError::Database`] for any other database or SQL problems
    fn check_schema_version(&self) -> Result<(), ImpulsePhmError>;

}
