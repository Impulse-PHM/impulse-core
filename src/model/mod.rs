//! Types that provide the core logic of Impulse PHM

pub mod user;

use crate::{
    model::{user::UserContext}, 
    database::{core::CoreDatabase, user::UserDatabase}
};
 

/// Provides the core functionality of Impulse PHM
pub struct ImpulseCore {
    core_database: CoreDatabase,
    user_database: UserDatabase,
}

impl ImpulseCore {
    /// Create a new object with all fields already instantiated
    /// 
    /// # Parameters:
    /// `core_database` a [`CoreDatabase`] whose ownership will be moved
    /// `user_database` a [`UserDatabase`] whose ownership will be moved
    /// 
    /// # Returns:
    /// A new instance
    pub fn new(core_database: CoreDatabase, user_database: UserDatabase) -> Self {
        ImpulseCore {
            core_database: core_database,
            user_database: user_database,
        }
    }

    /// Return the operations for managing a user account
    pub fn with_user(&self) -> UserContext<'_> {
        UserContext::new(&self.user_database)
    }
}
