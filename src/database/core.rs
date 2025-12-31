//! Everything needed to manage the application's core database

use std::{collections::HashSet, io, path::{Path, PathBuf}};

use rusqlite::Connection;
use semver::{Version, VersionReq};

use crate::{database::{Query, Validate}, error::ImpulsePhmError};


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
    /// Returns a [`rusqlite::Error`] if a connection cannot be created or if a connection 
    /// cannot be configured.
    pub fn new(database_path: PathBuf, schema_path: PathBuf, 
        init_path: PathBuf) -> Result<Self, rusqlite::Error> {
        let connection = Connection::open(&database_path)?;
        log::debug!("The connection was created successfully");

        let core_database = Self {
            database_path: database_path,
            connection: connection, 
            schema_path: schema_path, 
            init_path: init_path
        };

        core_database.configure_connection()?;
        log::debug!("The connection was configured successfully");

        Ok(core_database)
    }
}

impl Query for CoreDatabase {
    fn get_database_path(&self) -> &Path {
        &self.database_path
    }
    
    fn get_connection(&self) -> &Connection {
        &self.connection
    }

    fn create_schema(&self) -> Result<(), ImpulsePhmError> {
        self.execute_sql_from_file(&self.schema_path)
    }

    fn initialize(&self) -> Result<(), ImpulsePhmError> {
        self.execute_sql_from_file(&self.init_path)
    }
}

impl Validate for CoreDatabase {
    fn check_file_properties(&self) -> Result<(), ImpulsePhmError> {
        let path = &self.database_path;
        if !path.exists() {
            log::error!("The given path does not exist");
            Err(
                ImpulsePhmError::Io(
                    io::Error::new(io::ErrorKind::NotFound,
                    "The given path does not exist".to_owned())
                )
            )
        }
        else {
            Ok(())
        }
    }

    fn check_tables(&self) -> Result<(), ImpulsePhmError> {
        log::debug!("Checking to see if the core database has all of the required tables");

        let required_tables: HashSet<&str> = HashSet::from(["app_log", "database_release"]);

        for table in required_tables {
            let has_table_result = self
                .get_connection()
                .table_exists(Some("main"), table);
            
            match has_table_result {
                Ok(true) => log::debug!("Confirmed to have table: {}", table),
                Ok(false) => {
                    log::error!("Missing table {}", table);
                    return Err(
                        ImpulsePhmError::MissingTable(
                        format!("Missing table {}", table))
                    );
                },
                Err(e) => {
                    log::error!("An error occurred when checking if table {table} exists: {e}");
                    return Err(ImpulsePhmError::Database(e));
                }
            }
        }

        log::debug!("The core database has all of the required tables");
        Ok(())
    }

    fn check_schema_version(&self) -> Result<(), ImpulsePhmError> {
        let result: Result<String, rusqlite::Error> = self
            .get_connection()
            .query_one(
            "SELECT version FROM database_release ORDER BY created_at DESC LIMIT 1;",
            [],
            |row| row.get(0) 
            );
        
        let version: String = match result {
            Ok(value) => value,
            Err(e) => {
                log::error!(
                    "An error occurred when running the SQL queries to get the database's schema \
                    version: {}", e
                );

                return Err(ImpulsePhmError::Database(e));
            }
        };

        if version.is_empty() {
            log::error!("The schema version has an empty string value");
            return Err(ImpulsePhmError::InvalidSchemaVersion(
                "The schema version has an empty string value".to_owned())
            );
        }

        // Create and parse the semantic version requirement/rule
        let semantic_version_requirement = match VersionReq::parse(">=0.1.0") {
            Ok(req) => req,
            Err(e) => {
                log::error!(
                    "There was an error when creating and parsing the semantic version \
                    requirement: {}", e
                );

                return Err(ImpulsePhmError::SemVerParse(e));
            }
        };
        
        // Check if the version uses the general semantic versioning format
        let valid_semantic_version = match Version::parse(&version) {
            Ok(value) => value,
            Err(e) => {
                log::error!(
                    "The schema version does not follow the general semantic versioning \
                    format: {}", e);

                return Err(
                    ImpulsePhmError::InvalidSchemaVersion(
                        format!("The schema version does not follow the general semantic \
                        versioning format: {}", e)
                    )
                );
            }
        };

        // Check if the version, confirmed to be in semantic form, meets the custom requirement 
        // created earlier.
        if !semantic_version_requirement.matches(&valid_semantic_version) {
            log::error!(
                "The schema version has a valid semantic versioning format, but it does not meet \
                the custom version requirement: \"{}\"", semantic_version_requirement
            );

            return Err(
                ImpulsePhmError::InvalidSchemaVersion(
                    format!(
                        "The schema version has a valid semantic versioning format, but it \
                        does not meet the custom version requirement: {}", 
                        semantic_version_requirement
                    )
                )
            );
        }

        Ok(())
    }
}
