use actix::prelude::*;
use diesel::prelude::*;

use super::{extract_db_connection, DbExecutor};
use crate::errors::ServerError;
use crate::models::{InstanceRequest, InstanceStatus, NewDependency, NewFile, NewInstance};

#[derive(Debug)]
pub struct CreateNewInstance(InstanceRequest);

impl CreateNewInstance {
  pub fn new(instance: InstanceRequest) -> Self {
    CreateNewInstance(instance)
  }
}

impl Message for CreateNewInstance {
  type Result = Result<i64, ServerError>;
}

impl Handler<CreateNewInstance> for DbExecutor {
  type Result = Result<i64, ServerError>;

  fn handle(&mut self, msg: CreateNewInstance, _: &mut Self::Context) -> Self::Result {
    let connection = extract_db_connection(self)?;

    let instance_id = connection
      .transaction::<_, diesel::result::Error, _>(|| {
        use crate::schema::dependencies::dsl as d_dsl;
        use crate::schema::executors::dsl as e_dsl;
        use crate::schema::files::dsl as f_dsl;
        use crate::schema::instances::dsl as i_dsl;

        let obj = msg.0;

        // find valid executor
        let executor_id = e_dsl::executors
          .select(e_dsl::id)
          .filter(e_dsl::tag.eq(obj.executor))
          .first(&connection)?;

        // insert instance and fetch its id
        diesel::insert_into(i_dsl::instances)
          .values(NewInstance {
            title: obj.title,
            executor_id,
            status: InstanceStatus::Running,
            result: None,
          })
          .execute(&connection)?;

        let instance_id = i_dsl::instances
          .select(i_dsl::id)
          .order(i_dsl::id.desc())
          .limit(1)
          .get_result::<i64>(&connection)?;

        // bulk insert dependencies and files
        diesel::insert_into(d_dsl::dependencies)
          .values(
            obj
              .dependencies
              .iter()
              .map(|w| NewDependency {
                name_with_version: w.name_with_version.to_owned(),
                instance_id,
              })
              .collect::<Vec<NewDependency>>(),
          )
          .execute(&connection)?;

        diesel::insert_into(f_dsl::files)
          .values(
            obj
              .files
              .iter()
              .map(|w| NewFile {
                title: w.title.to_owned(),
                content: w.content.to_owned(),
                instance_id,
              })
              .collect::<Vec<NewFile>>(),
          )
          .execute(&connection)?;

        Ok(instance_id)
      })
      .map_err(|_| ServerError::DbExecutionError)?;

    Ok(instance_id)
  }
}
