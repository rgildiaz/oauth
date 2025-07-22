#[macro_use]
extern crate rocket;
mod db;
mod error;
mod oauth;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/// need basic webserver with
///     GET /login/oauth/auth
///         - return auth grant
///     POST /login/oauth/access
///         - exchange auth grant for access token
///     GET /{resource}
///         - use access token to get resource
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/login/oauth", oauth::routes())
}
