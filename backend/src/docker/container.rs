use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use actix::prelude::*;
use actix_web::web::Bytes;
use bollard::container::*;
use futures::stream::StreamExt;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::Sender;
use tokio::time::delay_for;

use crate::docker::DockerExecutor;
use crate::errors::ServerError;
use crate::executors::{to_bytes, Event, ExecutorEvent};

// Run
#[derive(Clone, Debug)]
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
  pub runtime: Option<String>,
  pub timeout: Option<u64>,
  pub ulimits: Option<Vec<HashMap<String, String>>>,
  pub user: Option<String>,
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
      let mut logs = Vec::new();

      fn send_logs(logs: &mut Vec<String>, tx: Option<Sender<Bytes>>, log: String) {
        logs.push(log.to_owned());

        if tx.is_some() {
          match tx.unwrap().clone().try_send(to_bytes(ExecutorEvent {
            event: Event::Message,
            data: Some(log.to_owned()),
          })) {
            Ok(_) => {}
            Err(_) => {}
          }
        }
      };

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
            user: msg.user,

            // optionals
            env: msg.env,
            working_dir: msg.working_dir,
            host_config: Some(HostConfig::<String> {
              auto_remove: Some(true),
              binds: msg.bindings,
              runtime: msg.runtime,
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
      let container_id = Arc::new(container.id.clone());

      docker
        .clone()
        .start_container(&container_id, None::<StartContainerOptions<String>>)
        .await
        .map_err(|e| {
          println!("Failed to start container because {}", e);
          ServerError::DockerExecutionError
        })?;

      if msg.timeout.is_some() {
        let timeout = msg.timeout.unwrap().clone();
        let docker = docker.clone();
        let container_id = container_id.clone();

        // create a new thread as non-blocking background worker
        tokio::spawn(async move {
          delay_for(Duration::from_secs(timeout)).await;

          let _ = docker
            .clone()
            .kill_container(&container_id, None::<KillContainerOptions<String>>)
            .await
            .map_err(|e| {
              println!("Failed to kill container because {}", e);
              ServerError::DockerExecutionError
            });
        });
      }

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

      while let Some(value) = &stream.next().await {
        match value {
          Ok(log) => match log {
            LogOutput::StdOut { message } => {
              send_logs(
                &mut logs,
                msg.logger.clone(),
                format!("stdout: {}", message),
              );
            }
            LogOutput::StdErr { message } => {
              send_logs(
                &mut logs,
                msg.logger.clone(),
                format!("stderr: {}", message),
              );
            }
            LogOutput::Console { message } => {
              send_logs(
                &mut logs,
                msg.logger.clone(),
                format!("stdout: {}", message),
              );
            }
            LogOutput::StdIn { message: _ } => {}
          },
          Err(_) => return Err(ServerError::DockerExecutionError),
        }
      }

      let stream = &mut docker.clone().wait_container(
        &container_id,
        Some(WaitContainerOptions {
          condition: "not-running",
        }),
      );

      // check exit code of container
      match &stream.next().await {
        Some(value) => match value {
          Ok(value) => match value.status_code {
            // SIG-KILL
            137 => send_logs(
              &mut logs,
              msg.logger.clone(),
              "stderr: container killed by Arteria".to_owned(),
            ),
            _ => {}
          },
          Err(_) => {}
        },
        None => {}
      };

      Ok(logs)
    })?;

    Ok(vec)
  }
}
