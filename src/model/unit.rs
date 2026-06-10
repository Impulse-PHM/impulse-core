//! Logic for the management of units of measurement

use rusqlite::params;

use crate::{FromRow, GetById, GetByName, ImpulsePhmError, ManageDatabase, UserDatabase};


pub const DEFAULT_ID: i64 = 0;


/// Allows database operations to be performed on a unit of measurement
pub trait ManageUnit {
    /// Get all non-frequency units
    /// 
    /// This method is useful when needing to get the units that can be associated with the 
    /// quantity of a [`crate::BioactiveAgent`]. For example, milligrams (mg).
    /// 
    /// # Returns:
    /// All non-frequency units
    /// 
    /// # Errors:
    /// [`rusqlite::Error`] if there's a problem with executing the query
    fn get_non_frequency_units(&self) -> Result<Vec<Unit>, rusqlite::Error>;

    /// Get all frequency units
    /// 
    /// This method is useful to get frequency-based units that can be associated with a 
    /// [`crate::BioactiveAgent`]. For example, "once daily".
    /// 
    /// # Returns:
    /// All frequency-based units
    /// 
    /// # Errors:
    /// [`rusqlite::Error`] if there's a problem with executing the query
    fn get_frequency_units(&self) -> Result<Vec<Unit>, rusqlite::Error>;
}

/// A simple data object that represents a unit of measurement
#[derive(Clone, Debug, PartialEq)]
pub struct Unit {
    pub id: i64,
    pub name: String,
    pub plural_name: String,
    pub abbreviation: Option<String>
}

impl Unit {
    /// Create an instance based on the given plural name
    /// 
    /// # Parameters:
    /// `database`: the user's database
    /// `plural_name`: the plural name to search for
    /// 
    /// # Returns:
    /// An new instance
    /// 
    /// # Errors:
    /// [`rusqlite::Error`] if any issues occur with the database
    pub fn get_by_plural_name(database: &UserDatabase, plural_name: &str)
        -> Result<Self, rusqlite::Error> {
        let unit = database.get_connection().query_one(
            "SELECT * FROM unit WHERE plural_name = ?1;",
            params![plural_name], |row| {
                Ok(Unit::from_row(&row)?)
            }
        )?;

        Ok(unit)   
    }

    /// Create an instance based on the given abbreviation
    /// 
    /// # Parameters:
    /// `database`: the user's database
    /// `abbreviation`: the plural name to search for
    /// 
    /// # Returns:
    /// A new instance
    /// 
    /// # Errors:
    /// [`rusqlite::Error`] if any issues occur with the database
    pub fn get_by_abbreviation(database: &UserDatabase, abbreviation: &str)
        -> Result<Self, rusqlite::Error> {
        let unit = database.get_connection().query_one(
            "SELECT * FROM unit WHERE abbreviation = ?1;",
            params![abbreviation], |row| {
                Ok(Unit::from_row(&row)?)
            }
        )?;

        Ok(unit)   
    }
}

impl FromRow for Unit {
    fn from_row(row: &rusqlite::Row<'_>) -> Result<Self, rusqlite::Error> {
        Ok(Unit {
            id: row.get("id")?,
            name: row.get("name")?,
            plural_name: row.get("plural_name")?,
            abbreviation: row.get("abbreviation")?
        }) 
    }
}

impl GetById<i64> for Unit {
    fn get_by_id<D: ManageDatabase>(database: &D, id: i64) -> Result<Self, rusqlite::Error> {
        let unit = database.get_connection().query_one(
            "SELECT * FROM unit WHERE id = ?1;",
            params![id], |row| {
                Ok(Unit::from_row(&row)?)
            }
        )?;

        Ok(unit)   
    }
}

impl GetByName for Unit {
    fn get_by_name<D: ManageDatabase>(database: &D, name: &str) -> Result<Self, rusqlite::Error> {
        let unit = database.get_connection().query_one(
            "SELECT * FROM unit WHERE name = ?1;",
            params![name], |row| {
                Ok(Unit::from_row(&row)?)
            }
        )?;

        Ok(unit)   
    }
}

/// A builder for a [`Unit`]
/// 
/// The "id" field doesn't have a public method because it's determined by the end-user's user 
/// database.
pub struct UnitBuilder {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub plural_name: Option<String>,
    pub abbreviation: Option<String>
}

impl UnitBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
            plural_name: None,
            abbreviation: None
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
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
        let name = match self.name {
            Some(value) => value,
            None => {
                log::error!("A name is required");
                return Err(
                    ImpulsePhmError::MissingValue("A name is required".to_owned())
                );
            },
        };

        let plural_name = match self.plural_name {
            Some(value) => value,
            None => {
                log::error!("A plural name is required (can be equal to the regular name)");
                return Err(
                    ImpulsePhmError::MissingValue("A plural name is required (can be equal to the \
                    regular name)".to_owned())
                );
            },
        };

        let id = match self.id {
            Some(value) => value,
            None => DEFAULT_ID,
        };

        let unit = Unit {
            id: id,
            name: name,
            plural_name: plural_name,
            abbreviation: self.abbreviation,
        };

        Ok(unit)
    }
}
