//! The integration tests for [`impulse_phm::model::user`]

mod common;

use tempfile::NamedTempFile;

use impulse_core::{
    database::{core::CoreDatabase, user::UserDatabase}, 
    environment,
    model::{
        user::{User, UserBuilder, DEFAULT_USER_CREATED_AT, DEFAULT_USER_ID}, 
        ImpulseCore
    }
};


/// Verify that a user can be created
#[test]
fn create_user() {
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
    
    let user: User = UserBuilder::new()
        .with_first_name("Tony")
        .with_last_name("Stark")
        .with_birth_year(1970)
        .with_birth_month(5)
        .with_birth_day(29)
        .build()
        .expect("Failed to build a user");

    // Expecting default 0 values for the id and created_at fields
    assert_eq!(
        user.id, DEFAULT_USER_ID, 
        "The ID should be 0 since it has not been assigned in the database yet"
    );
    
    assert_eq!(
        user.created_at, DEFAULT_USER_CREATED_AT, 
        "created_at should be 0 since it has not been assigned in the database yet"
    );

    let created_user = impulse_core
        .with_user()
        .create_user(&user)
        .expect("Failed to create a new user in the database");

    assert_ne!(
        created_user, user, 
        "Should not be equal due to the id and created_at fields having default values."
    );

    let mut expected_user = user;
    expected_user.id = created_user.id;
    expected_user.created_at = created_user.created_at;

    assert_eq!(
        created_user, expected_user,
        "Both should be equal now since the database assigned real values for the internal id and \
        created_at columns."
    );

}

/// Verify that an error is returned when an invalid birth year is specified
#[test]
fn create_user_fails_with_invalid_birth_year() {
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
        .build()
        .expect_err("Expected an error since an invalid birth year was used");
}

/// Verify that an error is returned when an invalid birth month is specified
#[test]
fn create_user_fails_with_invalid_birth_month() {
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
        .build()
        .expect_err("Expected an error since an invalid birth month was used");
}

/// Verify that an error is returned when an invalid day of birth is specified
#[test]
fn create_user_fails_with_invalid_day_of_birth() {
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
        .build()
        .expect_err("Expected an error since an invalid day of birth was used");
}
