use actix::prelude::*;
use diesel::prelude::*;

use super::{extract_db_connection, DbExecutor};
use crate::errors::ServerError;
use crate::models::{DependencySlim, NewDependency};

#[derive(Debug)]
pub struct FetchDependenciesByInstanceId(i64);

impl FetchDependenciesByInstanceId {
  pub fn new(instance_id: i64) -> Self {
    FetchDependenciesByInstanceId(instance_id)
  }
}

impl Message for FetchDependenciesByInstanceId {
  type Result = Result<Vec<DependencySlim>, ServerError>;
}

impl Handler<FetchDependenciesByInstanceId> for DbExecutor {
  type Result = Result<Vec<DependencySlim>, ServerError>;

  fn handle(&mut self, msg: FetchDependenciesByInstanceId, _: &mut Self::Context) -> Self::Result {
    use crate::schema::dependencies::dsl::*;

    let connection = extract_db_connection(self)?;

    let items = dependencies
      .filter(instance_id.eq(msg.0))
      .select(name_with_version)
      .load::<String>(&connection)
      .map(|w| {
        w.iter()
          .map(|v| DependencySlim {
            name_with_version: v.to_owned(),
          })
          .collect()
      })
      .map_err(|_| ServerError::DbExecutionError)?;

    Ok(items)
  }
}

#[derive(Debug)]
pub struct BulkInsertDependencies(Vec<NewDependency>);

impl BulkInsertDependencies {
  pub fn new(dependencies: Vec<NewDependency>) -> Self {
    BulkInsertDependencies(dependencies)
  }
}

impl Message for BulkInsertDependencies {
  type Result = Result<(), ServerError>;
}

impl Handler<BulkInsertDependencies> for DbExecutor {
  type Result = Result<(), ServerError>;

  fn handle(&mut self, msg: BulkInsertDependencies, _: &mut Self::Context) -> Self::Result {
    use crate::schema::dependencies::dsl::*;

    let connection = extract_db_connection(self)?;

    diesel::insert_into(dependencies)
      .values(msg.0)
      .execute(&connection)
      .map_err(|_| ServerError::DbExecutionError)?;

    Ok(())
  }
}
