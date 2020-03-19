use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};

use crate::services;
use crate::web as arteria_web;
use crate::DbPool;

pub fn routings(app: &mut web::ServiceConfig) {
  app.service(executors);
}

#[actix_web::get("/executors")]
async fn executors(_: HttpRequest, db: web::Data<DbPool>) -> Result<impl Responder, Error> {
  let connection = db
    .get()
    .map_err(|_| arteria_web::ErrorResponse::db_connection_error())?;

  let executors = web::block(move || services::executors::fetch_executors(&connection))
    .await
    .map_err(|_| HttpResponse::InternalServerError().finish())?;

  Ok(HttpResponse::Ok().json(executors))
}
