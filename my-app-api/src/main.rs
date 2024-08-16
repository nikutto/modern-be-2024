mod configuration;
mod handler;
mod repository;
mod service;
mod model;
use actix_web::{web, App, HttpServer};
use configuration::{mysql_configuration::get_mysql_pool, opensearch_configuration::get_opensearch_client};
use handler::hello_handler::{get_hello, post_hello, search_hello};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = get_mysql_pool();
    let shared_data1 = web::Data::new(pool);
    let client = get_opensearch_client();
    let shared_data2 = web::Data::new(client);
    HttpServer::new(move || {
        App::new()
            .app_data(shared_data1.clone())
            .app_data(shared_data2.clone())
            .service(get_hello)
            .service(post_hello)
            .service(search_hello)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
