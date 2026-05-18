//! Types that provide the core logic of Impulse PHM

pub mod bioactive;
pub mod unit;
pub mod user;

use rusqlite::{Statement, params};

use crate::{
    Query, Unit, 
    database::{core::CoreDatabase, user::UserDatabase}, 
    model::{unit::{ManageUnit}, user::UserContext}
};
 

/// Provides the core functionality of Impulse PHM
pub struct ImpulseCore {
    #[allow(dead_code)]
    core_database: CoreDatabase,
    user_database: UserDatabase
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

impl ManageUnit for ImpulseCore {
    fn get_non_frequency_units(&self) -> Result<Vec<Unit>, rusqlite::Error> {
        // This SQL query gets any custom, non-frequency unit added by the end-user too.
        let mut sql: Statement = self.user_database.get_connection().prepare(
            "SELECT unit.id, unit.singular_name, unit.plural_name, unit.abbreviation \
            FROM categorized_unit \
            JOIN unit ON categorized_unit.unit_id=unit.id \
            JOIN unit_category ON categorized_unit.category_id=unit_category.id \
            WHERE unit_category.name NOT IN ('frequency', 'custom frequency');"
        )?;

        let rows = match sql.query_map(
            [], |row| {
            Ok(Unit {
                id: row.get("id")?,
                singular_name: row.get("singular_name")?,
                plural_name: row.get("plural_name")?,
                abbreviation: row.get("abbreviation")?
            }) 
        }) {
            Ok(rows) => rows,
            Err(e) => {
                log::error!("Failed to get the non-frequency units: {}", e);
                return Err(e);
            },
        };

        let non_frequency_units = rows.collect::<Result<Vec<Unit>, rusqlite::Error>>()?;

        Ok(non_frequency_units)
    }

    fn get_frequency_units(&self) -> Result<Vec<Unit>, rusqlite::Error> {
        let mut sql: Statement = self.user_database.get_connection().prepare(
            "SELECT unit.id, unit.singular_name, unit.plural_name, unit.abbreviation \
            FROM categorized_unit \
            JOIN unit ON categorized_unit.unit_id=unit.id \
            JOIN unit_category ON categorized_unit.category_id=unit_category.id \
            WHERE unit_category.name IN ('frequency', 'custom frequency');"
        )?;

        let rows = match sql.query_map(
            [], |row| {
            Ok(Unit {
                id: row.get("id")?,
                singular_name: row.get("singular_name")?,
                plural_name: row.get("plural_name")?,
                abbreviation: row.get("abbreviation")?
            }) 
        }) {
            Ok(rows) => rows,
            Err(e) => {
                log::error!("Failed to get the frequency units: {}", e);
                return Err(e);
            },
        };

        let frequency_units = rows.collect::<Result<Vec<Unit>, rusqlite::Error>>()?;

        Ok(frequency_units)
    }

    fn save_custom_non_frequency_unit(&mut self, custom_unit: &Unit) -> 
        Result<Unit, rusqlite::Error> {
        // TODO: Use a transaction to add both the custom unit and categorized_unit mapping
        let sql = "INSERT INTO unit (singular_name, plural_name, abbreviation) \
            VALUES (?1, ?2, ?3) \
            RETURNING id, singular_name, plural_name, abbreviation";

        let transaction = self.user_database.get_connection_mut().transaction()?;
        
        let save_unit_result: Result<Unit, rusqlite::Error> = transaction.query_one(
            sql, 
            params![
                &custom_unit.singular_name,
                &custom_unit.plural_name,
                &custom_unit.abbreviation
            ],
            |row| Ok(Unit {
                id: row.get("id")?,
                singular_name: row.get("singular_name")?,
                plural_name: row.get("plural_name")?,
                abbreviation: row.get("abbreviation")?,
            })
        );

        match save_unit_result {
            Ok(saved_unit) => Ok(saved_unit),
            Err(e) => {
                log::error!("Failed to save a custom, non-frequency unit: {}", e);
                return Err(e);
            },
        }

        // TODO: Finish the implementation
    }
}
