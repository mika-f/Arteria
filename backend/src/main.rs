#[macro_use]
extern crate diesel;

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
mod executors;
mod models;
mod schema;
mod services;
mod values;
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

    let server_bind = values::bind_address();
    let server_port = values::bind_port();

    let database_host = format!(
        "mysql://{}:{}@{}:{}/arteria",
        values::database_user(),
        values::database_pass(),
        values::database_host(),
        values::database_port()
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
    let docker_addr = SyncArbiter::start((num_cpus::get() * 2) as usize, move || {
        docker::DockerExecutor(docker.clone())
    });

    // id generator
    let harsh = harsh::HarshBuilder::new()
        .salt(values::hashed_id_salt())
        .length(10)
        .init()
        .expect("Failed to create hash id builder");

    // instance runner
    let executor = executors::PerlExecutor::create(db_addr.clone(), docker_addr.clone());

    HttpServer::new(move || {
        App::new()
            .app_data(executor.clone())
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
    let cache_dir = dirs::cache_dir();
    if !cache_dir.exists() {
        fs::create_dir(cache_dir.to_str().unwrap()).unwrap();
    }

    // create a project directory for executing source codes
    let temp_dir = dirs::temp_dir();
    if !temp_dir.exists() {
        fs::create_dir(temp_dir.to_str().unwrap()).unwrap();
    }
}
