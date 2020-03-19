use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};

use crate::database;
use crate::services;

pub fn routings(app: &mut web::ServiceConfig) {
  app.service(executors);
}

#[actix_web::get("/executors")]
async fn executors(
  _: HttpRequest,
  db: web::Data<database::DbPool>,
) -> Result<impl Responder, Error> {
  let connection = database::extract_connection(db)?;

  let executors = web::block(move || services::executors::fetch_executors(&connection))
    .await
    .map_err(|_| HttpResponse::InternalServerError().finish())?;

  Ok(HttpResponse::Ok().json(executors))
}
