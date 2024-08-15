use crate::{
    model::hello::Hello,
    repository::hello_repository::{insert_hello, select_hello},
};

pub fn get_hello(pool: &mysql::Pool, id: u64) -> Result<Option<Hello>, mysql::error::Error> {
    let hello = select_hello(pool, id)?;
    match hello {
        Some(hello) => Ok(Some(hello)),
        None => Ok(None),
    }
}

pub fn create_hello(pool: &mysql::Pool, msg: &String) -> Result<u64, mysql::error::Error> {
    let id = insert_hello(pool, msg)?;
    Ok(id)
}
