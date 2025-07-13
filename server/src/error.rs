use rocket::response::{Responder, Response};
use rocket::http::Status;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UserError {
    pub error: String,
    pub code: u16,
}

impl<'r> Responder<'r, 'static> for UserError {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        Response::build()
            .status(Status::new(self.code))
            .sized_body(self.error.len(), std::io::Cursor::new(self.error))
            .ok()
    }
}
