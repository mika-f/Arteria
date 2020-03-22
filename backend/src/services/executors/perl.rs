use std::error;
use std::fs;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use actix::prelude::*;
use actix_web::web::Bytes;
use tokio::sync::mpsc::Sender;
use uuid::Uuid;

use crate::dirs;
use crate::docker::DockerExecutor;
use crate::executors::{to_bytes, Event, ExecutorEvent};
use crate::models::InstanceResponse;

pub async fn execute(
  tx: Sender<Bytes>,
  docker: Addr<DockerExecutor>,
  instance: InstanceResponse,
) -> Result<Vec<String>, ()> {
  let uuid = Uuid::new_v4();
  let path = create_project_directory(uuid).unwrap();
  let mut logs = Vec::new();

  create_deps_file(&path, &instance).unwrap();

  for log in execute_installer(tx.clone(), docker.clone(), &path, &instance)
    .unwrap()
    .iter()
  {
    logs.push(log.to_owned());
  }

  for log in execute_executor(tx.clone(), docker.clone(), &path, &instance)
    .unwrap()
    .iter()
  {
    logs.push(log.to_owned());
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

fn cleanup_project_directory(path: &PathBuf) -> Result<(), Box<dyn error::Error>> {
  fs::remove_dir_all(path)?;

  Ok(())
}

fn execute_installer(
  tx: Sender<Bytes>,
  docker: Addr<DockerExecutor>,
  path: &PathBuf,
  instance: &InstanceResponse,
) -> Result<Vec<String>, ()> {
  if instance.dependencies.len() == 0 {
    safe_send(
      tx.clone(),
      Event::Message,
      Some("No dependencies found, skip installation process".to_owned()),
    )
    .unwrap();

    return Ok(vec![
      "No dependencies found, skip installation process".to_string()
    ]);
  }

  safe_send(
    tx.clone(),
    Event::Message,
    Some("Some dependencies found, start installation process".to_owned()),
  )
  .unwrap();

  Ok(Vec::new())
}

fn execute_executor(
  tx: Sender<Bytes>,
  docker: Addr<DockerExecutor>,
  path: &PathBuf,
  instance: &InstanceResponse,
) -> Result<Vec<String>, ()> {
  Ok(Vec::new())
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
