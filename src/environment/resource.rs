//! Simplifies the management of resource files that are loaded at runtime
//! 
//! Specifically, this module contains operations that provide the absolute path to all 
//! resource files used in this project. An absolute path will be returned whether 
//! an actual file exists or not because some operations need to know what a resource 
//! file's path "should be" even before it gets created. 
//! 
//! The primary benefit of this module is that calling code will not have to worry about all of the 
//! specialized directories that certain files go in --which can vary considerably based on the 
//! operating system as well as whether debug mode or release mode was used to build the project.

use std::{ffi::OsStr, fs, io, path::PathBuf};

use crate::database::user::{USER_DATABASE_DEFAULT_FILE_NAME, USER_DATABASE_FILE_EXTENSION};


/// The name of the resources directory
pub const RESOURCES_DIRECTORY_NAME: &str = "resources";

/// The name of the resource setup directory
pub const RESOURCES_SETUP_DIRECTORY_NAME: &str = "setup";

/// The base name of the log file (I split the file name to match what flexi_logger expects)
pub const LOG_BASE_NAME: &str = "impulse";

/// The suffix, or extension without the leading period, of the log file.
pub const LOG_SUFFIX: &str = "log";

/// The core database (requires write access)
pub const CORE_DATABASE_FILE_NAME: &str = "core.db";

/// Contains the SQL to create the core database's schema (read-only file)
pub const CORE_DATABASE_SCHEMA_FILE_NAME: &str = "core_schema.sql";

/// Contains the SQL to insert initial values into the core database (read-only file)
pub const CORE_DATABASE_INIT_FILE_NAME: &str = "core_init.sql";

/// Contains the SQL to create the user database's schema (read-only file)
pub const USER_DATABASE_SCHEMA_FILE_NAME: &str = "user_schema.sql";

/// Contains the SQL to insert initial values into the user database (read-only file)
pub const USER_DATABASE_INIT_FILE_NAME: &str = "user_init.sql";


/// Get the absolute path to the resources directory
pub fn get_resources_directory() -> PathBuf {
    if cfg!(debug_assertions) {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("debug")
            .join(RESOURCES_DIRECTORY_NAME)
    }
    else {
        todo!("Implement release mode logic later")
    }
}
/// Get the absolute path to the directory that contains the databases
pub fn get_database_directory() -> PathBuf {
    // Currently, the databases should be stored in the top-level resources directory.
    get_resources_directory()
}

/// Get the absolute value of the resource's setup directory
pub fn get_resources_setup_directory() -> PathBuf {
    PathBuf::from(get_resources_directory())
        .join(RESOURCES_SETUP_DIRECTORY_NAME)
}

/// Get the absolute path that contains the log file
pub fn get_log_directory() -> PathBuf {
    if cfg!(debug_assertions) {
        // In debug mode, logs should be placed in the resources directory
        get_resources_directory()
    }
    else {
        // In release mode, the logs should generally be placed outside of the resources dir
        todo!("Implement release mode logic later as some platforms have specific dirs for logs")
    }
}

/// Get the absolute path to the log file
pub fn get_log_file() -> PathBuf {
    // No debug vs release mode logic is needed here since the directory is determined
    // by another function.
    let log_file_name = format!("{}.{}", LOG_BASE_NAME, LOG_SUFFIX);
    PathBuf::from(get_log_directory())
        .join(log_file_name)
}

/// Get the absolute path to the core database
pub fn get_core_database() -> PathBuf {
    if cfg!(debug_assertions) {
        PathBuf::from(get_resources_directory())
            .join(CORE_DATABASE_FILE_NAME)
    }
    else {
        todo!("Implement release mode logic later")
    }
}

/// Get the absolute path to the core database's schema file
pub fn get_core_database_schema() -> PathBuf {
    // No debug vs release logic as this should always be in the setup directory
    PathBuf::from(get_resources_setup_directory())
        .join(CORE_DATABASE_SCHEMA_FILE_NAME)
}

/// Get the absolute path to the core database's init file
pub fn get_core_database_init() -> PathBuf {
    // No debug vs release logic as this should always be in the setup directory
    PathBuf::from(get_resources_setup_directory())
        .join(CORE_DATABASE_INIT_FILE_NAME)
}

/// Get the absolute path to the user database
/// 
/// # Returns:
/// A path to the first file found with the proper file extension (regardless of the file's name).
/// Otherwise, returns a path that uses a default file name with the proper file extension.
/// 
/// # Errors:
/// Returns an [`io::Error`] if there's a problem with reading the directory that contains the 
/// databases.
pub fn get_user_database() -> Result<PathBuf, io::Error> {
    let database_directory = get_database_directory();
    let mut existing_user_database_path: Option<PathBuf> = None;

    if cfg!(debug_assertions) {
        // Extract the entries iterator before the for-loop to add logging and more clear 
        // error-handling.
        let entries = match fs::read_dir(&database_directory) {
            Ok(entries) => entries,
            Err(e) => {
                log::error!("Failed to read the directory that contains the user database: {}", e);
                return Err(e);
            }
        };

        for entry_result in entries {
            let entry = match entry_result {
                Ok(entry) => entry,
                Err(e) => {
                    log::error!("Failed to read a specific entry: {}", e);
                    return Err(e);
                }
            };

            let entry_type = match entry.file_type() {
                Ok(entry_type) => entry_type,
                Err(e) => {
                    log::error!("Failed to get the entry type for an entry: {}", e);
                    return Err(e);
                }
            };

            if !entry_type.is_file() {
                continue;
            }

            let path = entry.path();
            if let Some(extension) = path.extension() {
                if extension.to_ascii_lowercase() == OsStr::new(USER_DATABASE_FILE_EXTENSION) {
                    existing_user_database_path = Some(path);
                    break;
                }
            } 
        }

        match existing_user_database_path {
            Some(path) => return Ok(path),
            None => {
                // Create a path that uses a default file name
                let new_user_database_path = Ok(PathBuf::from(&database_directory)
                    .join(format!("{}.{}", USER_DATABASE_DEFAULT_FILE_NAME, 
                    USER_DATABASE_FILE_EXTENSION)));
                
                return new_user_database_path;
            },
        }
    }
    else {
        todo!("Implement release mode logic later")
    }
}

/// Get the absolute path to the user database's schema file
pub fn get_user_database_schema() -> PathBuf {
    // No debug vs release logic as this should always be in the setup directory
    PathBuf::from(get_resources_setup_directory())
        .join(USER_DATABASE_SCHEMA_FILE_NAME)
}

/// Get the absolute path to the user database's init file
pub fn get_user_database_init() -> PathBuf {
    // No debug vs release logic as this should always be in the setup directory
    PathBuf::from(get_resources_setup_directory())
        .join(USER_DATABASE_INIT_FILE_NAME)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_resources_directory() {
        let expected_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("debug")
            .join(RESOURCES_DIRECTORY_NAME);

        let resources_directory = super::get_resources_directory();

        assert_eq!(resources_directory.is_absolute(), true, "Should be an absolute path");
        assert_eq!(resources_directory, expected_path);
    }

    #[test]
    fn get_resources_setup_directory() {
        let expected_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("debug")
            .join(RESOURCES_DIRECTORY_NAME)
            .join(RESOURCES_SETUP_DIRECTORY_NAME);

        let resources_setup_directory = super::get_resources_setup_directory();

        assert_eq!(resources_setup_directory.is_absolute(), true, "Should be an absolute path");
        assert_eq!(resources_setup_directory, expected_path);
    }

    #[test]
    fn get_log_directory() {
        let expected_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("debug")
            .join(RESOURCES_DIRECTORY_NAME);

        let log_directory = super::get_log_directory();

        assert_eq!(log_directory.is_absolute(), true, "Should be an absolute path");
        assert_eq!(log_directory, expected_path);
    }

    #[test]
    fn get_log_file() {
        let expected_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("debug")
            .join(RESOURCES_DIRECTORY_NAME)
            .join(format!("{}.{}", LOG_BASE_NAME, LOG_SUFFIX));

        let log_file = super::get_log_file();

        assert_eq!(log_file.is_absolute(), true, "Should be an absolute path");
        assert_eq!(log_file, expected_path);
    }

    #[test]
    fn get_core_database() {
        let expected_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("debug")
            .join(RESOURCES_DIRECTORY_NAME)
            .join(CORE_DATABASE_FILE_NAME);

        let core_database_path = super::get_core_database();
        
        assert_eq!(core_database_path.is_absolute(), true, "Should be an absolute path");
        assert_eq!(core_database_path, expected_path);
    }    

    #[test]
    fn get_core_database_schema() {
        let expected_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("debug")
            .join(RESOURCES_DIRECTORY_NAME)
            .join(RESOURCES_SETUP_DIRECTORY_NAME)
            .join(CORE_DATABASE_SCHEMA_FILE_NAME);

        let core_database_schema_path = super::get_core_database_schema();
        
        assert_eq!(core_database_schema_path.is_absolute(), true, "Should be an absolute path");
        assert_eq!(core_database_schema_path, expected_path);
    }

    #[test]
    fn get_core_database_init() {
        let expected_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("debug")
            .join(RESOURCES_DIRECTORY_NAME)
            .join(RESOURCES_SETUP_DIRECTORY_NAME)
            .join(CORE_DATABASE_INIT_FILE_NAME);

        let core_database_init_path = super::get_core_database_init();
        
        assert_eq!(core_database_init_path.is_absolute(), true, "Should be an absolute path");
        assert_eq!(core_database_init_path, expected_path);
    }

    #[test]
    fn get_user_database() {
        let expected_file_name = format!(
            "{}.{}", USER_DATABASE_DEFAULT_FILE_NAME, USER_DATABASE_FILE_EXTENSION
        );

        let expected_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("debug")
            .join(RESOURCES_DIRECTORY_NAME)
            .join(expected_file_name);

        let user_database_path = super::get_user_database()
            .expect("Failed to get the user database");
        
        assert_eq!(user_database_path.is_absolute(), true, "Should be an absolute path");
        assert_eq!(user_database_path, expected_path);
    }

    #[test]
    fn get_user_database_schema() {
        let expected_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("debug")
            .join(RESOURCES_DIRECTORY_NAME)
            .join(RESOURCES_SETUP_DIRECTORY_NAME)
            .join(USER_DATABASE_SCHEMA_FILE_NAME);

        let user_database_schema_path = super::get_user_database_schema();
        
        assert_eq!(user_database_schema_path.is_absolute(), true, "Should be an absolute path");
        assert_eq!(user_database_schema_path, expected_path);
    }

    #[test]
    fn get_user_database_init() {
        let expected_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("debug")
            .join(RESOURCES_DIRECTORY_NAME)
            .join(RESOURCES_SETUP_DIRECTORY_NAME)
            .join(USER_DATABASE_INIT_FILE_NAME);

        let user_database_init_path = super::get_user_database_init();
        
        assert_eq!(user_database_init_path.is_absolute(), true, "Should be an absolute path");
        assert_eq!(user_database_init_path, expected_path);
    }
}

