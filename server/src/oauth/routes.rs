use crate::{
    error::UserError,
    oauth::lib::{generate_auth_grant, AuthGrant},
};
use rocket::{serde::json::Json, Route};

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

pub fn routes() -> Vec<Route> {
    routes![auth]
}
