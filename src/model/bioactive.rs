//! Logic for the management of bioactive agents
//! 
//! For clarity, a bioactive agent is a substance that can influence an organism, tissue, or cell. 
//! Specifically, the bioactive agents relevant to this project are medications and 
//! dietary supplements.

use crate::{Unit, User};


/// Allows database operations to be performed on bioactive agents
pub trait ManageBioactiveAgent {
    /// Save a new [`BioactiveAgent`] in the user database
    /// 
    /// # Parameters
    /// `agent`: the bioactive agent to save
    /// 
    /// # Returns:
    /// The newly saved [`BioactiveAgent`]
    /// 
    /// # Errors:
    /// An [`rusqlite::Error`] if there's a problem with preparing or executing the SQL query.
    fn save_bioactive_agent(&self, agent: &BioactiveAgent) -> 
        Result<BioactiveAgent, rusqlite::Error>;
    
    // TODO: Add get_bioactive_agent
    // TODO: Add iter_bioactive_agents
    // TODO: Add update_bioactive_agent
    // TODO: Add delete_bioactive_agent
}

/// A type of [`BioactiveAgent`]
#[derive(Clone, Debug, PartialEq)]
pub enum BioactiveAgentKind {
    PrescriptionMedication,
    OverTheCounterMedication,
    DietarySupplement
}

/// A simple data object that represents a bioactive agent
#[derive(Clone, Debug, PartialEq)]
pub struct BioactiveAgent {
    pub id: i64,
    pub user: User,
    pub name: String,
    pub quantity: f64,
    pub quantity_unit: Unit,
    pub frequency_unit: Unit,
    pub kind: BioactiveAgentKind,
    pub created_at: i64,
    pub is_deleted: bool,
    pub reason: Option<String>,
    pub notes: Option<String>
}
