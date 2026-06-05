//! Logic for the management of bioactive agents
//! 
//! For clarity, a bioactive agent is a substance that can influence an organism, tissue, or cell. 
//! Specifically, the bioactive agents relevant to this project are medications and 
//! dietary supplements.

use rusqlite::{ToSql, params, types::{FromSql, FromSqlError, ToSqlOutput}};

use crate::{FromRow, GetById};


pub const BIOACTIVE_AGENT_KIND_PRESCRIPTION: &str = "prescription medication";
pub const BIOACTIVE_AGENT_KIND_OTC: &str = "over-the-counter (OTC) medication";
pub const BIOACTIVE_AGENT_KIND_SUPPLEMENT: &str = "dietary supplement";

pub const DEFAULT_ID: i64 = 0;
pub const DEFAULT_CREATED_AT: i64 = 0;
pub const DEFAULT_IS_DELETED: bool = false;

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

impl ToSql for BioactiveAgentKind {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let agent_kind = match self {
            BioactiveAgentKind::PrescriptionMedication => BIOACTIVE_AGENT_KIND_PRESCRIPTION,
            BioactiveAgentKind::OverTheCounterMedication => BIOACTIVE_AGENT_KIND_OTC,
            BioactiveAgentKind::DietarySupplement => BIOACTIVE_AGENT_KIND_SUPPLEMENT
        };

        Ok(ToSqlOutput::from(agent_kind))
    }
}

impl FromSql for BioactiveAgentKind {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value.as_str()? {
            BIOACTIVE_AGENT_KIND_PRESCRIPTION => Ok(BioactiveAgentKind::PrescriptionMedication),
            BIOACTIVE_AGENT_KIND_OTC => Ok(BioactiveAgentKind::OverTheCounterMedication),
            BIOACTIVE_AGENT_KIND_SUPPLEMENT => Ok(BioactiveAgentKind::DietarySupplement),
            _ => Err(FromSqlError::InvalidType)
        }
    }
}

/// A simple data object that represents a bioactive agent
#[derive(Clone, Debug, PartialEq)]
pub struct BioactiveAgent {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub quantity: f64,
    pub quantity_unit_id: i64,
    pub frequency_unit_id: i64,
    pub agent_type_id: i64,
    pub created_at: i64,
    pub is_deleted: bool,
    pub reason: Option<String>,
    pub notes: Option<String>
}

impl FromRow for BioactiveAgent {
    fn from_row(row: &rusqlite::Row<'_>) -> Result<Self, rusqlite::Error> {
        Ok(BioactiveAgent {
            id: row.get("id")?,
            user_id: row.get("user_id")?,
            name: row.get("name")?,
            quantity: row.get("quantity")?,
            quantity_unit_id: row.get("quantity_unit_id")?,
            frequency_unit_id: row.get("frequency_unit_id")?,
            agent_type_id: row.get("agent_type_id")?,
            created_at: row.get("created_at")?,
            is_deleted: row.get("is_deleted")?,
            reason: row.get("reason")?,
            notes: row.get("notes")?
        })
    }
}

impl GetById<i64> for BioactiveAgent {
    fn get_by_id(database: &impl crate::ManageDatabase, id: i64) -> Result<Self, rusqlite::Error> {
        let agent = database.get_connection().query_one(
            "SELECT ba.*, baoi.reason, baoi.notes \
            FROM bioactive_agent AS ba \
            JOIN bioactive_agent_optional_information AS baoi\
            ON baoi.agent_id = ba.id
            WHERE ba.id = ?1;",
            params![id], |row| {
                Ok(BioactiveAgent::from_row(&row)?)
            }
        )?;

        Ok(agent)        
    }
}
