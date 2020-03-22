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

  DbExecutionError,

  DockerConnectionError,

  DockerExecutionError,

  InternalServerError,

  ResourceNotFound,
}

impl std::fmt::Display for ServerError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    match self {
      ServerError::DbConnectionError => write!(f, "{}", "Database Connection Error"),
      ServerError::DbExecutionError => write!(f, "{}", "Database Execution Error"),
      ServerError::DockerConnectionError => write!(f, "{}", "Docker Connection Error"),
      ServerError::DockerExecutionError => write!(f, "{}", "Docker Execution Error"),
      ServerError::InternalServerError => write!(f, "{}", "Internal Server Error"),
      ServerError::ResourceNotFound => write!(f, "{}", "Resource Not Found"),
    }
  }
}

impl ResponseError for ServerError {
  fn error_response(&self) -> HttpResponse {
    match self {
      ServerError::DbConnectionError => HttpResponse::InternalServerError().json(ErrorResponse {
        code: 100,
        message: "Internal Server Error",
      }),
      ServerError::DbExecutionError => HttpResponse::InternalServerError().json(ErrorResponse {
        code: 101,
        message: "Internal Server Error",
      }),
      ServerError::DockerConnectionError => {
        HttpResponse::InternalServerError().json(ErrorResponse {
          code: 200,
          message: "Internal Server Error",
        })
      }
      ServerError::DockerExecutionError => {
        HttpResponse::InternalServerError().json(ErrorResponse {
          code: 201,
          message: "Internal Server Error",
        })
      }
      ServerError::InternalServerError => HttpResponse::InternalServerError().json(ErrorResponse {
        code: 500,
        message: "Internal Server Error",
      }),
      ServerError::ResourceNotFound => HttpResponse::NotFound().json(ErrorResponse {
        code: 404,
        message: "Resource Not Found",
      }),
    }
  }
}
