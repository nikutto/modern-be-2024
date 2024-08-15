mod configuration;
mod handler;
mod model;
mod repository;
mod service;
use actix_web::{web, App, HttpServer};
use configuration::mysql_configuration::get_mysql_pool;
use handler::hello_handler::{get_hello, post_hello};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = get_mysql_pool();
    let shared_data = web::Data::new(pool);
    HttpServer::new(move || {
        App::new()
            .app_data(shared_data.clone())
            .service(get_hello)
            .service(post_hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
