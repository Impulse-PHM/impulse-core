//! Common constants and operations for the integration tests

use std::{path::PathBuf, sync::OnceLock};

use flexi_logger::{Logger, WriteMode};

use tempfile::NamedTempFile;

use impulse_phm::{database::{core::CoreDatabase, user::UserDatabase}, error::ImpulsePhmError};


pub const CORE_DATABASE_SCHEMA: &str = "tests/resources/setup/core_schema.sql";
pub const CORE_DATABASE_SCHEMA_WITH_MISSING_TABLES: &str = "tests/resources/setup/core_schema_with_missing_tables.sql";
pub const CORE_DATABASE_INIT: &str = "tests/resources/setup/core_init.sql";
pub const CORE_DATABASE_INIT_WITH_INVALID_SCHEMA_VERSION: &str = "tests/resources/setup/core_init_with_invalid_schema_version.sql";

pub const USER_DATABASE_SCHEMA: &str = "tests/resources/setup/user_schema.sql";
pub const USER_DATABASE_SCHEMA_WITH_MISSING_TABLES: &str = "tests/resources/setup/user_schema_with_missing_tables.sql";
pub const USER_DATABASE_INIT: &str = "tests/resources/setup/user_init.sql";
pub const USER_DATABASE_INIT_WITH_INVALID_SCHEMA_VERSION: &str = "tests/resources/setup/user_init_with_invalid_schema_version.sql";
// The file extension with the leading period --primarily used by temporary files in the tests
pub const USER_DATABASE_FILE_SUFFIX: &str = ".impulse";

/// Controlling initialization because a panic occurs if the logger is initialized more than once.
pub static TEST_LOGGER: OnceLock<()> = OnceLock::new();


/// Setup logging for the integration tests by initializing the logger
/// 
/// Not required for the tests to use but, ideally, all of the tests should call this for logging 
/// support to help with any needed debugging.
/// 
/// Additionally, there's no need to put an explicit filter level here because that would prevent  
/// the "RUST_LOG" environment variable from being able to override it.
pub fn setup_logging() {
    TEST_LOGGER.get_or_init(
    || {
        Logger::try_with_env_or_str("error")
            .expect("Failed to try with env")
            // Allows cargo test to capture log output and display it only for failing tests
            .write_mode(WriteMode::SupportCapture)
            .format(flexi_logger::colored_detailed_format)
            .start()
            .expect("Failed to start");
    }
    );
}

/// Create a temp file to be later turned into the core database of a test
pub fn create_core_database_temp_file() -> NamedTempFile {
    NamedTempFile::new()
        .expect("Failed to create the temp file for the core database")
}

/// Create a test version of the core database
pub fn create_test_core_database(database_path: PathBuf, schema_path: Option<PathBuf>, 
    init_path: Option<PathBuf>) -> Result<CoreDatabase, ImpulsePhmError> {
    let schema_path = schema_path.unwrap_or(PathBuf::from(CORE_DATABASE_SCHEMA));
    
    let init_path = init_path.unwrap_or(PathBuf::from(CORE_DATABASE_INIT));
    
    let test_core_database = CoreDatabase::new(
        database_path, schema_path, init_path
    )?;


    Ok(test_core_database)
}

/// Create a test version of the core database with default values for the schema and init paths
pub fn create_test_core_database_with_defaults(database_path: PathBuf) 
    -> Result<CoreDatabase, ImpulsePhmError> {
    create_test_core_database(database_path, None, None)
}

/// Create a test version of the user database
pub fn create_test_user_database(database_path: PathBuf, schema_path: Option<PathBuf>, 
    init_path: Option<PathBuf>) -> Result<UserDatabase, ImpulsePhmError> {
    let schema_path = schema_path.unwrap_or(PathBuf::from(USER_DATABASE_SCHEMA));
    let init_path = init_path.unwrap_or(PathBuf::from(USER_DATABASE_INIT));
    
    let test_user_database = UserDatabase::new(
        database_path, schema_path, init_path
    )?;

    Ok(test_user_database)
}

/// Create a test version of the user database with default values for the schema and init paths
pub fn create_test_user_database_with_defaults(database_path: PathBuf) 
    -> Result<UserDatabase, ImpulsePhmError> {
    create_test_user_database(database_path, None, None)
}

/// Create a temp file to be later turned into the user database of a test
/// 
/// Additionally, it will be created to meet all rules of being the application file format.
pub fn create_user_database_temp_file() -> NamedTempFile {
    NamedTempFile::with_suffix(USER_DATABASE_FILE_SUFFIX)
        .expect("Failed to create the temp file for the user database")
}
