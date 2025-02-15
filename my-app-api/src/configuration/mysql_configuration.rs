use std::env;

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

pub fn get_mysql_pool() -> mysql::Pool {
    let db_user = env::var("MYSQL_USER").expect("MYSQL_USER is not set");
    let db_password = env::var("MYSQL_PASSWORD").expect("MYSQL_PASSWORD is not set");
    let db_host = env::var("MYSQL_HOST").expect("MYSQL_HOST is not set");
    let db_port = env::var("MYSQL_PORT").expect("MYSQL_PORT is not set");
    let db_name = env::var("MYSQL_DBNAME").expect("MYSQL_DBNAME is not set");
    let db_port = db_port.parse().unwrap();

    let builder = get_conn_builder(db_user, db_password, db_host, db_port, db_name);
    let pool = mysql::Pool::new(builder).unwrap();
    pool
}
