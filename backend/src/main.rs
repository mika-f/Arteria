use actix_web::{middleware, App, HttpServer};
use bollard::Docker;
use dotenv;

mod routings;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let server_bind = std::env::var("ARTERIA_BIND").unwrap_or("127.0.0.1".to_string());
    let server_port = std::env::var("ARTERIA_PORT").unwrap_or("3000".to_string());
    // docker connection
    let docker =
        Docker::connect_with_local_defaults().expect("Failed to create a connection with Docker");


    HttpServer::new(move || {
        App::new()
            .data(docker.clone())
            .wrap(middleware::Logger::default())
            .service(routings::index)
            .service(routings::version)
    })
    .bind(format!("{}:{}", server_bind, server_port))?
    .run()
    .await
}
