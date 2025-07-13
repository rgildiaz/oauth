use rocket::{
    http::Status,
    response::{Responder, Response, Result},
    Request,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UserError {
    pub error: String,
    pub code: u16,
}

/// Format the error response when an error happens
impl<'r> Responder<'r, 'static> for UserError {
    fn respond_to(self, _: &'r Request<'_>) -> Result<'static> {
        Response::build()
            .status(Status::new(self.code))
            .sized_body(self.error.len(), std::io::Cursor::new(self.error))
            .ok()
    }
}
