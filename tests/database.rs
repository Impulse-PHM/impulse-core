//! The integration tests for [`impulse_phm::database`]
mod common;

use std::{fs, path::{Path, PathBuf}};

use tempfile::NamedTempFile;

use impulse_core::{
    database::{core::CoreDatabase, user::UserDatabase, Query, Validate}, error::ImpulsePhmError
};


/// Verify that a core database file can be created with the correct connection configuration
#[test]
fn create_core_database() {
    common::setup_logging();

    let database_path = Path::new("/tmp/create_core_database_test_file");
    let schema_path = Path::new(common::CORE_DATABASE_SCHEMA);
    let init_path = Path::new(common::CORE_DATABASE_INIT);

    if database_path.exists() {
        fs::remove_file(database_path)
            .expect("Failed to delete an old test core database file");
    }

    assert_eq!(database_path.exists(), false, "The database file should not exist yet");

    let core_database = CoreDatabase::new(
        database_path.to_owned(), schema_path.to_owned(), init_path.to_owned()
    )
        .expect("Failed to create the core database");

    assert_eq!(
        core_database.get_database_path(), 
        database_path, 
        "The database paths should be equal"
    );

    assert_eq!(database_path.exists(), true, "The database file should exist now");
    
    let foreign_keys_pragma = get_foreign_keys_pragma(&core_database)
        .expect("Failed to get the foreign_keys PRAGMA");

    assert_eq!(foreign_keys_pragma, 1, "The foreign_keys PRAGMA was not enabled");

    let secure_delete_pragma = get_secure_delete_pragma(&core_database)
        .expect("Failed to get the secure_delete PRAGMA");

    assert_eq!(secure_delete_pragma, 1, "The secure_delete PRAGMA was not enabled");
}

/// Verify the file properties check of the core database is successful
#[test]
fn check_core_database_file_properties() {
    common::setup_logging();

    let core_db_temp_file: NamedTempFile = common::create_core_database_temp_file();

    let core_database: CoreDatabase = common::create_test_core_database_with_defaults(core_db_temp_file.path().to_owned())
        .expect("Failed to create the core database");

    core_database.check_file_properties()
        .expect("An error occurred when executing the file properties check");

}

/// Verify that the schema of the core database can be created
#[test]
fn create_core_database_schema() {
    common::setup_logging();

    let core_db_temp_file: NamedTempFile = common::create_core_database_temp_file();

    let core_database: CoreDatabase = common::create_test_core_database_with_defaults(core_db_temp_file.path().to_owned())
        .expect("Failed to create the core database");

    assert_eq!(
        core_database
            .has_tables()
            .expect("Failed to check if the core database has tables"), 
        false,
        "The core database should not have any tables yet"
    );

    core_database.create_schema()
        .expect("Failed to create the core database's schema");

        assert_eq!(
        core_database
            .has_tables()
            .expect("Failed to check if the core database has tables"), 
        true,
        "The core database should have tables due to the schema being created"
    );
}

/// Verify that the core database can be initialized
#[test]
fn initialize_core_database() {
    common::setup_logging();

    let core_db_temp_file: NamedTempFile = common::create_core_database_temp_file();

    let core_database: CoreDatabase = common::create_test_core_database_with_defaults(core_db_temp_file.path().to_owned())
        .expect("Failed to create the core database");

    core_database.create_schema()
        .expect("Failed to create the core database's schema");

    match get_schema_version(&core_database) {
        Ok(_) => panic!("No data should exist yet"),
        Err(rusqlite::Error::QueryReturnedNoRows) => (), // This is expected at this point
        Err(e) => panic!("An unexpected error was returned: {}", e)
    }

    core_database.initialize()
        .expect("Failed to initialize the core database");

    let version = get_schema_version(&core_database)
        .expect("Failed to get the schema version");

    assert!(!version.is_empty(), "The version should not be an empty string")
}

/// Verify that the foreign_keys PRAGMA is enabled for the core database
#[test]
fn check_core_database_foreign_keys_pragma() {
    common::setup_logging();

    let core_db_temp_file: NamedTempFile = common::create_core_database_temp_file();

    let core_database: CoreDatabase = common::create_test_core_database_with_defaults(core_db_temp_file.path().to_owned())
        .expect("Failed to create the core database");

    // Disable so that it's confirmed to be enabled later by the method under test rather than 
    // enabled by a compile-time setting.
    disable_foreign_keys_pragma(&core_database)
        .expect("Failed to disable the foreign_keys PRAGMA");

    let foreign_keys_pragma = get_foreign_keys_pragma(&core_database)
        .expect("Failed to get the value for the foreign_keys PRAGMA");
    
    assert_eq!(
        foreign_keys_pragma,    
        0, 
        "The foreign_keys PRAGMA should be disabled"
    );

    match core_database.check_foreign_keys_pragma()
        .expect_err("Expected an error since the foreign_keys PRAGMA is disabled") {
            ImpulsePhmError::InvalidPragma(_) => (), // The expected error
            e => panic!("An error was returned, but it was the wrong type: {e}")
    };

    core_database.enable_foreign_keys_enforcement()
        .expect("Failed to enable to the foreign_keys PRAGMA");

    let foreign_keys_pragma = get_foreign_keys_pragma(&core_database)
        .expect("Failed to get the value for the foreign_keys PRAGMA");
    
    assert_eq!(
        foreign_keys_pragma,    
        1, 
        "The foreign_keys PRAGMA should be enabled"
    );

    core_database.check_foreign_keys_pragma()
        .expect("An error occurred when checking the foreign_keys PRAGMA")
}

/// Verify that the secure_delete PRAGMA is enabled for the core database
#[test]
fn check_core_database_secure_delete_pragma() {
    common::setup_logging();

    let core_db_temp_file: NamedTempFile = common::create_core_database_temp_file();

    let core_database: CoreDatabase = common::create_test_core_database_with_defaults(core_db_temp_file.path().to_owned())
        .expect("Failed to create the core database");

    // Disable so that it's confirmed to be enabled later by the method under test rather than 
    // enabled by a compile-time setting.
    disable_secure_delete_pragma(&core_database)
        .expect("Failed to disable the secure_delete PRAGMA");

    let secure_delete_pragma = get_secure_delete_pragma(&core_database)
        .expect("Failed to get the value for the secure_delete PRAGMA");
    
    assert_eq!(
        secure_delete_pragma,    
        0, 
        "The secure_delete PRAGMA should be disabled"
    );

    match core_database.check_secure_delete_pragma()
        .expect_err("Expected an error since the secure_delete PRAGMA is disabled") {
            ImpulsePhmError::InvalidPragma(_) => (), // The expected error
            e => panic!("An error was returned, but it was the wrong type: {e}")
    };

    core_database.enable_secure_delete()
        .expect("Failed to enable to the secure_delete PRAGMA");

    let secure_delete_pragma = get_secure_delete_pragma(&core_database)
        .expect("Failed to get the value for the secure_delete PRAGMA");
    
    assert_eq!(
        secure_delete_pragma,    
        1, 
        "The secure_delete PRAGMA should be enabled"
    );

    core_database.check_secure_delete_pragma()
        .expect("An error occurred when checking the secure_delete PRAGMA")
}

/// Verify that the expected error is returned when checking the tables of the core database
#[test]
fn check_core_database_fails_with_missing_tables() {
    common::setup_logging();

    let core_db_temp_file: NamedTempFile = common::create_core_database_temp_file();

    let core_database: CoreDatabase = common::create_test_core_database(
        core_db_temp_file.path().to_owned(), 
        Some(PathBuf::from(common::CORE_DATABASE_SCHEMA_WITH_MISSING_TABLES)),
        None
    )
        .expect("Failed to create the core database");

    core_database.create_schema()
        .expect("Failed to create the schema");

    core_database.initialize()
        .expect("Failed to initialize the database");

    match core_database.check_tables()
        .expect_err("Expected an error due to missing tables") {
            ImpulsePhmError::MissingTable(_) => (),
            e => panic!("An error was returned, but it was the wrong type:{e}")
        }
}

/// Verify that the expected error is returned when checking the schema version but it's invalid
#[test]
fn check_core_database_fails_with_invalid_schema_version() {
    common::setup_logging();

    let core_db_temp_file: NamedTempFile = common::create_core_database_temp_file();

    let core_database: CoreDatabase = common::create_test_core_database(
        core_db_temp_file.path().to_owned(), 
        None,
        Some(PathBuf::from(common::CORE_DATABASE_INIT_WITH_INVALID_SCHEMA_VERSION))
    )
        .expect("Failed to create the core database");

    core_database.create_schema()
        .expect("Failed to create the schema");

    core_database.initialize()
        .expect("Failed to initialize the database");

    match core_database.check_schema_version()
        .expect_err("Expected an error due to initializing with an invalid schema version") {
            ImpulsePhmError::InvalidSchemaVersion(_) => (),
            e => panic!("An error was returned, but it was the wrong type: {e}")
    }

}

/// Verify that a user database file can be created with the correct connection configuration
#[test]
fn create_user_database() {
    common::setup_logging();

    let database_path = Path::new("/tmp/create_user_database_test_file");
    let schema_path = Path::new(common::USER_DATABASE_SCHEMA);
    let init_path = Path::new(common::USER_DATABASE_INIT);

    if database_path.exists() {
        fs::remove_file(database_path)
            .expect("Failed to delete an old test user database file");
    }

    assert_eq!(database_path.exists(), false, "The database file should not exist yet");

    let user_database = UserDatabase::new(
        database_path.to_owned(), schema_path.to_owned(), init_path.to_owned()
    )
        .expect("Failed to create the user database");

    assert_eq!(
        user_database.get_database_path(), 
        database_path, 
        "The database paths should be equal"
    );
    
    assert_eq!(database_path.exists(), true, "The database file should exist now");

    let foreign_keys_pragma = get_foreign_keys_pragma(&user_database)
        .expect("Failed to get the foreign_keys PRAGMA");

    assert_eq!(foreign_keys_pragma, 1, "The foreign_keys PRAGMA was not enabled");

    let secure_delete_pragma = get_secure_delete_pragma(&user_database)
        .expect("Failed to get the secure_delete PRAGMA");

    assert_eq!(secure_delete_pragma, 1, "The secure_delete PRAGMA was not enabled");
}

/// Verify the file properties check of the user database is successful
#[test]
fn check_user_database_file_properties() {
    common::setup_logging();

    let user_db_temp_file: NamedTempFile = common::create_user_database_temp_file();

    let user_database: UserDatabase = common::create_test_user_database_with_defaults(user_db_temp_file.path().to_owned())
        .expect("Failed to create the user database");

    user_database.check_file_properties()
        .expect("An error occurred when executing the file properties check");

}

/// Verify that the schema of the user database can be created
#[test]
fn create_user_database_schema() {
    common::setup_logging();

    let user_db_temp_file: NamedTempFile = common::create_user_database_temp_file();

    let user_database: UserDatabase = common::create_test_user_database_with_defaults(user_db_temp_file.path().to_owned())
        .expect("Failed to create the user database");

    assert_eq!(
        user_database
            .has_tables()
            .expect("Failed to check if the user database has tables"), 
        false,
        "The user database should not have any tables yet"
    );

    user_database.create_schema()
        .expect("Failed to create the user database's schema");

        assert_eq!(
        user_database
            .has_tables()
            .expect("Failed to check if the user database has tables"), 
        true,
        "The user database should have tables due to the schema being created"
    );
}

/// Verify that the user database can be initialized
#[test]
fn initialize_user_database() {
    common::setup_logging();

    let user_db_temp_file: NamedTempFile = common::create_user_database_temp_file();

    let user_database: UserDatabase = common::create_test_user_database_with_defaults(user_db_temp_file.path().to_owned())
        .expect("Failed to create the user database");

    user_database.create_schema()
        .expect("Failed to create the user database's schema");

    match get_schema_version(&user_database) {
        Ok(_) => panic!("No data should exist, yet a row was returned"),
        Err(rusqlite::Error::QueryReturnedNoRows) => (), // This is expected at this point
        Err(e) => panic!("An unexpected error was returned: {}", e)
    }

    user_database.initialize()
        .expect("Failed to initialize the user database");

    let version = match get_schema_version(&user_database) {
        Ok(value) => value,
        Err(e) => panic!("An unexpected error was returned: {}", e)
    };

    assert!(!version.is_empty(), "The version should not be an empty string")
}

/// Verify that the foreign_keys PRAGMA is enabled for the user database
#[test]
fn check_user_database_foreign_keys_pragma() {
    common::setup_logging();

    let user_db_temp_file: NamedTempFile = common::create_user_database_temp_file();

    let user_database: UserDatabase = common::create_test_user_database_with_defaults(user_db_temp_file.path().to_owned())
        .expect("Failed to create the user database");

    // Disable so that it's confirmed to be enabled later by the method under test rather than 
    // enabled by a compile-time setting.
    disable_foreign_keys_pragma(&user_database)
        .expect("Failed to disable the foreign_keys PRAGMA");

    let foreign_keys_pragma = get_foreign_keys_pragma(&user_database)
        .expect("Failed to get the value for the foreign_keys PRAGMA");
    
    assert_eq!(
        foreign_keys_pragma,    
        0, 
        "The foreign_keys PRAGMA should be disabled"
    );

    match user_database.check_foreign_keys_pragma()
        .expect_err("Expected an error since the foreign_keys PRAGMA is disabled") {
            ImpulsePhmError::InvalidPragma(_) => (), // The expected error
            e => panic!("An error was returned, but it was the wrong type: {e}")
    };

    user_database.enable_foreign_keys_enforcement()
        .expect("Failed to enable to the foreign_keys PRAGMA");

    let foreign_keys_pragma = get_foreign_keys_pragma(&user_database)
        .expect("Failed to get the value for the foreign_keys PRAGMA");
    
    assert_eq!(
        foreign_keys_pragma,    
        1, 
        "The foreign_keys PRAGMA should be enabled"
    );

    user_database.check_foreign_keys_pragma()
        .expect("An error occurred when checking the foreign_keys PRAGMA")
}

/// Verify that the secure_delete PRAGMA is enabled for the user database
#[test]
fn check_user_database_secure_delete_pragma() {
    common::setup_logging();

    let user_db_temp_file: NamedTempFile = common::create_user_database_temp_file();

    let user_database: UserDatabase = common::create_test_user_database_with_defaults(user_db_temp_file.path().to_owned())
        .expect("Failed to create the user database");

    // Disable so that it's confirmed to be enabled later by the method under test rather than 
    // enabled by a compile-time setting.
    disable_secure_delete_pragma(&user_database)
        .expect("Failed to disable the secure_delete PRAGMA");

    let secure_delete_pragma = get_secure_delete_pragma(&user_database)
        .expect("Failed to get the value for the secure_delete PRAGMA");
    
    assert_eq!(
        secure_delete_pragma,    
        0, 
        "The secure_delete PRAGMA should be disabled"
    );

    match user_database.check_secure_delete_pragma()
        .expect_err("Expected an error since the secure_delete PRAGMA is disabled") {
            ImpulsePhmError::InvalidPragma(_) => (), // The expected error
            e => panic!("An error was returned, but it was the wrong type: {e}")
    };

    user_database.enable_secure_delete()
        .expect("Failed to enable to the secure_delete PRAGMA");

    let secure_delete_pragma = get_secure_delete_pragma(&user_database)
        .expect("Failed to get the value for the secure_delete PRAGMA");
    
    assert_eq!(
        secure_delete_pragma,    
        1, 
        "The secure_delete PRAGMA should be enabled"
    );

    user_database.check_secure_delete_pragma()
        .expect("An error occurred when checking the secure_delete PRAGMA")
}

/// Verify that the expected error is returned when checking the tables of the user database
#[test]
fn check_user_database_fails_with_missing_tables() {
    common::setup_logging();

    let user_db_temp_file: NamedTempFile = common::create_user_database_temp_file();

    let user_database: UserDatabase = common::create_test_user_database(
        user_db_temp_file.path().to_owned(), 
        Some(PathBuf::from(common::USER_DATABASE_SCHEMA_WITH_MISSING_TABLES)),
        None
    )
        .expect("Failed to create the user database");

    user_database.create_schema()
        .expect("Failed to create the schema");

    user_database.initialize()
        .expect("Failed to initialize the database");

    match user_database.check_tables()
        .expect_err("Expected an error due to missing tables") {
            ImpulsePhmError::MissingTable(_) => (),
            e => panic!("An error was returned, but it was the wrong type: {e}")
        }
}

/// Verify that the expected error is returned when checking the schema version but it's invalid
#[test]
fn check_user_database_fails_with_invalid_schema_version() {
    common::setup_logging();

    let user_db_temp_file: NamedTempFile = common::create_user_database_temp_file();

    let user_database: UserDatabase = common::create_test_user_database(
        user_db_temp_file.path().to_owned(), 
        None,
        Some(PathBuf::from(common::USER_DATABASE_INIT_WITH_INVALID_SCHEMA_VERSION))
    )
        .expect("Failed to create the user database");

    user_database.create_schema()
        .expect("Failed to create the schema");

    user_database.initialize()
        .expect("Failed to initialize the database");

    match user_database.check_schema_version()
        .expect_err("Expected an error due to initializing with an invalid schema version") {
            ImpulsePhmError::InvalidSchemaVersion(_) => (),
            e => panic!("An error was returned, but it was the wrong type: {e}")
    }

}


// Utility Functions

/// Get the schema version used in a database
fn get_schema_version<T: Query>(database: &T) -> Result<String, rusqlite::Error> {
    database
        .get_connection()
        .query_one(
        "SELECT version FROM database_release ORDER BY created_at DESC LIMIT 1;",
        [],
        |row| row.get(0) 
        )
}

/// Disable the foreign_keys PRAGMA
/// 
/// This function was created for tests that need to verify the foreign_keys PRAGMA gets enabled 
/// by the method under test rather than it being enabled by a compile-time setting.
fn disable_foreign_keys_pragma<T: Query>(database: &T) -> Result<(), rusqlite::Error> {
        database
            .get_connection()
            .pragma_update(None, "foreign_keys", 0)
}

/// Disable the secure_delete PRAGMA
/// 
/// This function was created for tests that need to verify the secure_delete PRAGMA gets enabled 
/// by the method under test rather than it being enabled by a compile-time setting.
fn disable_secure_delete_pragma<T: Query>(database: &T) -> Result<(), rusqlite::Error> {
        database
            .get_connection()
            .pragma_update(None, "secure_delete", 0)
}

/// Get the value of the foreign_keys PRAGMA
fn get_foreign_keys_pragma<T: Query>(database: &T) -> Result<u8, rusqlite::Error> {
    let foreign_keys_pragma: u8 = database
        .get_connection()
        .query_one(
            "SELECT * FROM pragma_foreign_keys;",
            [],
            |row| row.get(0)
        )?;

    Ok(foreign_keys_pragma)
}

/// Get the value of the secure_delete PRAGMA
fn get_secure_delete_pragma<T: Query>(database: &T) -> Result<u8, rusqlite::Error> {
    let secure_delete_pragma: u8 = database
        .get_connection()
        .query_one(
            "SELECT * FROM pragma_secure_delete;",
            [],
            |row| row.get(0)
        )?;

    Ok(secure_delete_pragma)
}
