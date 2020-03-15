use actix_web::{middleware, web, App, HttpServer};

mod routings;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("ARTERIA_PORT").unwrap();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/version", web::get().to(routings::version))
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}
