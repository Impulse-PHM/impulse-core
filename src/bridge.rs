//! Types and functions to integrate this project with impulse-gui (i.e., Rust and C++)
//! 
//! Compared to the regular Rust API provided by [`crate::ImpulseCore`], the bridge API is 
//! intentionally redundant-looking because it's meant to keep things as simple as possible on the 
//! C++ side. This means the following:
//! 
//! - Custom Rust types exposed to C++ are "opaque" types to keep any complexity hidden from C++.
//! - The [`Box`] type is used to transfer ownership because, at the time of this writing, 
//! [`cxx::UniquePtr`] does not support this requirement.
//! - Functions have C-style namespace naming conventions.
//! - The bridge API has more functions compared to the same functionality in the regular API.
//! - Static Rust values are preferred over trying to have C++ manage complex lifetimes.

use std::{io, path::Path, sync::{Mutex, OnceLock}};

use crate::{ImpulseCore, ImpulsePhmError, UserBuilder, environment, resource};


#[cxx::bridge(namespace = "impulse_core")]
mod ffi {
    extern "Rust" {
        type User;
        type OptionalUser;

        fn initialize();

        // Resource Functions
        fn resource_check_user_database_exists() -> Result<bool>;
        fn resource_import_user_database(source_path: &str) -> Result<()>;

        // User Functions
        fn user_build_user(first_name: &str, last_name: &str, birth_month: i8, birth_day: i8, 
            birth_year: i64, is_primary: bool) -> Result<Box<User>>;
        fn user_save_user(user: &Box<User>) -> Result<Box<User>>;
        fn user_get_primary_user() -> Result<Box<OptionalUser>>;
        fn user_optional_user_is_some(optional_user: &Box<OptionalUser>) -> bool;
        fn user_get_id(user: &Box<User>) -> i64;
        fn user_get_first_name(user: &Box<User>) -> String;
        fn user_get_last_name(user: &Box<User>) -> String;
        fn user_get_birth_year(user: &Box<User>) -> i64;
        fn user_get_birth_month(user: &Box<User>) -> i8;
        fn user_get_birth_day(user: &Box<User>) -> i8;
        fn user_get_created_at(user: &Box<User>) -> i64;
        fn user_get_is_primary(user: &Box<User>) -> bool;
        fn user_try_from(optional_user: Box<OptionalUser>) -> Result<Box<User>>;
    }
}

/// A static [`ImpulseCore`] so that I don't have to pass a million references around in C++ 
/// for the impulse-gui project.
/// 
/// Requires wrapping in a mutex due to the inner connection objects requiring it for static use.
static IMPULSE_CORE: OnceLock<Mutex<ImpulseCore>> = OnceLock::new();

/// An opaque type for [`crate::User`]
pub struct User {
    real_user: crate::User
}

/// An opaque type for operations that may or may not return a [`crate::User`]
/// 
/// At the time of this writing, cxx does not support [`Option`], so creating a dedicated optional 
/// type is the next best thing.
pub struct OptionalUser {
    real_user: Option<crate::User>
}

/// Initialize impulse-core so that the core functionality and environment is ready to be used 
/// by impulse-gui.
/// 
/// Panics:
/// If the user's environment fails to set up for any reason (the application would not be usable).
pub fn initialize() {
    IMPULSE_CORE.get_or_init(|| {
        Mutex::new(environment::setup_environment()
            .expect("Failed to initialize impulse-core because setting up the environment failed")
        )
    });
}

pub fn resource_check_user_database_exists() -> Result<bool, io::Error> {
    resource::check_user_database_exists()
}

pub fn resource_import_user_database(source_path: &str) -> Result<(), ImpulsePhmError> {
    resource::import_user_database(Path::new(source_path))
}

fn user_build_user(first_name: &str, last_name: &str, birth_month: i8, birth_day: i8, 
    birth_year: i64, is_primary: bool) -> Result<Box<User>, ImpulsePhmError> {
    let real_user = UserBuilder::new()
        .with_first_name(first_name)
        .with_last_name(last_name)
        .with_birth_month(birth_month)
        .with_birth_day(birth_day)
        .with_birth_year(birth_year)
        .with_is_primary(is_primary)
        .build()?;

    let user = User {
        real_user
    };

    Ok(Box::new(user))
}

fn user_save_user(user: &Box<User>) -> Result<Box<User>, ImpulsePhmError> {
    let saved_user = IMPULSE_CORE.get().unwrap().lock().unwrap()
        .with_user()
        .save_user(&user.real_user)?;

    let user = User {
        real_user: saved_user
    };

    Ok(Box::new(user))

}

fn user_get_primary_user() -> Result<Box<OptionalUser>, rusqlite::Error> {
    let primary_user = IMPULSE_CORE.get().unwrap().lock().unwrap()
        .with_user()
        .get_primary_user()?;

    let optional_user = OptionalUser {
        real_user: primary_user
    };

    Ok(Box::new(optional_user))
}

fn user_optional_user_is_some(optional_user: &Box<OptionalUser>) -> bool {
    match optional_user.real_user {
        Some(_) => true,
        None => false
    }
}

fn user_get_id(user: &Box<User>) -> i64 {
    user.real_user.id
}

fn user_get_first_name(user: &Box<User>) -> String {
    user.real_user.first_name.clone()
}

fn user_get_last_name(user: &Box<User>) -> String {
    user.real_user.last_name.clone()
}

fn user_get_birth_year(user: &Box<User>) -> i64 {
    user.real_user.birth_year
}

fn user_get_birth_month(user: &Box<User>) -> i8 {
    user.real_user.birth_month
}

fn user_get_birth_day(user: &Box<User>) -> i8 {
    user.real_user.birth_day
}

fn user_get_created_at(user: &Box<User>) -> i64 {
    user.real_user.created_at
}

fn user_get_is_primary(user: &Box<User>) -> bool {
    user.real_user.is_primary
}

/// Try and convert an [`OptionalUser`] to a [`User`]
/// 
/// Despite the name, this function does not deal with the [`TryFrom`] trait --it just has 
/// similar behavior.
/// 
/// # Parameters
/// `optional_user`: The optional user to attempt to convert after consuming it
/// 
/// # Returns:
/// A [`Box<User>`]
/// 
/// # Errors:
/// [`ImpulsePhmError::MissingValue`] if there was no real user contained to convert
fn user_try_from(optional_user: Box<OptionalUser>) -> Result<Box<User>, ImpulsePhmError> {
    match optional_user.real_user {
        Some(valid_user) => {
            let user = User {
                real_user: valid_user
            };
            Ok(Box::new(user))
        },
        None => {
            Err(ImpulsePhmError::MissingValue("The optional user did not contain a \
            real user".to_string()))
        }
    }
}
