//! Logic for the management of bioactive agents
//! 
//! For clarity, a bioactive agent is a substance that can influence an organism, tissue, or cell. 
//! Specifically, the bioactive agents relevant to this project are medications and 
//! dietary supplements.

use rusqlite::{Statement, params};

use crate::{FromRow, GetById, GetByName, ImpulsePhmError, ManageDatabase, RowExt, database::Insert};


pub const BIOACTIVE_AGENT_CATEGORY_PRESCRIPTION: &str = "prescription medication";
pub const BIOACTIVE_AGENT_CATEGORY_OTC: &str = "over-the-counter (OTC) medication";
pub const BIOACTIVE_AGENT_CATEGORY_SUPPLEMENT: &str = "dietary supplement";

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
    fn save_bioactive_agent(&mut self, agent: &BioactiveAgent) -> 
        Result<BioactiveAgent, rusqlite::Error>;
    
    // TODO: Add get_bioactive_agent
    // TODO: Add iter_bioactive_agents
    // TODO: Add update_bioactive_agent
    // TODO: Add delete_bioactive_agent
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
    pub agent_category_id: i64,
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
            agent_category_id: row.get("agent_category_id")?,
            created_at: row.get("created_at")?,
            is_deleted: row.get("is_deleted")?,
            // Since the optional values are from another table, they may not always appear in all 
            // use-cases.
            reason: row.get_or("reason", None)?,
            notes: row.get_or("notes", None)?
        })
    }
}

impl GetById<i64> for BioactiveAgent {
    fn get_by_id<D: ManageDatabase>(database: &D, id: i64) -> Result<Self, rusqlite::Error> {
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

impl Insert for BioactiveAgent {
    fn insert<D: ManageDatabase>(database: &mut D, agent: &Self) -> Result<Self, rusqlite::Error> {
        let transaction = database.get_connection_mut().transaction()?;

        let mut required_data_sql: Statement = transaction.prepare(
            "INSERT INTO bioactive_agent (user_id, name, quantity, quantity_unit_id, \
            frequency_unit_id, agent_category_id) \
            VALUES (?1, ?2, ?3, ?4, ?5, ?6) \
            RETURNING id, user_id, name, quantity, quantity_unit_id, frequency_unit_id, \
            agent_category_id, created_at, is_deleted;")?;
        
        let save_agent_result = required_data_sql.query_one(
            params![
                &agent.user_id, 
                &agent.name, 
                &agent.quantity,
                &agent.quantity_unit_id,
                &agent.frequency_unit_id,
                &agent.agent_category_id
            ],
                |row| Ok(BioactiveAgent::from_row(row)?)
        );

        let mut saved_agent = match save_agent_result {
            Ok(saved_agent) => saved_agent,
            Err(e) => {
                log::error!("Failed to save a bioactive agent: {}", e);
                return Err(e);
            }
        };

        if agent.reason.is_some() || agent.notes.is_some() {
            // There is optional data to save into the database
            let mut optional_data_sql: Statement = transaction.prepare(
            "INSERT INTO bioactive_agent_optional_information (agent_id, reason, notes) \
            VALUES (?1, ?2, ?3);")?;

            match optional_data_sql.execute(
                params![saved_agent.id, agent.reason, agent.notes]
            ) {
                Ok(_) => {
                    saved_agent.reason = agent.reason.clone();
                    saved_agent.notes = agent.notes.clone();
                    Ok(saved_agent)
                },
                Err(e) => {
                    log::error!("Failed to save optional bioactive agent data: {}", e);
                    return Err(e);
                },
            }
        }
        else {
            // Only required data was saved to the database
            Ok(saved_agent)
        }
    }
}

/// A type of [`BioactiveAgent`]
#[derive(Clone, Debug, PartialEq)]
pub struct BioactiveAgentCategory {
    pub id: i64,
    pub name: String
}

impl FromRow for BioactiveAgentCategory {
    fn from_row(row: &rusqlite::Row<'_>) -> Result<Self, rusqlite::Error> {
        Ok(BioactiveAgentCategory {
            id: row.get("id")?,
            name: row.get("name")?
        })
    }
}

impl GetById<i64> for BioactiveAgentCategory {
    fn get_by_id<D: ManageDatabase>(database: &D, id: i64) -> Result<Self, rusqlite::Error> {
        let bioactive_agent_category = database.get_connection().query_one(
            "SELECT * FROM bioactive_agent_category WHERE id = ?1;",
            params![id], |row| {
                Ok(BioactiveAgentCategory::from_row(&row)?)
            }
        )?;

        Ok(bioactive_agent_category)   
    }
}

impl GetByName for BioactiveAgentCategory {
    fn get_by_name<D: ManageDatabase>(database: &D, name: &str) -> Result<Self, rusqlite::Error> {
        let bioactive_agent_category = database.get_connection().query_one(
            "SELECT * FROM bioactive_agent_category WHERE name = ?1;",
            params![name], |row| {
                Ok(BioactiveAgentCategory::from_row(&row)?)
            }
        )?;

        Ok(bioactive_agent_category)   
    }
}

/// A builder to create a [`BioactiveAgent`]
/// 
/// The "id" and "created_at" fields don't have public methods because they are determined by 
/// the end-user's user database. Additionally, the is_deleted field doesn't need one either as a 
/// default value is used.
pub struct BioactiveAgentBuilder {
    pub id: Option<i64>,
    pub user_id: Option<i64>,
    pub name: Option<String>,
    pub quantity: Option<f64>,
    pub quantity_unit_id: Option<i64>,
    pub frequency_unit_id: Option<i64>,
    pub agent_category_id: Option<i64>,
    pub created_at: Option<i64>,
    pub is_deleted: Option<bool>,
    pub reason: Option<String>,
    pub notes: Option<String>
}

impl BioactiveAgentBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            user_id: None,
            name: None,
            quantity: None,
            quantity_unit_id: None,
            frequency_unit_id: None,
            agent_category_id: None,
            created_at: None,
            is_deleted: None,
            reason: None,
            notes: None,
        }
    }

    pub fn with_user_id(mut self, user_id: i64) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }

    pub fn with_quantity(mut self, quantity: f64) -> Self {
        self.quantity = Some(quantity);
        self
    }

    pub fn with_quantity_unit_id(mut self, quantity_unit_id: i64) -> Self {
        self.quantity_unit_id = Some(quantity_unit_id);
        self
    }

    pub fn with_frequency_unit_id(mut self, frequency_unit_id: i64) -> Self {
        self.frequency_unit_id = Some(frequency_unit_id);
        self
    }

    pub fn with_agent_category_id(mut self, agent_category_id: i64) -> Self {
        self.agent_category_id = Some(agent_category_id);
        self
    }

    pub fn with_reason(mut self, reason: &str) -> Self {
        self.reason = Some(reason.to_owned());
        self
    }

    pub fn with_notes(mut self, notes: &str) -> Self {
        self.notes = Some(notes.to_owned());
        self
    }

    /// Build a [`BioactiveAgent`]
    /// 
    /// The builder instance is consumed here, so it will not be reusable afterwards.
    /// 
    /// # Returns:
    /// A [`BioactiveAgent`]
    /// 
    /// # Errors:
    /// Returns an [`ImpulsePhmError::MissingValue`] if any required field is missing
    pub fn build(self) -> Result<BioactiveAgent, ImpulsePhmError> {
        let user_id = match self.user_id {
            Some(value) => value,
            None => {
                log::error!("A user ID is required");
                return Err(
                    ImpulsePhmError::MissingValue("A user ID is required".to_owned())
                );
            },
        }; 

        let name = match self.name {
            Some(value) => value,
            None => {
                log::error!("A name is required");
                return Err(
                    ImpulsePhmError::MissingValue("A name is required".to_owned())
                );
            },
        };

        let quantity = match self.quantity {
            Some(value) => value,
            None => {
                log::error!("A quantity is required");
                return Err(
                    ImpulsePhmError::MissingValue("A quantity is required".to_owned())
                );
            },
        };     

        let quantity_unit_id = match self.quantity_unit_id {
            Some(value) => value,
            None => {
                log::error!("A quantity unit ID is required");
                return Err(
                    ImpulsePhmError::MissingValue("A quantity unit ID is required".to_owned())
                );
            },
        };   

        let frequency_unit_id = match self.frequency_unit_id {
            Some(value) => value,
            None => {
                log::error!("A frequency unit ID is required");
                return Err(
                    ImpulsePhmError::MissingValue("A frequency unit ID is required".to_owned())
                );
            },
        };     

        let agent_category_id = match self.agent_category_id {
            Some(value) => value,
            None => {
                log::error!("An agent category ID is required");
                return Err(
                    ImpulsePhmError::MissingValue("An agent category ID is required".to_owned())
                );
            },
        }; 

        let id = match self.id {
            Some(value) => value,
            None => DEFAULT_ID
        }; 

        let created_at = match self.created_at {
            Some(value) => value,
            None => DEFAULT_CREATED_AT
        };

        let is_deleted = match self.is_deleted {
            Some(value) => value,
            None => DEFAULT_IS_DELETED
        };

        let bioactive_agent = BioactiveAgent {
            id: id,
            user_id: user_id,
            name: name,
            quantity: quantity,
            quantity_unit_id: quantity_unit_id,
            frequency_unit_id: frequency_unit_id,
            agent_category_id: agent_category_id,
            created_at: created_at,
            is_deleted: is_deleted,
            reason: self.reason,
            notes: self.notes,
        };

        Ok(bioactive_agent)
    }   
}
