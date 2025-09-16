//! Everything needed to manage the application's core database

use std::path::{Path, PathBuf};

use rusqlite::Connection;

use crate::{database::Query, error::ImpulsePhmError};


/// The path to the file that creates the core database's schema
pub const CORE_DATABASE_SCHEMA: &str = "resources/setup/core_schema.sql";

/// The path to the file that inserts initial data into the core database
pub const CORE_DATABASE_INIT: &str = "resources/setup/core_init.sql";

/// Represents the application's core database
pub struct CoreDatabase {
    database_path: PathBuf,
    connection: Connection,
    schema_path: PathBuf,
    init_path: PathBuf
}

impl CoreDatabase {
    /// Create a new object with a pre-configured connection to the core database
    /// 
    /// 
    /// # Parameters:
    /// `database_path`: The database's file path
    /// `schema_path`: The path to a file to create the schema.
    /// `init_path`: The path to a file to insert initial values
    /// 
    /// # Errors:
    /// Returns an [`Error`] if a connection cannot be created or if a connection cannot be 
    /// configured.
    pub fn new(database_path: &Path, schema_path: &Path, 
        init_path: &Path) -> Result<Self, rusqlite::Error> {
        let connection = Connection::open(database_path)?;
        log::debug!("The connection was created successfully");

        let core_database = Self {
            database_path: database_path.to_owned(),
            connection: connection, 
            schema_path: schema_path.to_owned(), 
            init_path: init_path.to_owned()
        };

        core_database.configure_connection()?;
        log::debug!("The connection was configured successfully");

        Ok(core_database)
    }
}

impl Query for CoreDatabase {
    fn connect(path: &Path) -> Result<Connection, ImpulsePhmError> {
        Self::connect_default(path)
    }

    fn get_database_path(&self) -> &Path {
        &self.database_path
    }
    
    fn get_connection(&self) -> &Connection {
        &self.connection
    }
    
    fn configure_connection(&self) -> Result<(), rusqlite::Error> {
        // Enabling foreign key enforcement is enough for the core database as no sensitive 
        // information will be stored in it. Thus, secure delete does not need to be enabled here.
        self.enable_foreign_key_enforcement()
    }

    fn create_schema(&self) -> Result<(), ImpulsePhmError> {
        self.execute_sql_from_file(&self.schema_path)
    }

    fn initialize(&self) -> Result<(), ImpulsePhmError> {
        self.execute_sql_from_file(&self.init_path)
    }
}
