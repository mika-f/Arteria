use actix::prelude::*;
use bollard::Docker;

pub struct DockerExecutor(pub Docker);

impl Actor for DockerExecutor {
  type Context = SyncContext<Self>;
}

// Run
pub struct ExecuteContainer {}

impl Message for ExecuteContainer {
  type Result = Result<Vec<String>, ()>;
}
