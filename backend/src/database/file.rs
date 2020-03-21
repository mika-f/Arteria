use actix::prelude::*;
use diesel::prelude::*;

use super::{extract_db_connection, DbExecutor};
use crate::errors::ServerError;
use crate::models::{FileSlim, NewFile};

#[derive(Debug)]
pub struct FetchFilesByInstanceId(i64);

impl FetchFilesByInstanceId {
  pub fn new(instance_id: i64) -> Self {
    FetchFilesByInstanceId(instance_id)
  }
}

impl Message for FetchFilesByInstanceId {
  type Result = Result<Vec<FileSlim>, ServerError>;
}

impl Handler<FetchFilesByInstanceId> for DbExecutor {
  type Result = Result<Vec<FileSlim>, ServerError>;

  fn handle(&mut self, msg: FetchFilesByInstanceId, _: &mut Self::Context) -> Self::Result {
    use crate::schema::files::dsl::*;

    let connection = extract_db_connection(self)?;

    let items = files
      .filter(instance_id.eq(msg.0))
      .select((title, content))
      .load::<FileSlim>(&connection)
      .map_err(|_| ServerError::DbExecutionError)?;

    Ok(items)
  }
}

#[derive(Debug)]
pub struct BulkInsertFiles(Vec<NewFile>);

impl BulkInsertFiles {
  pub fn new(files: Vec<NewFile>) -> Self {
    BulkInsertFiles(files)
  }
}

impl Message for BulkInsertFiles {
  type Result = Result<(), ServerError>;
}

impl Handler<BulkInsertFiles> for DbExecutor {
  type Result = Result<(), ServerError>;

  fn handle(&mut self, msg: BulkInsertFiles, _: &mut Self::Context) -> Self::Result {
    use crate::schema::files::dsl::*;

    let connection = extract_db_connection(self)?;

    diesel::insert_into(files)
      .values(msg.0)
      .execute(&connection)
      .map_err(|_| ServerError::DbExecutionError)?;

    Ok(())
  }
}
