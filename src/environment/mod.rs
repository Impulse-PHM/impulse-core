//! Functionality to manage the local environment
//! 
//! Specifically, this module provides types and operations to help simplify the management of 
//! the local environment such as logging, setting up databases, resources, and localization.

pub mod resource;

use flexi_logger::{Duplicate, FileSpec, Logger};

use crate::{
    database::{core::CoreDatabase, user::UserDatabase, Query, Validate}, 
    environment::resource::{LOG_BASE_NAME, LOG_SUFFIX},
    error::ImpulsePhmError, model::ImpulseCore
};


/// Setup logging for the application by initializing and configuring the logger
/// 
/// This method should only be called from src/main.rs and, ideally, should be the first 
/// function called.
pub fn setup_logging() -> Result<(), flexi_logger::FlexiLoggerError> {
    // The RUST_LOG environment variable, if explicitly set, takes precedence over the log level 
    // set below. Otherwise, the value below will be used.
    Logger::try_with_env_or_str("error")?
        .log_to_file(
            FileSpec::default()
                .directory(resource::get_log_directory())
                .basename(LOG_BASE_NAME)
                .suffix(LOG_SUFFIX)
                .use_timestamp(false)
        )
        // Duplicate any messages that get logged to the log file (which are still determined by 
        // the log level used above or the RUST_LOG environment variable if explicitly set).
        .duplicate_to_stderr(Duplicate::All)
        .format_for_files(flexi_logger::detailed_format)
        .format_for_stderr(flexi_logger::colored_detailed_format)
        .start()?;

    Ok(())
}

/// Get the core database (new or existing) using predetermined resource file paths.
/// 
/// Creates a new core database if one doesn't already exist, or returns the existing one.
/// 
/// # Errors:
/// A [`rusqlite::Error`] is returned if there was an issue with creating the database
pub fn get_core_database() -> Result<CoreDatabase, rusqlite::Error> {
    match CoreDatabase::new(
        resource::get_core_database(), 
        resource::get_core_database_schema(), 
        resource::get_core_database_init()
    ) 
    {
        Ok(valid_database) => Ok(valid_database),
        Err(e) => {
            log::error!("Failed to create the core database: {}", e);
            return Err(e);
        },
    }
}

/// Get the user database (new or existing) using predetermined resource file paths.
/// 
/// Creates a new user database if one doesn't already exist, or returns the existing one.
/// 
/// # Errors:
/// 1. [`ImpulsePhmError::Io`] if there was a problem with getting the path to the user database
/// 2. [`ImpulsePhmError::Database`] if there was an issue with creating the database
pub fn get_user_database() -> Result<UserDatabase, ImpulsePhmError> {
    let database_path = match resource::get_user_database() {
        Ok(valid_path) => valid_path,
        Err(e) => {
            log::error!("Failed to get the path to the user database: {}", e);
            return Err(ImpulsePhmError::Io(e));
        },
    };
    
    match UserDatabase::new(
        database_path, 
        resource::get_user_database_schema(), 
        resource::get_user_database_init()
    ) 
    {
        Ok(valid_database) => Ok(valid_database),
        Err(e) => {
            log::error!("Failed to create the user database: {}", e);
            return Err(ImpulsePhmError::Database(e));
        },
    }
}

/// Set up the core database
/// 
/// Behind the scenes SQL queries are executed to create the schema, insert initial values, 
/// and then final validation checks are performed to make sure the database is ready to be 
/// used.
/// 
/// # Errors:
/// The errors that can be returned from this method are documented already in 
/// [`Query::execute_sql_from_file`] as this method, ultimately, calls it.
pub fn setup_core_database(core_database: &CoreDatabase) -> Result<(), ImpulsePhmError> {
    if let Err(e) = core_database.create_schema() {
        log::error!("Failed to create the schema for the core database: {}", e);
        return Err(e);
    }

    if let Err(e) = core_database.initialize() {
        log::error!("Failed to initialize the core database: {}", e);
        return Err(e);
    }

    // The validation checks, in this context, serve as a final confirmation that the 
    // core database is correct.
    if let Err(e) = core_database.check_file_properties() {
        log::error!("The check file properties validation failed: {}", e);
        return Err(e);
    }

    if let Err(e) = core_database.check_tables() {
        log::error!("The check tables validation failed: {}", e);
        return Err(e);
    }

    if let Err(e) = core_database.check_schema_version() {
        log::error!("The check schema version validation failed: {}", e);
        return Err(e);
    }

    if let Err(e) = core_database.check_foreign_keys_pragma() {
        log::error!("The check foreign_keys PRAGMA validation failed: {}", e);
        return Err(e);
    }

    if let Err(e) = core_database.check_secure_delete_pragma() {
        log::error!("The check secure_delete PRAGMA validation failed: {}", e);
        return Err(e);
    }

    Ok(())
}

/// Set up the user database
/// 
/// Behind the scenes SQL queries are executed to create the schema, insert initial values, 
/// and then final validation checks are performed to make sure the database is ready to be 
/// used.
/// 
/// # Errors:
/// The errors that can be returned from this method are documented already in 
/// [`Query::execute_sql_from_file`] as this method, ultimately, calls it.
pub fn setup_user_database(user_database: &UserDatabase) -> Result<(), ImpulsePhmError> {
    if let Err(e) = user_database.create_schema() {
        log::error!("Failed to create the schema for the user database: {}", e);
        return Err(e);
    }

    if let Err(e) = user_database.initialize() {
        log::error!("Failed to initialize the user database: {}", e);
        return Err(e);
    }

    // The validation checks, in this context, serve as a final confirmation that the 
    // user database is correct and especially meets all of the standards of the application's 
    // file format.
    if let Err(e) = user_database.check_file_properties() {
        log::error!("The check file properties validation failed: {}", e);
        return Err(e);
    }

    if let Err(e) = user_database.check_tables() {
        log::error!("The check tables validation failed: {}", e);
        return Err(e);
    }

    if let Err(e) = user_database.check_schema_version() {
        log::error!("The check schema version validation failed: {}", e);
        return Err(e);
    }

    if let Err(e) = user_database.check_foreign_keys_pragma() {
        log::error!("The check foreign_keys PRAGMA validation failed: {}", e);
        return Err(e);
    }

    if let Err(e) = user_database.check_secure_delete_pragma() {
        log::error!("The check secure_delete PRAGMA validation failed: {}", e);
        return Err(e);
    }

    Ok(())
}

/// A convenience function to set up a new, or use an existing, local environment using 
/// predetermined resource file paths.
/// 
/// This function was created so that calling code can just use one function to execute all of the 
/// setup logic for the local environment without having to deal with the boilerplate.
/// 
/// # Returns:
/// An instance of [`ImpulseCore`] which, in this case, means the environment is ready to be used
/// 
/// # Errors:
/// See [`get_core_database`], [`get_user_database`], [`setup_core_database`], 
/// [`setup_user_database`], and [`Query::has_tables`] as the errors are documented there.
pub fn setup_environment() -> Result<ImpulseCore, ImpulsePhmError> {
    log::debug!("Attempting to get or create the core database");
    let core_database: CoreDatabase = get_core_database()?;

    log::debug!("Attempting to get or create the user database");
    let user_database: UserDatabase = get_user_database()?;

    log::debug!("Checking if the core database has any existing tables");
    match core_database.has_tables() {
        Ok(true) => (), // Do nothing as the database has already been set up
        Ok(false) => {
            log::debug!("No tables found in the core database, so attempting to set it up.");
            setup_core_database(&core_database)?;
        },
        Err(e) => {
            log::error!("Failed to determine if the core database already has tables: {}", e);
            return Err(ImpulsePhmError::Database(e));
        }
    };

    log::debug!("Checking if the user database has any existing tables");
    match user_database.has_tables() {
        Ok(true) => (), // Do nothing as the database has already been set up
        Ok(false) => {
            log::debug!("No tables found in the user database, so attempting to set it up.");
            setup_user_database(&user_database)?;
        },
        Err(e) => {
            log::error!("Failed to determine if the user database already has tables: {}", e);
            return Err(ImpulsePhmError::Database(e));
        }
    };

    log::debug!("The environment was set up successfully");

    Ok(ImpulseCore::new(core_database, user_database))
}
