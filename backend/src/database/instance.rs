use actix::prelude::*;
use diesel::prelude::*;

use super::{extract_db_connection, DbExecutor};
use crate::errors::ServerError;
use crate::models::{Executor, Instance, InstanceWithExecutor};

#[derive(Debug)]
pub struct FetchInstance(i64);

impl FetchInstance {
  pub fn new(instance_id: i64) -> Self {
    FetchInstance(instance_id)
  }
}

impl Message for FetchInstance {
  type Result = Result<Option<InstanceWithExecutor>, ServerError>;
}

impl Handler<FetchInstance> for DbExecutor {
  type Result = Result<Option<InstanceWithExecutor>, ServerError>;

  fn handle(&mut self, msg: FetchInstance, _: &mut Self::Context) -> Self::Result {
    use crate::schema::executors::dsl::*;
    use crate::schema::instances::dsl::*;

    let connection = extract_db_connection(self)?;

    let items = instances
      .find(msg.0)
      .inner_join(executors)
      .get_result::<(Instance, Executor)>(&connection)
      .optional()
      .map_err(|_| ServerError::DbExecutionError)?;

    let (instance, executor) = match items {
      Some(items) => items,
      None => return Ok(None),
    };

    Ok(Some(InstanceWithExecutor {
      title: instance.title,
      executor: executor.name,
      status: instance.status,
      result: instance.result,
    }))
  }
}
