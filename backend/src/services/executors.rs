use diesel::prelude::*;
use serde::Serialize;

use crate::models::Executor;

#[derive(Clone, Debug, Serialize)]
pub struct ExecutorResponse {
  name: String,
  tag: String,
}

pub fn fetch_executors(
  conn: &MysqlConnection,
) -> Result<Vec<ExecutorResponse>, diesel::result::Error> {
  let versions = Executor::all(conn)?;
  let executors: Vec<ExecutorResponse> = versions
    .iter()
    .map(|w| ExecutorResponse {
      name: w.name.to_owned(),
      tag: w.tag.to_owned(),
    })
    .collect();

  Ok(executors)
}
