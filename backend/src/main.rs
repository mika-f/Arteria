#[macro_use]
extern crate diesel;

use std::env;
use std::fs;
use actix_web::{middleware, App, HttpServer};
use bollard::Docker;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv;
use harsh;

mod database;
mod dirs;
mod errors;
mod models;
mod schema;
mod services;
mod web;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    create_reserved_directories();

    let server_bind = env::var("ARTERIA_BIND").expect("ARTERIA_BIND not set");
    let server_port = env::var("ARTERIA_PORT").expect("ARTERIA_PORT not set");

    let database_host = format!(
        "mysql://{}:{}@{}:{}/arteria",
        env::var("ARTERIA_DATABASE_USER").expect("ARTERIA_DATABASE_USER not set"),
        env::var("ARTERIA_DATABASE_PASS").expect("ARTERIA_DATABASE_PASS not set"),
        env::var("ARTERIA_DATABASE_HOST").expect("ARTERIA_DATABASE_HOST not set"),
        env::var("ARTERIA_DATABASE_PORT").expect("ARTERIA_DATABASE_PORT not set"),
    );

    // database connection
    let manager = ConnectionManager::<MysqlConnection>::new(database_host);
    let connection_pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create a pool");

    // docker connection
    let docker =
        Docker::connect_with_local_defaults().expect("Failed to create a connection with Docker");

    // id generator
    let harsh = harsh::HarshBuilder::new()
        .salt(env::var("ARTERIA_HASH_SALT").expect("ARTERIA_HASH_SALT not set"))
        .length(10)
        .init()
        .expect("Failed to create hash id builder");

    HttpServer::new(move || {
        App::new()
            .data(connection_pool.clone())
            .data(docker.clone())
            .data(harsh.clone())
            .wrap(middleware::Logger::default())
            // routings
            .configure(web::executors::routings)
            .configure(web::instances::routings)
            .configure(web::meta::routings)
            .configure(web::root::routings)
    })
    .bind(format!("{}:{}", server_bind, server_port))?
    .run()
    .await
}

fn create_reserved_directories() {
    // create a cache directory for installed modules
    fs::create_dir(dirs::cache_dir().to_str().unwrap()).unwrap();

    // create a project directory for executing source codes
    fs::create_dir(dirs::temp_dir().to_str().unwrap()).unwrap();
}
