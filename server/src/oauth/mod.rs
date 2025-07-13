use rocket::time::{self, Duration};
use serde::Serialize;

/// The time after which an auth grant expires, calculated from the time of grant generation
const GRANT_EXPIRATION: Duration = Duration::minutes(15);

#[derive(Serialize, Debug)]
pub struct AuthGrant {
    code: String,
    client_id: String,
    redirect_uri: String,
    expires_at: String,
}

#[derive(Debug)]
pub enum GrantError {
    ExpirationCalculationFailed,
}

/// Generate an `AuthGrant` that expires in 15 minutes.
pub fn generate_auth_grant(
    client_id: String,
    redirect_uri: String,
) -> Result<AuthGrant, GrantError> {
    let code = "auth_code".into();
    let expires_at = time::UtcDateTime::now()
        .checked_add(GRANT_EXPIRATION)
        .ok_or_else(|| GrantError::ExpirationCalculationFailed)?
        .unix_timestamp()
        .to_string();

    let grant = AuthGrant {
        code,
        client_id,
        redirect_uri,
        expires_at,
    };

    Ok(grant)
}
