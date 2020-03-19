use actix_http::Response;
use actix_web::HttpResponse;

use serde::Serialize;

pub mod executors;
pub mod meta;
pub mod root;

const ERR_DATABASE_CONNECTION: u16 = 101;

#[derive(Clone, Serialize)]
pub struct ErrorResponse<'a> {
  code: u16,
  message: &'a str,
}

impl<'a> ErrorResponse<'a> {
  pub fn db_connection_error() -> Response {
    HttpResponse::InternalServerError().json(ErrorResponse {
      code: ERR_DATABASE_CONNECTION,
      message: "Failed to connect to backend database",
    })
  }
}
