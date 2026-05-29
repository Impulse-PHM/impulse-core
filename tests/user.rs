//! The integration tests for [`impulse_core::model::user`]

mod common;

use tempfile::NamedTempFile;

use impulse_core::{
    ManageUser, database::{core::CoreDatabase, user::UserDatabase}, environment, model::{
        ImpulseCore, user::{DEFAULT_CREATED_AT, DEFAULT_ID, User, UserBuilder}
    }
};


/// Verify that a user can be saved
#[test]
fn save_user() {
    common::setup_logging();

    let core_db_temp_file: NamedTempFile = common::create_core_database_temp_file();
    let user_db_temp_file: NamedTempFile = common::create_user_database_temp_file();

    let core_database: CoreDatabase = common::create_test_core_database_with_defaults(core_db_temp_file.path().to_owned())
        .expect("Failed to create the core database");

    let user_database: UserDatabase = common::create_test_user_database_with_defaults(user_db_temp_file.path().to_owned())
        .expect("Failed to create the user database");

    environment::setup_user_database(&user_database)
        .expect("Failed to setup the user database");
    
    let impulse_core = ImpulseCore::new(core_database, user_database);
    
    let user = UserBuilder::new()
        .with_first_name("Tony")
        .with_last_name("Stark")
        .with_birth_year(1970)
        .with_birth_month(5)
        .with_birth_day(29)
        .with_is_primary(true)
        .build()
        .expect("Failed to build a user");

    // Expecting default 0 values for the id and created_at fields
    assert_eq!(
        user.id, DEFAULT_ID, 
        "The ID should be 0 since it has not been assigned in the database yet"
    );
    
    assert_eq!(
        user.created_at, DEFAULT_CREATED_AT, 
        "created_at should be 0 since it has not been assigned in the database yet"
    );

    let saved_user = impulse_core
        .save_user(&user)
        .expect("Failed to save a new user in the database");

    assert_ne!(
        saved_user, user, 
        "Should not be equal due to the id and created_at fields having default values."
    );

    let mut expected_user = user;
    expected_user.id = saved_user.id;
    expected_user.created_at = saved_user.created_at;

    assert_eq!(
        saved_user, expected_user,
        "Both should be equal now since the database assigned real values for the internal id and \
        created_at columns."
    );

}

/// Verify that an error is returned when an invalid birth year is specified
#[test]
fn save_user_fails_with_invalid_birth_year() {
    common::setup_logging();

    let user_db_temp_file: NamedTempFile = common::create_user_database_temp_file();

    let user_database: UserDatabase = common::create_test_user_database_with_defaults(user_db_temp_file.path().to_owned())
        .expect("Failed to create the user database");

    environment::setup_user_database(&user_database)
        .expect("Failed to setup the user database");
    
    UserBuilder::new()
        .with_first_name("Tony")
        .with_last_name("Stark")
        // The year used would put the user's age above the maximum age
        .with_birth_year(1900)
        .with_birth_month(5)
        .with_birth_day(29)
        .with_is_primary(true)
        .build()
        .expect_err("Expected an error since an invalid birth year was used");
}

/// Verify that an error is returned when an invalid birth month is specified
#[test]
fn save_user_fails_with_invalid_birth_month() {
    common::setup_logging();

    let user_db_temp_file: NamedTempFile = common::create_user_database_temp_file();

    let user_database: UserDatabase = common::create_test_user_database_with_defaults(user_db_temp_file.path().to_owned())
        .expect("Failed to create the user database");

    environment::setup_user_database(&user_database)
        .expect("Failed to setup the user database");
    
    UserBuilder::new()
        .with_first_name("Tony")
        .with_last_name("Stark")
        .with_birth_year(1970)
        .with_birth_month(13)
        .with_birth_day(29)
        .with_is_primary(true)
        .build()
        .expect_err("Expected an error since an invalid birth month was used");
}

/// Verify that an error is returned when an invalid day of birth is specified
#[test]
fn save_user_fails_with_invalid_day_of_birth() {
    common::setup_logging();

    let user_db_temp_file: NamedTempFile = common::create_user_database_temp_file();

    let user_database: UserDatabase = common::create_test_user_database_with_defaults(user_db_temp_file.path().to_owned())
        .expect("Failed to create the user database");

    environment::setup_user_database(&user_database)
        .expect("Failed to setup the user database");
    
    UserBuilder::new()
        .with_first_name("Tony")
        .with_last_name("Stark")
        .with_birth_year(1970)
        .with_birth_month(5)
        .with_birth_day(32)
        .with_is_primary(true)
        .build()
        .expect_err("Expected an error since an invalid day of birth was used");
}

/// Verify that an error is returned when trying to save a primary user when one already exists
#[test]
fn save_user_fails_with_existing_primary_user() {
    common::setup_logging();

    let core_db_temp_file: NamedTempFile = common::create_core_database_temp_file();
    let user_db_temp_file: NamedTempFile = common::create_user_database_temp_file();

    let core_database: CoreDatabase = common::create_test_core_database_with_defaults(core_db_temp_file.path().to_owned())
        .expect("Failed to create the core database");

    let user_database: UserDatabase = common::create_test_user_database_with_defaults(user_db_temp_file.path().to_owned())
        .expect("Failed to create the user database");

    environment::setup_user_database(&user_database)
        .expect("Failed to setup the user database");
    
    let impulse_core = ImpulseCore::new(core_database, user_database);
    
    let user = UserBuilder::new()
        .with_first_name("Tony")
        .with_last_name("Stark")
        .with_birth_year(1970)
        .with_birth_month(5)
        .with_birth_day(29)
        .with_is_primary(true)
        .build()
        .expect("Failed to build a user");

    impulse_core
        .save_user(&user)
        .expect("Failed to save a new user in the database");

    let invalid_user: User = UserBuilder::new()
        .with_first_name("Howard")
        .with_last_name("Stark")
        .with_birth_year(1917)
        .with_birth_month(8)
        .with_birth_day(15)
        .with_is_primary(true)
        .build()
        .expect("Failed to build a user");

    impulse_core
        .save_user(&invalid_user)
        .expect_err("Expected an error since a primary user already exists");

}

/// Verify a primary user can be extracted
#[test]
fn get_primary_user() {
    common::setup_logging();

    let core_db_temp_file: NamedTempFile = common::create_core_database_temp_file();
    let user_db_temp_file: NamedTempFile = common::create_user_database_temp_file();

    let core_database: CoreDatabase = common::create_test_core_database_with_defaults(core_db_temp_file.path().to_owned())
        .expect("Failed to create the core database");

    let user_database: UserDatabase = common::create_test_user_database_with_defaults(user_db_temp_file.path().to_owned())
        .expect("Failed to create the user database");

    environment::setup_user_database(&user_database)
        .expect("Failed to setup the user database");
    
    let impulse_core = ImpulseCore::new(core_database, user_database);

    let primary_user = impulse_core
        .get_primary_user()
        .expect("Failed to get the primary user");

    assert!(primary_user.is_none(), "There should not be a primary user in the database yet");

    let user = UserBuilder::new()
        .with_first_name("Tony")
        .with_last_name("Stark")
        .with_birth_year(1970)
        .with_birth_month(5)
        .with_birth_day(29)
        .with_is_primary(true)
        .build()
        .expect("Failed to build a user");

    impulse_core
        .save_user(&user)
        .expect("Failed to save the user in the database");

    let primary_user: User = impulse_core
        .get_primary_user()
        .expect("Failed to get the primary user")
        .expect("The query was successful, but no primary user was returned.");

    assert_eq!(user.first_name, primary_user.first_name);
    assert_eq!(user.last_name, primary_user.last_name);
    assert_eq!(user.birth_month, primary_user.birth_month);
    assert_eq!(user.birth_day, primary_user.birth_day);
    assert_eq!(user.birth_year, primary_user.birth_year);
    assert_eq!(user.is_primary, primary_user.is_primary);
}

/// Verify a user can be updated
#[test]
fn update_user() {
    common::setup_logging();

    let core_db_temp_file: NamedTempFile = common::create_core_database_temp_file();
    let user_db_temp_file: NamedTempFile = common::create_user_database_temp_file();

    let core_database: CoreDatabase = common::create_test_core_database_with_defaults(core_db_temp_file.path().to_owned())
        .expect("Failed to create the core database");

    let user_database: UserDatabase = common::create_test_user_database_with_defaults(user_db_temp_file.path().to_owned())
        .expect("Failed to create the user database");

    environment::setup_user_database(&user_database)
        .expect("Failed to setup the user database");
    
    let impulse_core = ImpulseCore::new(core_database, user_database);

    let user = UserBuilder::new()
        .with_first_name("Tony")
        .with_last_name("Stark")
        .with_birth_year(1970)
        .with_birth_month(5)
        .with_birth_day(29)
        .with_is_primary(true)
        .build()
        .expect("Failed to build a user");

    let user = impulse_core
        .save_user(&user)
        .expect("Failed to save the user in the database");

    let mut updated_user = user.clone();
    updated_user.birth_year = 1971;
    updated_user.birth_month = 6;
    updated_user.birth_day = 30;

    let updated_user = impulse_core
        .update_user(&updated_user)
        .expect("Failed to update the user in the database");

    let primary_user: User = impulse_core
        .get_primary_user()
        .expect("Failed to get the primary user")
        .expect("The query was successful, but no primary user was returned.");

    assert_eq!(primary_user.id, updated_user.id);
    assert_eq!(primary_user.first_name, updated_user.first_name);
    assert_eq!(primary_user.last_name, updated_user.last_name);
    assert_eq!(primary_user.birth_month, updated_user.birth_month);
    assert_eq!(primary_user.birth_day, updated_user.birth_day);
    assert_eq!(primary_user.birth_year, updated_user.birth_year);
    assert_eq!(primary_user.is_primary, updated_user.is_primary);
    assert_eq!(primary_user.created_at, updated_user.created_at);

}

/// Verify that updating a user fails when the ID is the default value as that represents a user 
/// that has not yet been saved in the database for this project.
#[test]
fn update_user_fails_with_default_id() {
    common::setup_logging();

    let core_db_temp_file: NamedTempFile = common::create_core_database_temp_file();
    let user_db_temp_file: NamedTempFile = common::create_user_database_temp_file();

    let core_database: CoreDatabase = common::create_test_core_database_with_defaults(core_db_temp_file.path().to_owned())
        .expect("Failed to create the core database");

    let user_database: UserDatabase = common::create_test_user_database_with_defaults(user_db_temp_file.path().to_owned())
        .expect("Failed to create the user database");

    environment::setup_user_database(&user_database)
        .expect("Failed to setup the user database");
    
    let impulse_core = ImpulseCore::new(core_database, user_database);

    let user = UserBuilder::new()
        .with_first_name("Tony")
        .with_last_name("Stark")
        .with_birth_year(1970)
        .with_birth_month(5)
        .with_birth_day(29)
        .with_is_primary(true)
        .build()
        .expect("Failed to build a user");

    // Intentionally, do not save the user into the database so it uses the default ID.

    let mut updated_user = user.clone();
    updated_user.created_at = 1;
    
    impulse_core.update_user(&updated_user)
        .expect_err("Updating should have failed since the default ID was used");

}

/// Verify that updating a user fails when the "created at" value is the default value as that 
/// represents a user that has not yet been saved in the database for this project.
#[test]
fn update_user_fails_with_default_created_at() {
    common::setup_logging();

    let core_db_temp_file: NamedTempFile = common::create_core_database_temp_file();
    let user_db_temp_file: NamedTempFile = common::create_user_database_temp_file();

    let core_database: CoreDatabase = common::create_test_core_database_with_defaults(core_db_temp_file.path().to_owned())
        .expect("Failed to create the core database");

    let user_database: UserDatabase = common::create_test_user_database_with_defaults(user_db_temp_file.path().to_owned())
        .expect("Failed to create the user database");

    environment::setup_user_database(&user_database)
        .expect("Failed to setup the user database");
    
    let impulse_core = ImpulseCore::new(core_database, user_database);

    let user = UserBuilder::new()
        .with_first_name("Tony")
        .with_last_name("Stark")
        .with_birth_year(1970)
        .with_birth_month(5)
        .with_birth_day(29)
        .with_is_primary(true)
        .build()
        .expect("Failed to build a user");

    // Intentionally, do not save the user into the database so it uses the default "created at" 
    // value.

    let mut updated_user = user.clone();
    updated_user.id = 1;
    
    impulse_core.update_user(&updated_user)
        .expect_err("Updating should have failed since the default \"created at\" value was used");

}
