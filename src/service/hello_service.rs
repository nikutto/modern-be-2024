use crate::model::hello::Hello;

pub fn get_hello() -> Hello {
    Hello {
        id: 0,
        msg: "Hello, World!".to_string(),
    }
}
