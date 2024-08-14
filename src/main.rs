mod handler;
mod model;
mod service;
use actix_web::{App, HttpServer};
use handler::hello_handler::hello;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
