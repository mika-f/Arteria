use actix_web::{middleware, web, App, HttpServer};
use actix_web::{middleware, App, HttpServer};
use dotenv;

mod routings;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("ARTERIA_PORT").unwrap();
    dotenv::dotenv().ok();
    let server_bind = std::env::var("ARTERIA_BIND").unwrap_or("127.0.0.1".to_string());
    let server_port = std::env::var("ARTERIA_PORT").unwrap_or("3000".to_string());

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/version", web::get().to(routings::version))
            .service(routings::index)
            .service(routings::version)
    })
    .bind(format!("{}:{}", server_bind, server_port))?
    .run()
    .await
}
