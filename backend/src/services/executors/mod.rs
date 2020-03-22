use actix::*;
use actix_web::Error;

use crate::database::executor::FetchExecutors;
use crate::database::DbExecutor;

use crate::models::ExecutorResponse;

pub mod perl;

pub async fn fetch_executors(database: Addr<DbExecutor>) -> Result<Vec<ExecutorResponse>, Error> {
  let items = database.send(FetchExecutors {}).await??;

  Ok(items)
}
