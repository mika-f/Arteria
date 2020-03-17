#[macro_use]
extern crate diesel;

use actix_web::{middleware, App, HttpServer};
use bollard::Docker;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv;
use harsh;

mod routings;

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let server_bind = std::env::var("ARTERIA_BIND").expect("ARTERIA_BIND not set");
    let server_port = std::env::var("ARTERIA_PORT").expect("ARTERIA_PORT not set");

    let database_host =
        std::env::var("ARTERIA_DATABASE_HOST").expect("ARTERIA_DATABASE_HOST not set");

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
        .salt(std::env::var("ARTERIA_HASH_SALT").expect("ARTERIA_HASH_SALT not set"))
        .length(10)
        .init()
        .expect("Failed to create hash id builder");

    HttpServer::new(move || {
        App::new()
            .data(connection_pool.clone())
            .data(docker.clone())
            .data(harsh.clone())
            .wrap(middleware::Logger::default())
            .service(routings::index)
            .service(routings::version)
    })
    .bind(format!("{}:{}", server_bind, server_port))?
    .run()
    .await
}
