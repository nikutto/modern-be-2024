use my_app_domain::model::hello::Hello;
use mysql::{params, prelude::*};

pub fn insert_hello(pool: &mysql::Pool, message: &String) -> mysql::error::Result<u64> {
    let mut conn = pool.get_conn()?;
    conn.exec_drop(
        r"INSERT INTO hello (message) VALUES (:message)",
        params! {
          "message" => message,
        },
    )
    .map(|_| conn.last_insert_id())
}

pub fn select_hello(pool: &mysql::Pool, id: u64) -> mysql::error::Result<Option<Hello>> {
    let mut conn = pool.get_conn()?;
    let result = conn.exec_first(
        r"SELECT id, message FROM hello WHERE id = :id",
        params! {
          "id" => id,
        },
    )?;
    match result {
        Some(row) => {
            let (id, message) = mysql::from_row(row);
            Ok(Some(Hello { id, message }))
        }
        None => Ok(None),
    }
}
