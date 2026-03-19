//! Types and functions to integrate this project with impulse-gui (i.e., Rust and C++)

use std::{io, sync::{Mutex, OnceLock}};

use crate::{ImpulseCore, ImpulsePhmError, User, UserBuilder, environment, resource};


#[cxx::bridge(namespace = "impulse_core")]
mod ffi {
    extern "Rust" {
        type UserProxy;

        fn initialize();
        fn db_check_user_database_exists() -> Result<bool>;
        fn user_build_user(first_name: &str, last_name: &str, birth_month: i8, birth_day: i8, 
            birth_year: i64) -> Result<Box<UserProxy>>;
        fn user_save_user(user_proxy: &Box<UserProxy>) -> Result<Box<UserProxy>>;
    }
}

/// A static [`ImpulseCore`] so that I don't have to pass a million references around in C++ 
/// for the impulse-gui project.
/// 
/// Requires wrapping in a mutex due to the inner connection objects requiring it for static use.
static IMPULSE_CORE: OnceLock<Mutex<ImpulseCore>> = OnceLock::new();

/// A proxy for a [`User`]
pub struct UserProxy {
    user: User
}

/// Initialize impulse-core so that the core functionality and environment is ready to be used 
/// by impulse-gui.
pub fn initialize() {
    IMPULSE_CORE.get_or_init(|| {
        Mutex::new(environment::setup_environment()
            .expect("Failed to initialize impulse-core because setting up the environment failed")
        )
    });
}

/// Determine if the user database exists or not
pub fn db_check_user_database_exists() -> Result<bool, io::Error> {
    resource::check_user_database_exists()
}

/// Build a user
fn user_build_user(first_name: &str, last_name: &str, birth_month: i8, birth_day: i8, 
    birth_year: i64) -> Result<Box<UserProxy>, ImpulsePhmError> {
    let user = UserBuilder::new()
        .with_first_name(first_name)
        .with_last_name(last_name)
        .with_birth_month(birth_month)
        .with_birth_day(birth_day)
        .with_birth_year(birth_year)
        .build()?;

    let user_proxy = UserProxy {
        user: user
    };

    Ok(Box::new(user_proxy))
}

/// Save the user built with [`build_user`]
/// 
/// # Returns:
/// The newly saved user including the data that the database created during the original save.
fn user_save_user(user_proxy: &Box<UserProxy>) -> Result<Box<UserProxy>, ImpulsePhmError> {
    let saved_user = IMPULSE_CORE.get().unwrap().lock().unwrap()
        .with_user()
        .save_user(&user_proxy.user)?;

    let user_proxy = UserProxy {
        user: saved_user
    };

    Ok(Box::new(user_proxy))

}
