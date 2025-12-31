//! Provides logic for user management

use std::collections::HashMap;

use rusqlite::{Statement};
use time::{Date, Month};

use crate::{
    database::{Query, user::UserDatabase}, 
    error::ImpulsePhmError, util::date_util
};


pub const DEFAULT_USER_ID: i64 = 0;
pub const DEFAULT_USER_CREATED_AT: i64 = 0;

/// A simple data object that represents an end-user
#[derive(Debug, PartialEq)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub birth_year: i64,
    pub birth_month: i8,
    // Specifically, the day of a user's date of birth
    pub birth_day: i8,
    pub created_at: i64
}

/// A builder to create a [`User`]
/// 
/// The "id" and "created_at" fields don't have public methods because they are determined by 
/// the end-user's user database.
pub struct UserBuilder {
    id: Option<i64>,
    first_name: Option<String>,
    last_name: Option<String>,
    birth_year: Option<i64>,
    birth_month: Option<i8>,
    birth_day: Option<i8>,
    created_at: Option<i64>
}

impl UserBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            first_name: None,
            last_name: None,
            birth_year: None,
            birth_month: None,
            birth_day: None,
            created_at: None
        }
    }

    pub fn with_first_name(mut self, first_name: &str) -> Self {
        self.first_name = Some(first_name.to_owned());
        self
    }

    pub fn with_last_name(mut self, last_name: &str) -> Self {
        self.last_name = Some(last_name.to_owned());
        self
    }

    pub fn with_birth_year(mut self, birth_year: i64) -> Self {
        self.birth_year = Some(birth_year);
        self
    }

    pub fn with_birth_month(mut self, birth_month: i8) -> Self {
        self.birth_month = Some(birth_month);
        self
    }

        pub fn with_birth_day(mut self, birth_day: i8) -> Self {
        self.birth_day = Some(birth_day);
        self
    }

    // Private Methods
    fn with_id(mut self, id: i64) -> Self {
        self.id = Some(id);
        self
    }

    fn with_created_at(mut self, created_at: i64) -> Self {
        self.created_at = Some(created_at);
        self
    }

    /// Build a [`User`]
    /// 
    /// The builder instance is consumed here, so it will not be reusable afterwards.
    /// 
    /// # Returns:
    /// A [`User`]
    /// 
    /// # Errors:
    /// 1. Returns an [`ImpulsePhmError::MissingValue`] if any required field is missing
    pub fn build(self) -> Result<User, ImpulsePhmError> {
        let first_name = match self.first_name {
            Some(value) => value,
            None => {
                log::error!("A first name is required");
                return Err(
                    ImpulsePhmError::MissingValue("A first name is required".to_owned())
                );
            },
        };

        let last_name = match self.last_name {
            Some(value) => value,
            None => {
                log::error!("A last name is required");
                return Err(
                    ImpulsePhmError::MissingValue("A last name is required".to_owned())
                );
            },
        };

        let birth_month = match self.birth_month {
            Some(value) => {
                match value {
                    1..=12 => value,
                    _ => {
                        log::error!("The birth month must be from 1 to 12");
                        return Err(
                            ImpulsePhmError::InvalidValue("The birth month must be from \
                            1 to 12".to_owned())
                        );
                    }
                }
            },
            None => {
                log::error!("A birth month is required");
                return Err(
                    ImpulsePhmError::MissingValue("A birth month is required".to_owned())
                );
            },
        };

        let birth_day = match self.birth_day {
            Some(value) => {
                match value {
                    1..31 => {
                        let max_days_per_month: HashMap<i8, i8> = HashMap::from([
                            (1, 31),
                            // A reminder that February has 29  days on a leap year
                            (2, 29),
                            (3, 31),
                            (4, 30),
                            (5, 31),
                            (6, 30),
                            (7, 31),
                            (8, 31),
                            (9, 30),
                            (10, 31),
                            (11, 30),
                            (12, 31),

                        ]);
                        
                        let max_days_for_chosen_month = max_days_per_month
                            .get(&birth_month)
                            .unwrap();
                        
                        if value <= *max_days_for_chosen_month {
                            value
                        }
                        else {
                            log::error!("{} is not a valid day for month {}", value, birth_month);
                            return Err(
                                ImpulsePhmError::InvalidValue(
                                    format!("{} is not a valid day for month {}", value, 
                                    birth_month)
                                )
                            );
                        }
                    },
                    _ => {
                        log::error!("The specific day of birth must be from 1 to 31");
                        return Err(
                            ImpulsePhmError::InvalidValue("The specific day of birth day must be \
                            from 1 to 31".to_owned())
                        );
                    }
                }
            },
            None => {
                log::error!("A specific day of birth is required");
                return Err(
                    ImpulsePhmError::MissingValue("A specific day of birth day is \
                    required".to_owned())
                );
            },
        };

        let birth_year = match self.birth_year {
            Some(value) => {
                // The oldest recorded age
                let max_age = 122;

                // The calls to unwrap are okay since the birth month and day have already 
                // been validated.
                let date_of_birth = Date::from_calendar_date(
                    value as i32,
                    Month::try_from(birth_month as u8).unwrap(),
                    birth_day as u8
                ).unwrap();

                let age = date_util::get_age(&date_of_birth);
                if age <= max_age {
                    value
                }
                else {
                    log::error!("Your age should be less than or equal to {max_age} unless you set \
                    a new world record for the oldest living person!");
                    return Err(
                            ImpulsePhmError::InvalidValue(
                                format!("Your age should be less than or equal to {max_age} unless \
                                you set a new world record for the oldest living person!")
                            )
                    );
                }
            },
            None => {
                log::error!("A birth year is required");
                return Err(
                    ImpulsePhmError::MissingValue("A birth year is required".to_owned())
                );
            },
        };
        
        let id = match self.id {
            Some(value) => value,
            None => DEFAULT_USER_ID,
        };

        let created_at = match self.created_at {
            Some(value) => value,
            None => DEFAULT_USER_CREATED_AT,
        };

        let user = User {
            id: id,
            first_name: first_name,
            last_name: last_name,
            birth_year: birth_year,
            birth_month: birth_month,
            birth_day: birth_day,
            created_at: created_at,
        };

        Ok(user)
    }
}

/// Represents everything the application can do with user accounts
pub struct UserContext<'a> {
    database: &'a UserDatabase
}

impl<'a> UserContext<'a> {
    pub fn new(database: &'a UserDatabase) -> Self {
        Self {
            database: database
        }
    }

    /// Create a new user in the user database
    /// 
    /// # Parameters
    /// `user`: the end-user to create
    /// 
    /// # Returns:
    /// The newly created user (including the ID and created_at values).
    /// 
    /// # Errors:
    /// 1. Returns a [`rusqlite::Error`] if there's a problem with preparing or executing the 
    /// SQL query.
    pub fn create_user(&self, user: &User) 
    -> Result<User, rusqlite::Error> {
        let mut sql: Statement = self.database.get_connection().prepare(
            "INSERT INTO user (first_name, last_name, birth_year, birth_month, \
            birth_day, created_at) \
            VALUES \
            (?1, ?2, ?3, ?4, ?5, unixepoch('now')) \
            RETURNING id, first_name, last_name, birth_year, birth_month, birth_day, created_at;"
        )?;

        log::debug!("Successfully prepared the SQL statement");

        let create_user_result: Result<User, rusqlite::Error> = sql.query_one(
            [&user.first_name, 
            &user.last_name, 
            &user.birth_year.to_string(),
            &user.birth_month.to_string(),
            &user.birth_day.to_string()],
            |row| Ok(User {
                id: row.get(0)?,
                first_name: row.get("first_name")?, 
                last_name: row.get("last_name")?, 
                birth_year: row.get("birth_year")?,
                birth_month: row.get("birth_month")?,
                birth_day: row.get("birth_day")?,
                created_at: row.get("created_at")?
            })
        );

        match create_user_result {
            Ok(created_user) => Ok(created_user),
            Err(e) => {
                log::error!("Failed to create a user: {}", e);
                return Err(e);
            }
        }
    }
}
