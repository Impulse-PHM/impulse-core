//! The integration tests for the "environment" feature

mod common;

use std::path::Path;

use tempfile::NamedTempFile;

use impulse_phm::{application::CoreApplication, 
    database::{core::CoreDatabase, user::{UserDatabase, UserDatabaseValidator}, Query}, 
    error::ImpulsePhmError};


    /// Verify that the core database can be created and set up correctly
#[test]
fn setup_core_database() {
    common::setup_logging();

    let core_db_temp_file: NamedTempFile = common::create_core_database_temp_file();
    let user_db_temp_file: NamedTempFile = common::create_user_database_temp_file();

    let core_database: CoreDatabase = common::create_test_core_database(core_db_temp_file.path())
        .expect("Failed to create the core database");

    let user_database: UserDatabase = common::create_test_user_database_with_defaults(user_db_temp_file.path())
        .expect("Failed to create the user database");
    
    let app = CoreApplication::new(&core_database, &user_database);

    app
        .for_environment()
        .setup_core_database()
        .expect("Failed to setup the core database correctly");

    // Verify foreign key enforcement is enabled
    let foreign_keys_pragma: u8 = core_database
        .get_connection()
        .query_one(
            "SELECT * FROM pragma_foreign_keys;",
            [],
            |row| row.get(0)
        )
        .expect("Failed to enable the \"foreign_keys\" pragma.");

    assert_eq!(foreign_keys_pragma, 1, "the foreign_keys pragma was not enabled");
    
    // Verify the existence of specific tables
    let app_log_table = "app_log";
    let database_release_table = "database_release";
    core_database
        .get_connection()
        .table_exists(None, app_log_table)
        .expect(format!("The table \"{}\" was not created", app_log_table).as_str());

    core_database
        .get_connection()
        .table_exists(None, database_release_table)
        .expect(format!("The table \"{}\" was not created", database_release_table).as_str());

    // Verify an initialization value
    let db_schema_version: String = core_database
        .get_connection()
        .query_one(
            "SELECT version FROM database_release ORDER BY created_at ASC LIMIT 1;",
            [],
            |row| row.get(0) 
        )
        .expect("No initial value for the schema version was found");

    assert_eq!(
        db_schema_version, 
        common::CORE_DATABASE_INITIAL_SCHEMA_VERSION, 
        "the schema version was found, but it's not the expected value"
    );

}

/// Verify that the user database can be created and set up correctly
/// 
/// Because the user database is also the application's custom file format, there are additional 
/// checks required here in contrast to the core database.
#[test]
fn setup_user_database() {
    common::setup_logging();

    let core_db_temp_file: NamedTempFile = common::create_core_database_temp_file();
    let user_db_temp_file: NamedTempFile = common::create_user_database_temp_file();

    let core_database: CoreDatabase = common::create_test_core_database(core_db_temp_file.path())
        .expect("Failed to create the core database");

    let user_database: UserDatabase = common::create_test_user_database_with_defaults(user_db_temp_file.path())
        .expect("Failed to create the user database");

    let app = CoreApplication::new(&core_database, &user_database);

    app
        .for_environment()
        .setup_user_database()
        .expect("Failed to setup the user database correctly");

    // Verify foreign key enforcement is enabled
    let foreign_keys_pragma: u8 = user_database
        .get_connection()
        .query_one(
            "SELECT * FROM pragma_foreign_keys;",
            [],
            |row| row.get(0)
        )
        .expect("Failed to enable the \"foreign_keys\" pragma.");

    assert_eq!(foreign_keys_pragma, 1, "The foreign_keys pragma was not enabled");
    
    // Verify the additional checks for being an application file format
    UserDatabaseValidator::check_file_properties(user_db_temp_file.path())
        .expect("The given user database is not a valid application file format of this application");

    // Verify the existence of the required tables
    UserDatabaseValidator::check_tables(&user_database)
        .expect("There was an unexpected error when checking the tables");
    
    // Verify the existence of a schema version
    UserDatabaseValidator::check_schema_version(&user_database)
        .expect("There was an unexpected error when checking the schema version");
}

/// Verify that an error occurs when attempting to setup a user database with missing tables
/// 
/// In order for this test to work properly, the tables removed cannot be referenced by any other 
/// table in the schema file. Additionally, those removed cannot be tables that need to be 
/// initialized in the init file.
#[test]
fn setup_user_database_with_missing_tables() {
common::setup_logging();

    let core_db_temp_file: NamedTempFile = common::create_core_database_temp_file();
    let user_db_temp_file: NamedTempFile = common::create_user_database_temp_file();

    let core_database: CoreDatabase = common::create_test_core_database(core_db_temp_file.path())
        .expect("Failed to create the core database");

    let user_database: UserDatabase = common::create_test_user_database(
        user_db_temp_file.path(), 
        Some(Path::new(common::USER_DATABASE_SCHEMA_WITH_MISSING_TABLES)),
        None
    )
        .expect("Failed to create the user database");

    let app = CoreApplication::new(&core_database, &user_database);

    match app
        .for_environment()
        .setup_user_database()
        .expect_err("Expected an error due to missing tables") {
            ImpulsePhmError::AppFileFormatError(_) => (),
            _ => panic!("An error was returned, but it was the wrong type")
        }

    
}

/// Verify that an error occurs when attempting to setup a user database with an invalid 
/// schema version.
#[test]
fn setup_user_database_with_invalid_version() {
common::setup_logging();

    let core_db_temp_file: NamedTempFile = common::create_core_database_temp_file();
    let user_db_temp_file: NamedTempFile = common::create_user_database_temp_file();

    let core_database: CoreDatabase = common::create_test_core_database(core_db_temp_file.path())
        .expect("Failed to create the core database");

    let user_database: UserDatabase = common::create_test_user_database(
        user_db_temp_file.path(), 
        None,
        Some(Path::new(common::USER_DATABASE_INIT_WITH_INVALID_VERSION))
    )
        .expect("Failed to create the user database");

    let app = CoreApplication::new(&core_database, &user_database);

    match app
        .for_environment()
        .setup_user_database()
        .expect_err("Expected an error due to initializing with an invalid schema version") {
            ImpulsePhmError::AppFileFormatError(_) => (),
            _ => panic!("An error was returned, but it was the wrong type")
        }

}
