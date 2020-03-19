use actix_http::ResponseError;
use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponse<'a> {
  code: i16,
  message: &'a str,
}

#[derive(Debug, Serialize)]
pub enum ServerError {
  DbConnectionError,

  InternalServerError,
}

impl std::fmt::Display for ServerError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    match self {
      ServerError::DbConnectionError => write!(f, "{}", "Database Connection Error"),
      ServerError::InternalServerError => write!(f, "{}", "Internal Server Error"),
    }
  }
}

impl ResponseError for ServerError {
  fn error_response(&self) -> HttpResponse {
    match self {
      ServerError::DbConnectionError => HttpResponse::InternalServerError().json(ErrorResponse {
        code: 100,
        message: "Failed to connect to database",
      }),
      ServerError::InternalServerError => HttpResponse::InternalServerError().json(ErrorResponse {
        code: 500,
        message: "Internal Server Error",
      }),
    }
  }
}
