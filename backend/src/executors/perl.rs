use std::sync::Mutex;
use std::time::Duration;

use actix::*;
use actix_rt;
use actix_web::web::{Bytes, Data};
use futures::StreamExt;
use tokio::sync::mpsc::{channel, Sender};
use tokio::time::{interval_at, Instant};

use super::{to_bytes, Client, Event, ExecutorEvent, HeartbeatEvent};
use crate::database::instance::UpdateInstance;
use crate::database::DbExecutor;
use crate::docker::DockerExecutor;
use crate::models::{Executor, InstanceChangeset, InstanceResponse, InstanceStatus};
use crate::services;

pub struct PerlExecutor {
  db: Addr<DbExecutor>,
  docker: Addr<DockerExecutor>,
  clients: Vec<Sender<Bytes>>,
}

impl PerlExecutor {
  pub fn create(db: Addr<DbExecutor>, docker: Addr<DockerExecutor>) -> Data<Mutex<Self>> {
    let myself = Data::new(Mutex::new(PerlExecutor::new(db, docker)));

    PerlExecutor::spawn_ping(myself.clone());

    myself
  }

  fn new(db: Addr<DbExecutor>, docker: Addr<DockerExecutor>) -> Self {
    PerlExecutor {
      db,
      docker,
      clients: Vec::new(),
    }
  }

  fn spawn_ping(myself: Data<Mutex<Self>>) {
    actix_rt::spawn(async move {
      let mut task = interval_at(Instant::now(), Duration::from_secs(5));
      while let Some(_) = task.next().await {
        myself.lock().unwrap().remove_stale_clients();
      }
    })
  }

  fn remove_stale_clients(&mut self) {
    let mut lived_clients = Vec::new();

    for client in self.clients.iter() {
      let r = client.clone().try_send(to_bytes(HeartbeatEvent::create()));

      if let Ok(_) = r {
        lived_clients.push(client.clone());
      }
    }

    self.clients = lived_clients;
  }

  pub fn execute(&mut self, id: i64, instance: InstanceResponse, executor: Executor) -> Client {
    let (tx, rx) = channel(100);

    self.clients.push(tx.clone());

    self.spawn_task(tx.clone(), id, instance.clone(), executor.clone());

    Client(rx)
  }

  fn spawn_task(
    &mut self,
    tx: Sender<Bytes>,
    instance_id: i64,
    instance: InstanceResponse,
    executor: Executor,
  ) {
    let database = self.db.clone();
    let docker = self.docker.clone();

    actix_rt::spawn(async move {
      let r = services::executors::perl::execute(
        tx.clone(),
        docker.clone(),
        instance.clone(),
        executor.clone(),
      )
      .await;

      let _ = match r {
        Ok(value) => {
          database
            .send(UpdateInstance::new(
              instance_id,
              InstanceChangeset {
                title: None,
                status: Some(InstanceStatus::Success),
                result: Some(value.join("\n")),
              },
            ))
            .await
        }
        Err(_) => {
          database
            .send(UpdateInstance::new(
              instance_id,
              InstanceChangeset {
                title: None,
                status: Some(InstanceStatus::Failure),
                result: None,
              },
            ))
            .await
        }
      };

      // I want to close network connection by server, but I don't know how to do it...
      tx.clone()
        .try_send(to_bytes(ExecutorEvent::<String> {
          event: Event::Command,
          data: Some("system: close".to_owned()),
        }))
        .unwrap();
    })
  }
}
