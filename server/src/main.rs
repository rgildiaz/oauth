#[macro_use]
extern crate rocket;
use crate::oauth::{generate_auth_grant, AuthGrant};
use rocket::serde::json::Json;

mod oauth;

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

#[get("/login/oauth/authorize")]
fn auth() -> Json<AuthGrant> {
    // TODO: check if the client ID exists in the db, reject if not
    let client_id = "client_id".into();
    let redirect_uri = "https://redirect.uri".into();
    let grant = generate_auth_grant(client_id, redirect_uri).unwrap();
    Json(grant)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, auth])
}
