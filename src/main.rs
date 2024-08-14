mod handler;
mod model;
mod service;
mod repository;
use actix_web::{web, App, HttpServer};
use handler::hello_handler::{get_hello, post_hello};

fn get_conn_builder(
    db_user: String,
    db_password: String,
    db_host: String,
    db_port: u16,
    db_name: String,
) -> mysql::OptsBuilder {
    mysql::OptsBuilder::new()
        .ip_or_hostname(Some(db_host))
        .tcp_port(db_port)
        .db_name(Some(db_name))
        .user(Some(db_user))
        .pass(Some(db_password))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let builder = get_conn_builder(
        str::to_string("user"), 
        str::to_string("password"),
        str::to_string("127.0.0.1"), 
        3306,
        str::to_string("mydb"),
    );
    let pool = mysql::Pool::new(builder).unwrap();
    let shared_data = web::Data::new(pool);
    HttpServer::new(move || 
        App::new()
        .app_data(shared_data.clone())
        .service(get_hello)
        .service(post_hello)
    ).bind(("127.0.0.1", 8080))?
    .run()
    .await
}
