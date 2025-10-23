//! We need a database in order to store the following tables:
//!     - the active Auth Grants (that can be exchanged for access tokens)
//!     - the active access tokens

use crate::oauth::lib::AuthGrant;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Mutex, MutexGuard, PoisonError};

/// A singleton instance of the DB
static GRANT_DB: Lazy<Mutex<DummyDB<AuthGrant>>> = Lazy::new(|| Mutex::new(DummyDB::new()));

static TOKEN_DB: Lazy<Mutex<DummyDB<TokenHash>>> = Lazy::new(|| Mutex::new(DummyDB::new()));

/// Get the active auth grant DB instance
pub fn get_auth_grant_db() -> Result<
    MutexGuard<'static, DummyDB<AuthGrant>>,
    PoisonError<MutexGuard<'static, DummyDB<AuthGrant>>>,
> {
    GRANT_DB.lock()
}

#[derive(Debug)]
pub struct TokenHash {
    pub expires_at: i64,
}

/// Get the active token db
pub fn get_token_db() -> Result<
    MutexGuard<'static, DummyDB<TokenHash>>,
    PoisonError<MutexGuard<'static, DummyDB<TokenHash>>>,
> {
    TOKEN_DB.lock()
}

/// A simple in-memory database for demonstration purposes
pub struct DummyDB<T> {
    table: HashMap<String, T>,
}

impl<T: Debug> DummyDB<T> {
    fn new() -> Self {
        DummyDB {
            table: HashMap::new(),
        }
    }

    /// Add an entry to the db
    pub fn insert(&mut self, pk: String, data: T) {
        self.table.insert(pk.clone(), data);
        println!("after insert");
        dbg!(&self.table);
    }

    /// Remove an entry from the db.
    pub fn remove(&mut self, pk: String) -> Option<T> {
        let v = self.table.remove(&pk);
        println!("after remove");
        dbg!(&self.table);
        v
    }

    /// Get an entry from the db.
    pub fn get(&self, pk: String) -> Option<&T> {
        println!("get");
        dbg!(&self.table);
        self.table.get(&pk)
    }
}
