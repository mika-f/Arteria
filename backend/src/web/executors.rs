use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};

use crate::services;
use crate::AppState;

pub fn routings(app: &mut web::ServiceConfig) {
  app.service(executors);
}

#[actix_web::get("/executors")]
async fn executors(_: HttpRequest, state: web::Data<AppState>) -> Result<impl Responder, Error> {
  let db = state.db.clone();
  let executors = services::executors::fetch_executors(db).await?;

  Ok(HttpResponse::Ok().json(executors))
}
