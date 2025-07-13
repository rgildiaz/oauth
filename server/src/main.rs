#[macro_use]
extern crate rocket;

use crate::oauth::{generate_auth_grant, AuthGrant};
use rocket::serde::json::Json;
use serde::Serialize;

mod oauth;

#[derive(Serialize)]
struct UserError {
    error: String,
    code: u16,
}

/// need basic webserver with
///     GET /login/oauth/auth
///         - return auth grant
///     POST /login/oauth/access
///         - exchange auth grant for access token
///     GET /{resource}
///         - use access token to get resource
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/// Start the OAuth 2.0 flow by requesting an authorization grant token that can be exchanged for an access token.
/// A client_id and redirect_uri are both required
#[get("/login/oauth/authorize?<client_id>&<redirect_uri>")]
fn auth(client_id: String, redirect_uri: String) -> Json<AuthGrant> {
    // TODO: check if the client ID exists in the db, reject if not
    let grant = generate_auth_grant(client_id, redirect_uri).unwrap();
    Json(grant)
}

#[get("/login/oauth/authorize")]
fn auth_missing_params() -> Json<UserError> {
    Json(UserError {
        error: "Unprocessable Entity: client_id and redirect_uri are required".into(),
        code: 422,
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, auth, auth_missing_params])
}
