#[macro_use]
extern crate diesel;

use std::env;
use std::fs;

use actix::prelude::*;
use actix_web::{middleware, App, HttpServer};
use bollard::Docker;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv;
use harsh;
use num_cpus;

mod database;
mod dirs;
mod docker;
mod errors;
mod models;
mod schema;
mod services;
mod web;

pub struct AppState {
    pub db: Addr<database::DbExecutor>,
    pub docker: Addr<docker::DockerExecutor>,
}

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
    let db_addr = SyncArbiter::start(num_cpus::get(), move || {
        database::DbExecutor(connection_pool.clone())
    });

    // docker connection
    let docker =
        Docker::connect_with_local_defaults().expect("Failed to create a connection with Docker");
    let docker_addr = SyncArbiter::start(num_cpus::get(), move || {
        docker::DockerExecutor(docker.clone())
    });

    // id generator
    let harsh = harsh::HarshBuilder::new()
        .salt(env::var("ARTERIA_HASH_SALT").expect("ARTERIA_HASH_SALT not set"))
        .length(10)
        .init()
        .expect("Failed to create hash id builder");

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                db: db_addr.clone(),
                docker: docker_addr.clone(),
            })
            .data(harsh.clone())
            // middleware
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
