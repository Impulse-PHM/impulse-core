//! Logic for the management of units of measurement

use crate::ImpulsePhmError;

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
