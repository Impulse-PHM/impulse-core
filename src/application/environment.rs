//! Functionality to manage the application's local environment

use crate::{database::{core::CoreDatabase, 
    user::{UserDatabase, UserDatabaseValidator}, Query}, 
    error::ImpulsePhmError};

/// Represents the operations that can be used to manage the end-user's local environment
pub struct Environment<'a, 'b> {
    core_database: &'a CoreDatabase,
    user_database: &'b UserDatabase
}

impl<'a, 'b> Environment<'a, 'b> {
    pub fn new(core_database: &'a CoreDatabase, user_database: &'b UserDatabase) -> Self {
        Environment {
            core_database: core_database,
            user_database: user_database
        }
    }

    /// Execute SQL queries to create the core database's schema and insert initial values
    /// 
    /// # Errors:
    /// The errors that can be returned from this method are documented already in 
    /// [`Query::execute_sql_from_file`] as this method, ultimately, calls it.
    pub fn setup_core_database(&self) -> Result<(), ImpulsePhmError> {
        if let Err(e) = self.core_database.create_schema() {
            log::error!("Failed to create the schema for the core database: {}", e);
            return Err(e);
        }

        if let Err(e) = self.core_database.initialize() {
            log::error!("Failed to initialize the core database: {}", e);
            return Err(e);
        }

        Ok(())
    }

    /// Execute SQL queries to create the core database's schema and insert initial values
    /// 
    /// The errors that can be returned from this method are documented already in 
    /// [`Query::execute_sql_from_file`] as this method, ultimately, calls it.
    pub fn setup_user_database(&self) -> Result<(), ImpulsePhmError> {
        if let Err(e) = self.user_database.create_schema() {
            log::error!("Failed to create the schema for the user database: {}", e);
            return Err(e);
        }

        if let Err(e) = self.user_database.initialize() {
            log::error!("Failed to initialize the user database: {}", e);
            return Err(e);
        }

        // The validation checks, in this context, serve as a final confirmation of that the 
        // user database is correct and especially meets all of the standards of the applicatoin's 
        // file format.
        if let Err(e) = UserDatabaseValidator::check_file_properties(self.user_database.get_database_path()) {
            log::error!("The check file properties validation check failed: {}", e);
            return Err(e);
        }

        if let Err(e) = UserDatabaseValidator::check_tables(&self.user_database) {
            log::error!("The check tables validation check failed: {}", e);
            return Err(e);
        }

        if let Err(e) = UserDatabaseValidator::check_schema_version(&self.user_database) {
            log::error!("The check schema version validation check failed: {}", e);
            return Err(e);
        }

        Ok(())
    }
}
