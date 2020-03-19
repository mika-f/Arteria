use actix_web::web::Data;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use crate::errors;

pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
pub type PooledConnection = r2d2::PooledConnection<ConnectionManager<MysqlConnection>>;

pub fn extract_connection(db: Data<DbPool>) -> Result<PooledConnection, errors::ServerError> {
  let connection = db
    .get()
    .map_err(|_| errors::ServerError::DbConnectionError)?;

  Ok(connection)
}
