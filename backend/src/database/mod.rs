use actix::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::*;

use crate::errors::ServerError;

pub mod dependency;
pub mod executor;
pub mod file;
pub mod instance;
pub mod transaction;

pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
pub type PooledConnection = r2d2::PooledConnection<ConnectionManager<MysqlConnection>>;

pub struct DbExecutor(pub DbPool);

impl Actor for DbExecutor {
  type Context = SyncContext<Self>;
}

pub fn extract_db_connection<'a>(
  executor: &mut DbExecutor,
) -> Result<PooledConnection, ServerError> {
  let connection = executor
    .0
    .get()
    .map_err(|_| ServerError::DbConnectionError)?;

  Ok(connection)
}
