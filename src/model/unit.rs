//! Logic for the management of units of measurement

use rusqlite::Statement;

use crate::{ImpulsePhmError, Query, UserDatabase};


pub const DEFAULT_ID: i64 = 0;

/// A simple data object that represents a unit of measurement
#[derive(Debug, PartialEq)]
pub struct Unit {
    pub id: i64,
    pub singular_name: String,
    pub plural_name: String,
    pub abbreviation: Option<String>
}

/// A builder for a [`Unit`]
/// 
/// The "id" field doesn't have a public method because it's determined by the end-user's user 
/// database.
pub struct UnitBuilder {
    pub id: Option<i64>,
    pub singular_name: Option<String>,
    pub plural_name: Option<String>,
    pub abbreviation: Option<String>
}

impl UnitBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            singular_name: None,
            plural_name: None,
            abbreviation: None
        }
    }

    pub fn with_singular_name(mut self, singular_name: &str) -> Self {
        self.singular_name = Some(singular_name.to_owned());
        self
    }

    pub fn with_plural_name(mut self, plural_name: &str) -> Self {
        self.plural_name = Some(plural_name.to_owned());
        self
    }

    pub fn with_abbreviation(mut self, abbreviation: &str) -> Self {
        self.abbreviation = Some(abbreviation.to_owned());
        self
    }

    /// Build a [`Unit`]
    /// 
    /// The builder instance is consumed here, so it will not be reusable afterwards.
    /// 
    /// # Returns:
    /// A [`Unit`]
    /// 
    /// # Errors:
    /// Returns an [`ImpulsePhmError::MissingValue`] if any required field is missing
    pub fn build(self) -> Result<Unit, ImpulsePhmError> {
        let singular_name = match self.singular_name {
            Some(value) => value,
            None => {
                log::error!("A singular name is required");
                return Err(
                    ImpulsePhmError::MissingValue("A singular name is required".to_owned())
                );
            },
        };

        let plural_name = match self.plural_name {
            Some(value) => value,
            None => {
                log::error!("A plural name is required (can be equal to the singular name)");
                return Err(
                    ImpulsePhmError::MissingValue("A plural name is required (can be equal to the \
                    singular name)".to_owned())
                );
            },
        };

        let id = match self.id {
            Some(value) => value,
            None => DEFAULT_ID,
        };

        let unit = Unit {
            id: id,
            singular_name: singular_name,
            plural_name: plural_name,
            abbreviation: self.abbreviation,
        };

        Ok(unit)
    }
}


/// Represents everything the application can do with units
pub struct UnitContext<'a> {
    database: &'a UserDatabase
}

impl<'a> UnitContext<'a> {
    pub fn new(database: &'a UserDatabase) -> Self {
        Self {
            database: database
        }
    }
    
    /// Get all non-frequency units
    /// 
    /// This method is useful when needing to get the units that can be associated with the 
    /// quantity of a bioactive ingredient. For example, milligrams (mg).
    /// 
    /// # Returns:
    /// All non-frequency units
    /// 
    /// # Errors:
    /// [`rusqlite::Error`] if there's a problem with executing the query
    pub fn get_non_frequency_units(&self) -> Result<Vec<Unit>, rusqlite::Error> {
        // This SQL query gets any custom, non-frequency unit added by the end-user too.
        let mut sql: Statement = self.database.get_connection().prepare(
            "SELECT unit.id, unit.singular_name, unit.plural_name, unit.abbreviation \
            FROM categorized_unit \
            JOIN unit ON categorized_unit.unit_id=unit.id \
            JOIN unit_category ON categorized_unit.category_id=unit_category.id \
            WHERE unit_category.name != 'frequency';"
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

    /// Get all frequency units
    /// 
    /// This method is useful to get frequency-based units that can be associated with a 
    /// bioactive agent. For example, "once daily".
    /// 
    /// # Returns:
    /// All frequency-based units
    /// 
    /// # Errors:
    /// [`rusqlite::Error`] if there's a problem with executing the query
    pub fn get_frequency_units(&self) -> Result<Vec<Unit>, rusqlite::Error> {
        // TODO: Must account for custom frequency units too
        let mut sql: Statement = self.database.get_connection().prepare(
            "SELECT unit.id, unit.singular_name, unit.plural_name, unit.abbreviation \
            FROM categorized_unit \
            JOIN unit ON categorized_unit.unit_id=unit.id \
            JOIN unit_category ON categorized_unit.category_id=unit_category.id \
            WHERE unit_category.name = 'frequency';"
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
}
