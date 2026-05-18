//! Types that provide the core logic of Impulse PHM

pub mod bioactive;
pub mod unit;
pub mod user;

use rusqlite::{OptionalExtension, Statement, params};

use crate::{
    ImpulsePhmError, ManageUnit, ManageUser, Query, Unit, User, 
    database::{core::CoreDatabase, user::UserDatabase}, model
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
}

impl ManageUnit for ImpulseCore {
    fn get_non_frequency_units(&self) -> Result<Vec<Unit>, rusqlite::Error> {
        let mut sql: Statement = self.user_database.get_connection().prepare(
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

    fn get_frequency_units(&self) -> Result<Vec<Unit>, rusqlite::Error> {
        let mut sql: Statement = self.user_database.get_connection().prepare(
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

impl ManageUser for ImpulseCore {
    fn save_user(&self, user: &User) -> Result<User, rusqlite::Error> {
        let mut sql: Statement = self.user_database.get_connection().prepare(
            "INSERT INTO user (first_name, last_name, birth_year, birth_month, \
            birth_day, is_primary, created_at) \
            VALUES \
            (?1, ?2, ?3, ?4, ?5, ?6, unixepoch('now')) \
            RETURNING id, first_name, last_name, birth_year, birth_month, birth_day, is_primary, \
            created_at;"
        )?;

        log::debug!("Successfully prepared the SQL statement");

        let save_user_result: Result<User, rusqlite::Error> = sql.query_one(
            params![&user.first_name, 
            &user.last_name, 
            &user.birth_year,
            &user.birth_month,
            &user.birth_day,
            &user.is_primary],
            |row| Ok(User {
                id: row.get("id")?,
                first_name: row.get("first_name")?, 
                last_name: row.get("last_name")?, 
                birth_year: row.get("birth_year")?,
                birth_month: row.get("birth_month")?,
                birth_day: row.get("birth_day")?,
                created_at: row.get("created_at")?,
                is_primary: row.get("is_primary")?
            })
        );

        match save_user_result {
            Ok(saved_user) => Ok(saved_user),
            Err(e) => {
                log::error!("Failed to save a user: {}", e);
                return Err(e);
            }
        }
    }

    fn get_primary_user(&self) -> Result<Option<User>, rusqlite::Error> {
        let user: Option<User> = self.user_database.get_connection().query_one(
            "SELECT * FROM user WHERE is_primary = 1;",
            params![], |row| {
                Ok(User {
                    id: row.get("id")?,
                    first_name: row.get("first_name")?, 
                    last_name: row.get("last_name")?, 
                    birth_year: row.get("birth_year")?,
                    birth_month: row.get("birth_month")?,
                    birth_day: row.get("birth_day")?,
                    created_at: row.get("created_at")?,
                    is_primary: row.get("is_primary")?
                })
            }
        ).optional()?;

        Ok(user)
    }
    
    fn update_user(&self, user: &User) -> Result<User, ImpulsePhmError> {
        if user.id == model::user::DEFAULT_ID {
            return Err(
                ImpulsePhmError::InvalidValue(
                    "Used the default ID, use an ID that maps to a real user instead.".to_owned()
                )
            );
        }

        if user.created_at == model::user::DEFAULT_CREATED_AT {
            return Err(
                ImpulsePhmError::InvalidValue(
                    "Used the default \"created at\" value, use a value that maps to a real user \
                    instead.".to_owned()
                )
            );
        }

        let mut sql: Statement = self.user_database.get_connection().prepare(
            "UPDATE user \
            SET \
              first_name = ?1, \
              last_name = ?2, \
              birth_year = ?3, \
              birth_month = ?4, \
              birth_day = ?5 \
            WHERE id = ?6
            RETURNING id, first_name, last_name, birth_year, birth_month, birth_day, is_primary, \
            created_at;"
        )?;

        let update_user_result: Result<User, rusqlite::Error> = sql.query_one(
            params![
                &user.first_name, 
                &user.last_name, 
                &user.birth_year,
                &user.birth_month,
                &user.birth_day,
                &user.id
            ],
            |row| Ok(User {
                id: row.get("id")?,
                first_name: row.get("first_name")?, 
                last_name: row.get("last_name")?, 
                birth_year: row.get("birth_year")?,
                birth_month: row.get("birth_month")?,
                birth_day: row.get("birth_day")?,
                created_at: row.get("created_at")?,
                is_primary: row.get("is_primary")?
            })
        );

        match update_user_result {
            Ok(updated_user) => Ok(updated_user),
            Err(e) => {
                log::error!("Failed to update a user: {}", e);
                return Err(ImpulsePhmError::Database(e));
            }
        }
    }
}
