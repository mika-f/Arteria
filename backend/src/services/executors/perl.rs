use std::error;
use std::fs;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use actix::prelude::*;
use actix_web::web::Bytes;
use tokio::sync::mpsc::Sender;
use uuid::Uuid;

use crate::dirs;
use crate::docker::container::ExecuteContainer;
use crate::docker::DockerExecutor;
use crate::executors::{to_bytes, Event, ExecutorEvent};
use crate::models::{Executor, InstanceResponse};

pub async fn execute(
  tx: Sender<Bytes>,
  docker: Addr<DockerExecutor>,
  instance: InstanceResponse,
  executor: Executor,
) -> Result<Vec<String>, ()> {
  let uuid = Uuid::new_v4();
  let path = create_project_directory(uuid).unwrap();
  let mut logs = Vec::new();

  create_deps_file(&path, &instance).unwrap();

  for log in execute_installer(tx.clone(), docker.clone(), &path, &instance)
    .await
    .unwrap()
    .iter()
  {
    logs.push(log.to_owned());
    safe_send(tx.clone(), Event::Message, Some(log.to_owned())).unwrap();
  }

  create_project_file(&path, &instance).unwrap();

  for log in execute_executor(
    uuid,
    &path,
    tx.clone(),
    docker.clone(),
    &instance,
    &executor,
  )
  .await
  .unwrap()
  .iter()
  {
    logs.push(log.to_owned());
    safe_send(tx.clone(), Event::Message, Some(log.to_owned())).unwrap();
  }

  cleanup_project_directory(&path).unwrap();

  Ok(logs)
}

fn create_project_directory(uuid: Uuid) -> Result<PathBuf, Box<dyn error::Error>> {
  let mut project_dir = dirs::temp_dir();
  project_dir.push(uuid.to_string());

  fs::create_dir(&project_dir)?;

  Ok(project_dir)
}

fn create_deps_file(
  path: &PathBuf,
  instance: &InstanceResponse,
) -> Result<(), Box<dyn error::Error>> {
  if instance.dependencies.len() == 0 {
    return Ok(());
  }

  let mut cpanfile = PathBuf::from(path);
  cpanfile.push("cpanfile");

  let mut cpanfile = BufWriter::new(fs::File::create(cpanfile)?);
  for dep in instance.dependencies.iter() {
    let module = dep.name_with_version.split("@").collect::<Vec<&str>>();
    if module.len() == 2 {
      cpanfile.write(&format!("requires '{}', '{}';\n", module[0], module[1]).as_bytes())?;
    } else {
      cpanfile.write(&format!("requires '{}';\n", module[0]).as_bytes())?;
    }
  }

  cpanfile.flush()?;

  Ok(())
}

fn create_project_file(
  path: &PathBuf,
  instance: &InstanceResponse,
) -> Result<(), Box<dyn error::Error>> {
  if instance.files.len() == 0 {
    // ????????
    return Ok(());
  }

  // Should I create files into container?
  for file in instance.files.iter() {
    let mut to = PathBuf::from(path);
    to.push(file.title.to_owned());

    let mut source = BufWriter::new(fs::File::create(to)?);
    source.write(&format!("{}\n", file.content).as_bytes())?;

    source.flush()?;
  }

  Ok(())
}

fn cleanup_project_directory(path: &PathBuf) -> Result<(), Box<dyn error::Error>> {
  fs::remove_dir_all(path)?;

  Ok(())
}

async fn execute_installer(
  tx: Sender<Bytes>,
  docker: Addr<DockerExecutor>,
  path: &PathBuf,
  instance: &InstanceResponse,
) -> Result<Vec<String>, ()> {
  if instance.dependencies.len() == 0 {
    return Ok(vec![
      "system: No dependencies found, skip installation process".to_string(),
    ]);
  }

  safe_send(
    tx.clone(),
    Event::Message,
    Some("system: Some dependencies found, start installation process".to_owned()),
  )
  .unwrap();

  Ok(Vec::new())
}

async fn execute_executor(
  uuid: Uuid,
  path: &PathBuf,
  tx: Sender<Bytes>,
  docker: Addr<DockerExecutor>,
  instance: &InstanceResponse,
  executor: &Executor,
) -> Result<Vec<String>, ()> {
  let r = docker
    .clone()
    .send(ExecuteContainer {
      name: uuid.to_string(),
      cmd: vec!["perl".to_owned(), "main.pl".to_owned()],
      env: Some(vec!["PERL5LIB=./local/lib/perl5:./lib".to_owned()]),
      image: format!("{}:{}", executor.image, executor.tag),
      bindings: Some(vec![format!(
        "{}:{}:ro",
        path.to_str().unwrap(),
        "/usr/local/PROJECT/workspace"
      )]),
      cpus: Some(250000000),                 // 0.25 cpus
      memory: Some(128000000),               // 128MB
      network_mode: Some("none".to_owned()), // Network is disabled
      ulimits: None,                         // I want to limit processes to: { soft: 16, hard: 32}
      working_dir: Some("/usr/local/PROJECT/workspace".to_owned()),
    })
    .await;

  match r {
    Ok(r) => match r {
      Ok(r) => Ok(r.iter().map(|w| w.to_owned()).collect()),
      Err(_) => Ok(Vec::new()),
    },
    Err(_) => Ok(Vec::new()),
  }
}

// if we fails to send message to server,
fn safe_send<T: serde::Serialize>(
  tx: Sender<Bytes>,
  event: Event,
  data: Option<T>,
) -> Result<(), ()> {
  match tx
    .clone()
    .try_send(to_bytes(ExecutorEvent::<T> { event, data }))
  {
    Ok(_) => Ok(()),
    Err(_) => Ok(()),
  }
}
