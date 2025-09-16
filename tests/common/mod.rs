//! Common constants and operations for the integration tests

use std::{path::{Path, PathBuf}, sync::OnceLock};

use tempfile::NamedTempFile;

use impulse_phm::{database::{core::CoreDatabase, user::UserDatabase}, error::ImpulsePhmError};


pub const PROJECT_ROOT: &str = env!("CARGO_MANIFEST_DIR");

pub const CORE_DATABASE_SCHEMA: &str = "tests/resources/setup/core_schema.sql";
pub const CORE_DATABASE_INIT: &str = "tests/resources/setup/core_init.sql";
pub const CORE_DATABASE_INITIAL_SCHEMA_VERSION: &str = "0.1.0";

pub const USER_DATABASE_SCHEMA: &str = "tests/resources/setup/user_schema.sql";
pub const USER_DATABASE_SCHEMA_WITH_MISSING_TABLES: &str = "tests/resources/setup/user_schema_with_missing_tables.sql";
pub const USER_DATABASE_INIT: &str = "tests/resources/setup/user_init.sql";
pub const USER_DATABASE_INIT_WITH_INVALID_VERSION: &str = "tests/resources/setup/user_init_with_invalid_version.sql";
// The file extension with the leading period --primarily used by temporary files in the tests
pub const USER_DATABASE_FILE_SUFFIX: &str = ".impulse";

/// Controlling initialization because a panic occurs if the logger is initialized more than once.
pub static LOGGER: OnceLock<()> = OnceLock::new();


/// Setup logging for the integration tests by initializing the logger
/// 
/// Not required for the tests to use but, ideally, all of the tests should call this for logging 
/// support to help with any needed debugging.
/// 
/// Additionally, there's no need to put an explicit filter level here because that would prevent  
/// the "RUST_LOG" environment variable from being able to override it.
pub fn setup_logging() {
    LOGGER.get_or_init(
    || {
        env_logger::builder()
            .is_test(true)
            .try_init()
            .expect("Failed to initialize the logger for the integration tests")
    }
    );
}

/// Create a temp file to be later turned into the core database of a test
pub fn create_core_database_temp_file() -> NamedTempFile {
    NamedTempFile::new()
        .expect("Failed to create the temp file for the core database")
}

/// Create a test version of the core database
pub fn create_test_core_database(database_path: &Path) -> Result<CoreDatabase, ImpulsePhmError> {
    let mut schema_path = PathBuf::from(PROJECT_ROOT);
    schema_path.push(CORE_DATABASE_SCHEMA);
    
    let mut init_path = PathBuf::from(PROJECT_ROOT);
    init_path.push(CORE_DATABASE_INIT);
    
    let test_core_database = CoreDatabase::new(
        database_path, &schema_path, &init_path
    )?;


    Ok(test_core_database)
}

/// Create a test version of the user database
pub fn create_test_user_database(database_path: &Path, schema_path: Option<&Path>, 
    init_path: Option<&Path>) -> Result<UserDatabase, ImpulsePhmError> {
    let schema_path = schema_path.unwrap_or(Path::new(USER_DATABASE_SCHEMA));
    let init_path = init_path.unwrap_or(Path::new(USER_DATABASE_INIT));
    
    let test_user_database = UserDatabase::new(
        database_path, schema_path, init_path
    )?;

    Ok(test_user_database)
}

/// Create a test version of the user database with default values for the schema and init paths
pub fn create_test_user_database_with_defaults(database_path: &Path) 
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
