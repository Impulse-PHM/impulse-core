//! Logic for the management of bioactive agents
//! 
//! For clarity, a bioactive agent is a substance that can influence an organism, tissue, or cell. 
//! Specifically, the bioactive agents relevant to this project are medications and 
//! dietary supplements.

use crate::{Unit, User, UserDatabase};


/// A type of [`BioactiveAgent`]
#[derive(Debug, PartialEq)]
pub enum BioactiveAgentKind {
    PrescriptionMedication,
    OverTheCounterMedication,
    DietarySupplement
}

/// A simple data object that represents a bioactive agent
#[derive(Debug, PartialEq)]
pub struct BioactiveAgent {
    pub id: i64,
    pub user: User,
    pub name: String,
    pub quantity: f64,
    pub quantity_unit: Unit,
    pub frequency_unit: Unit,
    pub kind: BioactiveAgentKind,
    pub is_prescription: i64,
    pub created_at: i64,
    pub is_deleted: i64
}

/// Represents everything the application can do with a [`BioactiveAgent`]
pub struct BioactiveAgentContext<'a> {
    database: &'a UserDatabase
}

impl<'a> BioactiveAgentContext<'a> {
    pub fn new(database: &'a UserDatabase) -> Self {
        Self {
            database: database
        }
    }
}
