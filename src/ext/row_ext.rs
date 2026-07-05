//! An extension trait for [`rusqlite::Row`]

use rusqlite::{Row, types::FromSql};


pub trait RowExt {
    /// Get the value of a column, or return the given default value if the column is missing.
    /// 
    /// # Parameters:
    /// `column`: the name of the column
    /// `default`: the value to use if the column is missing
    /// 
    /// # Returns:
    /// The value of a column, or the given default value if the column is missing.
    /// 
    /// # Errors:
    /// A [`rusqlite::Error`] if there's a problem executing the SQL query
    fn get_or<T: FromSql>(&self, column: &str, default: T) -> Result<T, rusqlite::Error>;
}

impl<'stmt> RowExt for Row<'stmt> {
    fn get_or<T: FromSql>(&self, column: &str, default: T) -> Result<T, rusqlite::Error> {
        match self.get(column) {
            Ok(value) => Ok(value),
            Err(rusqlite::Error::InvalidColumnName(_)) => Ok(default),
            Err(e) => {
                log::error!("Failed to get the value for column \"{column}\": {e}");
                return Err(e);
            },
        }
    }
}
