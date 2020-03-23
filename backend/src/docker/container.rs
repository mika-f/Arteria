use std::collections::HashMap;

use actix::prelude::*;
use actix_web::web::Bytes;
use bollard::container::*;
use futures::stream::StreamExt;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::Sender;

use crate::docker::DockerExecutor;
use crate::errors::ServerError;
use crate::executors::{to_bytes, Event, ExecutorEvent};

// Run
#[derive(Debug)]
pub struct ExecuteContainer {
  pub name: String,
  pub image: String,
  pub cmd: Vec<String>,

  // optional
  pub bindings: Option<Vec<String>>,
  pub cpus: Option<u64>,
  pub env: Option<Vec<String>>,
  pub logger: Option<Sender<Bytes>>,
  pub memory: Option<u64>,
  pub network_mode: Option<String>,
  pub ulimits: Option<Vec<HashMap<String, String>>>,
  pub working_dir: Option<String>,
}

impl Message for ExecuteContainer {
  type Result = Result<Vec<String>, ServerError>;
}

impl Handler<ExecuteContainer> for DockerExecutor {
  type Result = Result<Vec<String>, ServerError>;

  fn handle(&mut self, msg: ExecuteContainer, _: &mut Self::Context) -> Self::Result {
    let docker = self.0.clone();
    let mut runtime = Runtime::new().unwrap();

    let vec = runtime.block_on(async {
      let container = docker
        .clone()
        .create_container(
          Some(CreateContainerOptions { name: msg.name }),
          Config::<String> {
            attach_stdin: Some(true),
            attach_stdout: Some(true),
            attach_stderr: Some(true),
            cmd: Some(msg.cmd),
            image: Some(msg.image),

            // optionals
            env: msg.env,
            working_dir: msg.working_dir,
            host_config: Some(HostConfig::<String> {
              auto_remove: Some(true),
              binds: msg.bindings,
              memory: msg.memory,
              nano_cpus: msg.cpus,
              // ulimits: msg.ulimits,
              ..Default::default()
            }),
            ..Default::default()
          },
        )
        .await;

      let container = match container {
        Ok(container) => container,
        Err(_) => return Err(ServerError::DockerExecutionError),
      };
      let container_id = container.id.clone();

      docker
        .clone()
        .start_container(&container_id, None::<StartContainerOptions<String>>)
        .await
        .map_err(|e| {
          println!("Failed to start container because {}", e);
          ServerError::DockerExecutionError
        })?;

      let stream = &mut docker.clone().logs(
        &container_id,
        Some(LogsOptions {
          follow: true,
          stdout: true,
          stderr: true,
          tail: "all".to_owned(),
          ..Default::default()
        }),
      );

      let mut logs = Vec::new();

      fn send_logs(tx: Sender<Bytes>, log: String) {
        match tx.clone().try_send(to_bytes(ExecutorEvent {
          event: Event::Message,
          data: Some(log),
        })) {
          Ok(_) => {}
          Err(_) => {}
        }
      };

      let has_logger = msg.logger.clone().is_some();

      while let Some(value) = &stream.next().await {
        match value {
          Ok(log) => match log {
            LogOutput::StdOut { message } => {
              logs.push(format!("stdout: {}", message));
              if has_logger {
                send_logs(
                  msg.logger.clone().unwrap().clone(),
                  format!("stdout: {}", message),
                );
              }
            }
            LogOutput::StdErr { message } => {
              logs.push(format!("stderr: {}", message));
              if has_logger {
                send_logs(
                  msg.logger.clone().unwrap().clone(),
                  format!("stderr: {}", message),
                );
              }
            }
            LogOutput::Console { message } => {
              logs.push(format!("stdout: {}", message));
              if has_logger {
                send_logs(
                  msg.logger.clone().unwrap().clone(),
                  format!("stdout: {}", message),
                );
              }
            }
            LogOutput::StdIn { message: _ } => {}
          },
          Err(_) => return Err(ServerError::DockerExecutionError),
        }
      }

      let _ = docker.clone().clone().wait_container(
        &container_id,
        Some(WaitContainerOptions {
          condition: "not-running",
        }),
      );

      Ok(logs)
    })?;

    dbg!(&vec);

    Ok(vec)
  }
}
