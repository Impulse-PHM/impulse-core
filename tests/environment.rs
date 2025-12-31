//! The integration tests for [`impulse_phm::environment`]

mod common;

use std::path::PathBuf;

use tempfile::NamedTempFile;

use impulse_phm::{
    database::{core::CoreDatabase, user::UserDatabase, Query, Validate}, 
    environment,
    error::ImpulsePhmError
};


/// Verify that the core database can be created and set up correctly
#[test]
fn setup_core_database() {
    common::setup_logging();

    let core_db_temp_file: NamedTempFile = common::create_core_database_temp_file();

    let core_database: CoreDatabase = common::create_test_core_database_with_defaults(core_db_temp_file.path().to_owned())
        .expect("Failed to create the core database");

    let has_tables = core_database
        .has_tables()
        .expect("Failed to check if there are any tables");
    
    assert_eq!(has_tables, false, "There should not be any tables yet");

    core_database.check_file_properties()
        .expect("An error occurred when checking the database file's properties");
    
    environment::setup_core_database(&core_database)
        .expect("Failed to setup the core database correctly");

    // Verify foreign_keys PRAGMA is enabled
    core_database.check_foreign_keys_pragma()
        .expect("An error occurred when checking the foreign_keys PRAGMA");

    // Verify the secure_delete PRAGMA is enabled
    core_database.enable_secure_delete()
        .expect("An error occurred when checking the secure_delete PRAGMA");
    
    let has_tables = core_database
        .has_tables()
        .expect("Failed to check if there are any tables");
    
    assert_eq!(has_tables, true, "There should now be tables");
    
    // Verify the existence of the required tables
    core_database.check_tables()
        .expect("There was an unexpected error when checking the tables");

    // Verify the existence of a schema version
    core_database.check_schema_version()
        .expect("There was an unexpected error when checking the schema version");

}

/// Verify that an error occurs when attempting to setup a core database with missing tables
/// 
/// In order for this test to work properly, the tables removed cannot be referenced by any other 
/// table in the schema file. Additionally, those removed cannot be tables that need to be 
/// initialized in the init file.
#[test]
fn setup_core_database_fails_with_missing_tables() {
    common::setup_logging();

    let core_db_temp_file: NamedTempFile = common::create_core_database_temp_file();

    let core_database: CoreDatabase = common::create_test_core_database(
        core_db_temp_file.path().to_owned(), 
        Some(PathBuf::from(common::CORE_DATABASE_SCHEMA_WITH_MISSING_TABLES)),
        None
    )
        .expect("Failed to create the core database");

    match environment::setup_core_database(&core_database)
        .expect_err("Expected an error due to missing tables") {
            ImpulsePhmError::MissingTable(_) => (),
            e => panic!("An error was returned, but it was the wrong type: {e}")
        }
}

/// Verify that an error occurs when attempting to setup a core database with an invalid 
/// schema version.
#[test]
fn setup_core_database_fails_with_invalid_schema_version() {
    common::setup_logging();

    let core_db_temp_file: NamedTempFile = common::create_core_database_temp_file();

    let core_database: CoreDatabase = common::create_test_core_database(
        core_db_temp_file.path().to_owned(), 
        None,
        Some(PathBuf::from(common::CORE_DATABASE_INIT_WITH_INVALID_SCHEMA_VERSION))
    )
        .expect("Failed to create the core database");

    match environment::setup_core_database(&core_database)
        .expect_err("Expected an error due to initializing with an invalid schema version") {
            ImpulsePhmError::InvalidSchemaVersion(_) => (),
            e => panic!("An error was returned, but it was the wrong type: {e}")
        }

}


/// Verify that the user database can be created and set up correctly
/// 
/// Because the user database is also the application's custom file format, there are additional 
/// checks required here in contrast to the core database.
#[test]
fn setup_user_database() {
    common::setup_logging();

    let user_db_temp_file: NamedTempFile = common::create_user_database_temp_file();

    let user_database: UserDatabase = common::create_test_user_database_with_defaults(user_db_temp_file.path().to_owned())
        .expect("Failed to create the user database");

    let has_tables = user_database
        .has_tables()
        .expect("Failed to check if there are any tables");
    
    assert_eq!(has_tables, false, "There should not be any tables yet");

    user_database.check_file_properties()
        .expect("An error occurred when checking the database file's properties");

    environment::setup_user_database(&user_database)
        .expect("Failed to setup the user database correctly");

    // Verify foreign_keys PRAGMA is enabled
    user_database.check_foreign_keys_pragma()
        .expect("An error occurred when checking the foreign_keys PRAGMA");

    // Verify the secure_delete PRAGMA is enabled
    user_database.enable_secure_delete()
        .expect("An error occurred when checking the secure_delete PRAGMA");

    let has_tables = user_database
        .has_tables()
        .expect("Failed to check if there are any tables");
    
    assert_eq!(has_tables, true, "There should now be tables");

    // Verify the existence of the required tables
    user_database.check_tables()
        .expect("There was an unexpected error when checking the tables");
    
    // Verify the existence of a schema version
    user_database.check_schema_version()
        .expect("There was an unexpected error when checking the schema version");

}

/// Verify an error is returned when the database file does not have a file extension
#[test]
fn setup_user_database_fails_with_no_file_extension() {
    common::setup_logging();

    let file_without_extension = NamedTempFile::new()
        .expect("Failed to create the temp file");

    let user_database: UserDatabase = common::create_test_user_database(
        file_without_extension.path().to_owned(), 
        Some(PathBuf::from(common::USER_DATABASE_SCHEMA)),
        None
    )
        .expect("Failed to create the user database");

    match environment::setup_user_database(&user_database)
        .expect_err("Expected the database file not having a file extension") {
            ImpulsePhmError::AppFileFormat(_) => (),
            e => panic!("An error was returned, but it was the wrong type: {e}")
        }
}


/// Verify an error is returned when the database uses a wrong file extension
#[test]
fn setup_user_database_fails_with_wrong_file_extension() {
    common::setup_logging();

    let file_with_wrong_extension = NamedTempFile::with_suffix(".docx")
        .expect("Failed to create the temp file");

    let user_database: UserDatabase = common::create_test_user_database(
        file_with_wrong_extension.path().to_owned(), 
        Some(PathBuf::from(common::USER_DATABASE_SCHEMA)),
        None
    )
        .expect("Failed to create the user database");

    match environment::setup_user_database(&user_database)
        .expect_err("Expected the database file not having a file extension") {
            ImpulsePhmError::AppFileFormat(_) => (),
            e => panic!("An error was returned, but it was the wrong type: {e}")
        }
}

/// Verify that an error occurs when attempting to setup a user database with missing tables
/// 
/// In order for this test to work properly, the tables removed cannot be referenced by any other 
/// table in the schema file. Additionally, those removed cannot be tables that need to be 
/// initialized in the init file.
#[test]
fn setup_user_database_fails_with_missing_tables() {
    common::setup_logging();

    let user_db_temp_file: NamedTempFile = common::create_user_database_temp_file();

    let user_database: UserDatabase = common::create_test_user_database(
        user_db_temp_file.path().to_owned(), 
        Some(PathBuf::from(common::USER_DATABASE_SCHEMA_WITH_MISSING_TABLES)),
        None
    )
        .expect("Failed to create the user database");

    match environment::setup_user_database(&user_database)
        .expect_err("Expected an error due to missing tables") {
            ImpulsePhmError::MissingTable(_) => (),
            e => panic!("An error was returned, but it was the wrong type: {e}")
        }
}

/// Verify that an error occurs when attempting to setup a user database with an invalid 
/// schema version.
#[test]
fn setup_user_database_fails_with_invalid_schema_version() {
    common::setup_logging();

    let user_db_temp_file: NamedTempFile = common::create_user_database_temp_file();

    let user_database: UserDatabase = common::create_test_user_database(
        user_db_temp_file.path().to_owned(), 
        None,
        Some(PathBuf::from(common::USER_DATABASE_INIT_WITH_INVALID_SCHEMA_VERSION))
    )
        .expect("Failed to create the user database");

    match environment::setup_user_database(&user_database)
        .expect_err("Expected an error due to initializing with an invalid schema version") {
            ImpulsePhmError::InvalidSchemaVersion(_) => (),
            e => panic!("An error was returned, but it was the wrong type: {e}")
        }

}
