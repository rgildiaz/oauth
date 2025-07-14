//! We need a database in order to store the following tables:
//!     - the active Auth Grants (that can be exchanged for access tokens)
//!     - the active access tokens

use crate::oauth::lib::AuthGrant;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard, PoisonError};

#[derive(Debug)]
pub enum DatabaseInternalError {
    NoSuchValue,
}

/// A singleton instance of the DB
static DB_INSTANCE: Lazy<Mutex<AuthGrantDb>> = Lazy::new(|| Mutex::new(AuthGrantDb::new()));

/// Get the active DB instance
pub fn get_auth_db(
) -> Result<MutexGuard<'static, AuthGrantDb>, PoisonError<MutexGuard<'static, AuthGrantDb>>> {
    DB_INSTANCE.lock()
}

/// A very simple in-memory db to store AuthGrants
pub struct AuthGrantDb {
    active_grants: HashMap<String, AuthGrant>,
}

impl AuthGrantDb {
    /// Create an empty AuthGrantDb
    fn new() -> Self {
        Self {
            active_grants: HashMap::new(),
        }
    }

    /// Add an AuthGrant to the db
    pub fn insert(&mut self, grant: &AuthGrant) {
        dbg!(&grant);
        self.active_grants.insert(grant.code.clone(), grant.clone());
    }

    /// Remove an AuthGrant from the db if it exists
    pub fn remove(&mut self, code: String) -> Result<AuthGrant, DatabaseInternalError> {
        let grant = self
            .active_grants
            .remove(&code)
            .ok_or_else(|| DatabaseInternalError::NoSuchValue);
        dbg!(&grant);
        grant
    }
}
