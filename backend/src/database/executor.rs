use actix::prelude::*;
use diesel::prelude::*;

use super::{extract_db_connection, DbExecutor};
use crate::errors::ServerError;
use crate::models::{Executor, ExecutorResponse};

#[derive(Debug)]
pub struct FetchExecutors;

impl Message for FetchExecutors {
  type Result = Result<Vec<ExecutorResponse>, ServerError>;
}

impl Handler<FetchExecutors> for DbExecutor {
  type Result = Result<Vec<ExecutorResponse>, ServerError>;

  fn handle(&mut self, _: FetchExecutors, _: &mut Self::Context) -> Self::Result {
    use crate::schema::executors::dsl::*;

    let connection = extract_db_connection(self)?;

    let items = executors
      .select((name, tag))
      .load::<ExecutorResponse>(&connection)
      .map_err(|_| ServerError::DbExecutionError)?;

    Ok(items)
  }
}

#[derive(Debug)]
pub struct FetchExecutorByTag(String);

impl FetchExecutorByTag {
  pub fn new(tag: String) -> Self {
    FetchExecutorByTag(tag)
  }
}

impl Message for FetchExecutorByTag {
  type Result = Result<Option<Executor>, ServerError>;
}

impl Handler<FetchExecutorByTag> for DbExecutor {
  type Result = Result<Option<Executor>, ServerError>;

  fn handle(&mut self, msg: FetchExecutorByTag, _: &mut Self::Context) -> Self::Result {
    use crate::schema::executors::dsl::*;

    let connection = extract_db_connection(self)?;

    let item = executors
      .filter(tag.eq(msg.0))
      .first(&connection)
      .optional()
      .map_err(|_| ServerError::DbExecutionError)?;

    Ok(item)
  }
}
