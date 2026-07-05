//! The core library to Impulse Personal Health Manager (Impulse PHM)

pub mod bridge;
pub mod database;
pub mod environment;
pub mod error;
pub mod ext;
pub mod model;
pub mod util;

// Re-export for a more convenient public API
pub use database::{
    FromRow, GetById, GetByName, ManageDatabase, Validate, 
    core::CoreDatabase, 
    user::UserDatabase
};
pub use environment::resource;
pub use error::ImpulsePhmError;
pub use model::{
    ImpulseCore, 
    bioactive::{BioactiveAgent, BioactiveAgentBuilder, BioactiveAgentCategory, ManageBioactiveAgent}, 
    unit::{ManageUnit, Unit, UnitBuilder}, 
    user::{ManageUser, User, UserBuilder}
};
