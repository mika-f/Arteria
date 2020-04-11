use std::error;
use std::fs;
use std::io::{BufWriter, Write};
#[cfg(any(target_os = "macos", target_os = "linux"))]
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;

use actix::prelude::*;
use actix_web::web::Bytes;
use tokio::sync::mpsc::Sender;
use uuid::Uuid;

use crate::dirs;
use crate::docker::container::ExecuteContainer;
use crate::docker::DockerExecutor;
use crate::models::{Executor, InstanceResponse};
use crate::values;

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

  for log in execute_installer(
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
  }

  create_project_file(&path, &instance).unwrap();

  for log in execute_executor(uuid, &path, tx.clone(), docker.clone(), &executor)
    .await
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

  let mut runner = PathBuf::from(path);
  runner.push("run.sh");

  #[cfg(any(target_os = "macos", target_os = "linux"))]
  let mut run_sh = BufWriter::new(
    fs::OpenOptions::new()
      .create(true)
      .write(true)
      .mode(0o777)
      .open(&runner)?,
  );
  #[cfg(target_os = "windows")]
  let mut run_sh = BufWriter::new(fs::File::create(&runner)?);

  run_sh.write(
    &format!(
      "{}\n",
      "\
#!/bin/sh

cpanm --notest Carmel
carmel install && carmel rollout\
  "
    )
    .as_bytes(),
  )?;

  run_sh.flush()?;

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

    let dir = to.parent();
    match dir {
      Some(dir) => {
        if !dir.exists() {
          fs::create_dir_all(dir)?;
        }
      }
      None => {}
    };

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
  uuid: Uuid,
  path: &PathBuf,
  tx: Sender<Bytes>,
  docker: Addr<DockerExecutor>,
  instance: &InstanceResponse,
  executor: &Executor,
) -> Result<Vec<String>, ()> {
  if instance.dependencies.len() == 0 {
    return Ok(vec![
      "system: No dependencies found, skip installation process".to_string(),
    ]);
  }

  let cache_dir = dirs::cache_dir();

  let r = docker
    .clone()
    .send(ExecuteContainer {
      name: format!("arteria-{}-installer", uuid.to_string()),
      cmd: vec!["./run.sh".to_owned()],
      image: format!("{}:{}", executor.image, executor.tag),

      // optionals
      bindings: Some(vec![
        // project directory
        format!(
          "{}:{}:rw",
          path.to_str().unwrap(),
          "/usr/local/PROJECT/workspace"
        ),
        // module cache directory
        format!(
          "{}:{}:rw",
          cache_dir.to_str().unwrap().to_owned(),
          "/usr/local/PROJECT/caches".to_owned()
        ),
      ]),
      cpus: Some(values::installer_cpu_limit()),
      env: Some(vec![
        "PERL5LIB=./local/lib/perl5".to_owned(),
        "PERL_CARMEL_REPO=/usr/local/PROJECT/caches".to_owned(),
      ]),
      logger: Some(tx.clone()),
      memory: Some(values::installer_memory_limit()),
      network_mode: Some("bridge".to_owned()), // accept network connection
      runtime: None,
      timeout: None,
      ulimits: None,
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

async fn execute_executor(
  uuid: Uuid,
  path: &PathBuf,
  tx: Sender<Bytes>,
  docker: Addr<DockerExecutor>,
  executor: &Executor,
) -> Result<Vec<String>, ()> {
  let r = docker
    .clone()
    .send(ExecuteContainer {
      name: format!("arteria-{}-executor", uuid.to_string()),
      cmd: vec!["perl".to_owned(), "main.pl".to_owned()],
      env: Some(vec!["PERL5LIB=./local/lib/perl5:./lib".to_owned()]),
      image: format!("{}:{}", executor.image, executor.tag),
      bindings: Some(vec![format!(
        "{}:{}:ro",
        path.to_str().unwrap(),
        "/usr/local/PROJECT/workspace"
      )]),
      cpus: Some(values::executor_cpu_limit()),
      logger: Some(tx.clone()),
      memory: Some(values::executor_memory_limit()),
      network_mode: Some("none".to_owned()), // Network is disabled
      runtime: values::executor_runtime(),   // I want to use gVisor for container runtime
      timeout: Some(values::executor_timeout()), // timeout 10 seconds
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
