use diesel::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, Queryable, Serialize)]
pub struct ExecutorResponse {
  name: String,
  tag: String,
}

pub fn fetch_executors(
  conn: &MysqlConnection,
) -> Result<Vec<ExecutorResponse>, diesel::result::Error> {
  use crate::schema::executors::dsl::*;

  let items = executors
    .select((name, tag))
    .load::<ExecutorResponse>(conn)?;

  Ok(items)
}
