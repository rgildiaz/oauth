//! We need a database in order to store the following tables:
//!     - the active Auth Grants (that can be exchanged for access tokens)
//!     - the active access tokens

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard, PoisonError};

#[derive(Debug)]
pub enum DatabaseInternalError {
    NoSuchTable,
    NoSuchValue,
}

/// A singleton instance of the DB. this lets us use the same db everywhere in the server
static DB_INSTANCE: Lazy<Mutex<AuthDatabase>> = Lazy::new(|| {
    let mut db = AuthDatabase::new();
    db.create_table("auth_grants");
    db.create_table("access_tokens");
    Mutex::new(db)
});

/// Get the active DB instance
pub fn get_db(
) -> Result<MutexGuard<'static, AuthDatabase>, PoisonError<MutexGuard<'static, AuthDatabase>>> {
    DB_INSTANCE.lock()
}

/// A fake DbTable that exists in memory. this is temporary and will eventually be replaced with a real db
pub struct DbTable {
    pub name: String,
    data: HashMap<String, String>,
}

impl DbTable {
    fn new(name: &str) -> Self {
        DbTable {
            name: name.into(),
            data: HashMap::new(),
        }
    }

    /// Insert the given key/value pair into this table
    pub fn insert(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }
    
    /// Remove a key/value pair from this table and returns the value of the key if it exists
    /// Returns `DatabaseError::NoSuchValue` if the key cannot be found
    pub fn consume(&mut self, key: &str) -> Result<String, DatabaseInternalError> {
        self.data.remove(key).ok_or_else(|| DatabaseInternalError::NoSuchValue)
    }
    
    /// Get a key if it exists.
    /// Returns `DatabaseError::NoSuchValue` if the key cannot be found
    pub fn get(&self, key: &str) -> Result<&String, DatabaseInternalError> {
        self.data.get(key).ok_or_else(|| DatabaseInternalError::NoSuchValue)
    }
}

/// A very simple in-memory AuthDatabase
pub struct AuthDatabase {
    tables: HashMap<String, DbTable>,
}

impl AuthDatabase {
    /// Create a new AuthDatabase with no tables
    fn new() -> Self {
        AuthDatabase {
            tables: HashMap::new(),
        }
    }

    /// Create a new empty table
    pub fn create_table(&mut self, name: &str) {
        self.tables.insert(name.into(), DbTable::new(name));
    }

    /// Get a table if it exists
    pub fn get_table(&self, name: &str) -> Result<&DbTable, DatabaseInternalError> {
        self.tables
            .get(name)
            .ok_or_else(|| DatabaseInternalError::NoSuchTable)
    }
}
