use std::path::PathBuf;

use dirs as d;

pub fn temp_dir() -> PathBuf {
  let mut temp_dir = std::env::temp_dir();
  temp_dir.push("arteria");

  temp_dir
}

pub fn cache_dir() -> PathBuf {
  let mut cache_dir = d::cache_dir().unwrap();
  cache_dir.push("arteria");

  cache_dir
}
