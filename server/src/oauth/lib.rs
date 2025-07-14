use crate::db::{get_auth_db};
use rocket::time::{Duration, UtcDateTime};
use serde::Serialize;

/// The time after which an auth grant expires, calculated from the time of grant generation
const AUTH_CODE_LIFETIME: Duration = Duration::minutes(15);

#[derive(Serialize, Debug, Clone)]
pub struct AuthGrant {
    pub code: String,
    pub client_id: String,
    pub redirect_uri: String,
    pub expires_at: String,
}

#[derive(Debug)]
pub enum GrantError {
    DatabaseError,
    ExpirationCalculationFailed,
}

/// Generate an `AuthGrant` that expires in 15 minutes.
pub fn generate_auth_grant(
    client_id: String,
    redirect_uri: String,
) -> Result<AuthGrant, GrantError> {
    let code = "auth_code";
    let expires_at = UtcDateTime::now()
        .checked_add(AUTH_CODE_LIFETIME)
        .ok_or_else(|| GrantError::ExpirationCalculationFailed)?
        .unix_timestamp()
        .to_string();

    let grant = AuthGrant {
        code: String::from(code),
        client_id,
        redirect_uri,
        expires_at,
    };

    // save the grant to the db
    match get_auth_db() {
        Ok(mut db) => {
            db.insert(&grant);
        }
        Err(e) => {
            eprintln!("Error while getting database: {}", e);
            return Err(GrantError::DatabaseError);
        }
    }

    Ok(grant)
}
