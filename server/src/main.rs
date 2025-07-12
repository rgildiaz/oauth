use rocket::serde::json::Json;
use serde::Serialize;

#[macro_use] extern crate rocket;

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

#[derive(Serialize)]
struct AuthResponse {
    
}

#[get("/login/oauth/authorize")]
fn auth() -> Json<AuthResponse> {
    // check if the client ID exists in the db, reject if not
    Json(AuthResponse { })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, auth])
}