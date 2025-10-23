use crate::{
    db::{get_auth_grant_db, get_token_db, TokenHash},
    oauth::lib::hash::HashErr,
};
use rocket::time::{Duration, UtcDateTime};
use serde::Serialize;
mod hash;
use hash::hash;

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
    InternalError,
    NoAuthGrantFound,
    NoAccessTokenFound,
    AccessTokenExpired,
}

impl From<HashErr> for AuthError {
    fn from(_: HashErr) -> AuthError {
        AuthError::InternalError
    }
}

/// Generate an `AuthGrant` that expires in 15 minutes.
pub fn generate_auth_grant(
    client_id: String,
    redirect_uri: String,
) -> Result<AuthGrant, AuthError> {
    let code = "auth_code";
    let expires_at = UtcDateTime::now()
        .checked_add(AUTH_CODE_LIFETIME)
        .ok_or(AuthError::InternalError)?
        .unix_timestamp()
        .to_string();

    let grant = AuthGrant {
        code: String::from(code),
        client_id,
        redirect_uri,
        expires_at,
    };

    // save the grant to the db
    let mut db = get_auth_grant_db().map_err(|_| AuthError::DatabaseError)?;
    db.insert(grant.code.clone(), grant.clone());
    Ok(grant)
}

/// The time after which an access token expires
const TOKEN_LIFETIME: Duration = Duration::hours(24);

#[derive(Serialize, Debug, Clone)]
pub struct AccessToken {
    pub token: String,
    pub expires_at: i64,
}

/// Exchange an auth grant code for an access token
pub fn exchange_auth_grant(code: String) -> Result<AccessToken, AuthError> {
    let mut auth_grant_db = get_auth_grant_db().map_err(|_| AuthError::DatabaseError)?;
    let mut token_db = get_token_db().map_err(|_| AuthError::DatabaseError)?;

    auth_grant_db
        .remove(code)
        .ok_or(AuthError::NoAuthGrantFound)?;

    let token = generate_token().unwrap();
    let hash = hash(token.token.clone())?;
    token_db.insert(
        hash,
        TokenHash {
            expires_at: token.expires_at.clone(),
        },
    );
    Ok(token)
}

fn generate_token() -> Result<AccessToken, AuthError> {
    let expires_at = UtcDateTime::now()
        .checked_add(TOKEN_LIFETIME)
        .ok_or(AuthError::InternalError)?
        .unix_timestamp();

    Ok(AccessToken {
        token: "token".into(),
        expires_at,
    })
}

pub fn check_token(token: String) -> Result<String, AuthError> {
    let mut token_db = get_token_db().map_err(|_| AuthError::DatabaseError)?;

    let token_hash = hash(token.clone())?;
    println!("looking for {token_hash}");

    let t = token_db
        .get(token_hash.to_string())
        .ok_or(AuthError::NoAccessTokenFound)?;

    if t.expires_at < UtcDateTime::now().unix_timestamp() {
        return Ok(format!("{} is valid!!", token));
    }

    // remove the token once it's expired
    // NOTE: with this system, stale tokens are only removed if they are queried for. Probably a better way to auto-remove stale tokens
    token_db.remove(token);
    Err(AuthError::AccessTokenExpired)
}
