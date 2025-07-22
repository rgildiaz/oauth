use crate::db::get_auth_db;
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
pub enum AuthError {
    DatabaseError,
    ExpirationCalculationFailed,
}

/// Generate an `AuthGrant` that expires in 15 minutes.
pub fn generate_auth_grant(
    client_id: String,
    redirect_uri: String,
) -> Result<AuthGrant, AuthError> {
    let code = "auth_code";
    let expires_at = UtcDateTime::now()
        .checked_add(AUTH_CODE_LIFETIME)
        .ok_or_else(|| AuthError::ExpirationCalculationFailed)?
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
            return Err(AuthError::DatabaseError);
        }
    }

    Ok(grant)
}

#[derive(Serialize)]
pub struct AccessToken {
    pub token: String,
}

/// Exchange an auth grant code for an access token
pub fn exchange_auth_grant(code: String) -> Result<AccessToken, AuthError> {
    // TODO: prob a good idea to store the db on a struct
    match get_auth_db() {
        Ok(mut db) => {
            let _ = db.remove(code);
            Ok(AccessToken {
                token: "access_token".into(),
            })
        }
        Err(e) => {
            eprintln!("Error while getting database: {}", e);
            Err(AuthError::DatabaseError)
        }
    }
}
