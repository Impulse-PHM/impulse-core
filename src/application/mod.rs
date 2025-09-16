//! Everything needed to manage the core application

pub mod environment;

use crate::{application::{environment::Environment}, 
    database::{core::CoreDatabase, user::UserDatabase}};
 

/// Represents the core application
pub struct CoreApplication<'a, 'b> {
    core_database: &'a CoreDatabase,
    user_database: &'b UserDatabase,
    environment: Environment<'a, 'b>,
}

/// All methods that have the prefix "for_" are meant to be used as a fluent API to group common 
/// functionality of a major feature.
impl<'a, 'b,> CoreApplication<'a, 'b,> {
    /// Create a new object with all fields already instantiated
    pub fn new(core_database: &'a CoreDatabase, user_database: &'b UserDatabase) -> Self {
        CoreApplication {
            core_database: core_database,
            user_database: user_database,
            environment: Environment::new(core_database, user_database)
        }
    }

    /// Return the operations of the "environment" feature
    pub fn for_environment(&self) -> &Environment<'a, 'b> { 
        &self.environment
    }
}
