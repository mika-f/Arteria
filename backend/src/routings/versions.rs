use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

use crate::models::Version;
use crate::DbPool;

#[derive(Debug, Serialize)]
struct PerlVersion {
  name: String,
  tag: String,
}

#[derive(Debug, Serialize)]
struct PerlVersions {
  versions: Vec<PerlVersion>,
}

#[actix_web::get("/versions")]
pub async fn versions(_: HttpRequest, db: web::Data<DbPool>) -> Result<impl Responder, Error> {
  let connection = db
    .into_inner()
    .get()
    .map_err(|_| HttpResponse::InternalServerError().finish())?;

  let versions = web::block(move || Version::all(&connection))
    .await
    .map(|item| {
      item
        .iter()
        .map(|w| PerlVersion {
          name: w.name.to_string(),
          tag: w.tag.to_string(),
        })
        .collect::<Vec<PerlVersion>>()
    })
    .map_err(|_| HttpResponse::InternalServerError().finish())?;

  Ok(HttpResponse::Ok().json(PerlVersions { versions }))
}
