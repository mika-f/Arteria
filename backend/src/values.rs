use std::env;

use users;

pub fn bind_address() -> String {
  env::var("ARTERIA_BIND").unwrap_or("127.0.0.1".to_owned())
}

pub fn bind_port() -> String {
  env::var("ARTERIA_PORT").unwrap_or("3000".to_owned())
}

pub fn database_user() -> String {
  env::var("ARTERIA_DATABASE_USER").expect("ARTERIA_DATABASE_USER is not set")
}

pub fn database_pass() -> String {
  env::var("ARTERIA_DATABASE_PASS").expect("ARTERIA_DATABASE_PASS is not set")
}

pub fn database_host() -> String {
  env::var("ARTERIA_DATABASE_HOST").expect("ARTERIA_DATABASE_HOST is not set")
}

pub fn database_port() -> String {
  env::var("ARTERIA_DATABASE_PORT").expect("ARTERIA_DATABASE_PORT is not set")
}

pub fn hashed_id_salt() -> String {
  env::var("ARTERIA_HASH_SALT").expect("ARTERIA_HASH_SALT is not set")
}

pub fn cors_allowed_host() -> Option<String> {
  match env::var("ARTERIA_CORS_ALLOWED_HOST") {
    Ok(value) => Some(value),
    Err(_) => None,
  }
}

pub fn container_concurrency() -> usize {
  env::var("ARTERIA_CONTAINER_CONCURRENCY")
    .expect("ARTERIA_CONTAINER_CONCURRENCY is not set")
    .parse()
    .unwrap()
}

pub fn container_user() -> Option<String> {
  match env::var("ARTERIA_RUNNING_IN_CURRENT_USER") {
    Ok(_) => Some(format!(
      "{}:{}",
      users::get_current_uid(),
      users::get_current_gid()
    )),
    Err(_) => None,
  }
}

pub fn executor_cpu_limit() -> u64 {
  env::var("ARTERIA_CONTAINER_EXECUTOR_CPU_LIMIT")
    .unwrap_or("250000000".to_owned()) // 0.25 cpu
    .parse()
    .unwrap()
}

pub fn executor_memory_limit() -> u64 {
  env::var("ARTERIA_CONTAINER_EXECUTOR_MEMORY_LIMIT")
    .unwrap_or("128000000".to_owned()) // 128MB
    .parse()
    .unwrap()
}

pub fn executor_runtime() -> Option<String> {
  env::var("ARTERIA_CONTAINER_EXECUTOR_RUNTIME")
    .map(|w| Some(w))
    .unwrap_or(None)
}

pub fn executor_timeout() -> u64 {
  env::var("ARTERIA_CONTAINER_EXECUTOR_TIMEOUT")
    .unwrap_or("10".to_owned()) // 10 seconds
    .parse()
    .unwrap()
}

pub fn installer_cpu_limit() -> u64 {
  env::var("ARTERIA_CONTAINER_INSTALLER_CPU_LIMIT")
    .unwrap_or("500000000".to_owned()) // 0.5 cpu
    .parse()
    .unwrap()
}

pub fn installer_memory_limit() -> u64 {
  env::var("ARTERIA_CONTAINER_INSTALLER_MEMORY_LIMIT")
    .unwrap_or("256000000".to_owned()) // 256MB
    .parse()
    .unwrap()
}
