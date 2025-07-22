use crate::{
    error::UserError,
    oauth::lib::{exchange_auth_grant, generate_auth_grant, AccessToken, AuthGrant},
};
use rocket::{serde::json::Json, Route};
use serde::{Deserialize, Serialize};

/// Start the OAuth 2.0 flow by requesting an authorization grant token that can be exchanged for an access token.
/// A client_id and redirect_uri are both required
#[get("/authorize?<client_id>&<redirect_uri>")]
fn auth(
    client_id: Option<String>,
    redirect_uri: Option<String>,
) -> Result<Json<AuthGrant>, UserError> {
    let (client_id, redirect_uri) = match (client_id, redirect_uri) {
        (Some(cid), Some(uri)) => (cid, uri),
        _ => {
            return Err(UserError {
                error: "Missing client_id or redirect_uri".into(),
                code: 422,
            })
        }
    };

    let grant = generate_auth_grant(client_id, redirect_uri).map_err(|_| UserError {
        error: "Failed to generate grant".into(),
        code: 500,
    })?;

    Ok(Json(grant))
}

/// The expected format of the payload for a `POST /token` request
///
/// code: an auth grant code returned from the `/authorize` endpoint
#[derive(Serialize, Deserialize)]
struct AuthGrantRequest {
    code: String,
}

/// Exchange an auth grant code returned from the `/authorize` endpoint for an access token.
///
/// This endpoint will return a user error if the auth grant can't be exchanged. This may happen if:
/// - the code is expired
/// - no such grant can be found
#[post("/token", data = "<data>")]
fn token(data: Json<AuthGrantRequest>) -> Result<Json<AccessToken>, UserError> {
    if let Ok(token) = exchange_auth_grant(data.code.clone()) {
        Ok(Json(token))
    } else {
        Err(UserError {
            error: "Failed to exchange auth grant".into(),
            code: 400,
        })
    }
}

pub fn routes() -> Vec<Route> {
    routes![auth, token]
}
