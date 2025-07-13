use crate::oauth::lib::{generate_auth_grant, AuthGrant};
use rocket::{serde::json::Json, Route};
use serde::Serialize;

#[derive(Serialize)]
struct UserError {
    error: String,
    code: u16,
}

/// Start the OAuth 2.0 flow by requesting an authorization grant token that can be exchanged for an access token.
/// A client_id and redirect_uri are both required
#[get("/authorize?<client_id>&<redirect_uri>")]
fn auth(client_id: String, redirect_uri: String) -> Json<AuthGrant> {
    // TODO: check if the client ID exists in the db, reject if not
    let grant = generate_auth_grant(client_id, redirect_uri).unwrap();
    Json(grant)
}

#[get("/authorize")]
fn auth_missing_params() -> Json<UserError> {
    Json(UserError {
        error: "Unprocessable Entity: client_id and redirect_uri are required".into(),
        code: 422,
    })
}

pub fn routes() -> Vec<Route> {
    routes![auth, auth_missing_params]
}