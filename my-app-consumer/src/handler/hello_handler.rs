use crate::model::hello_message::HelloMessage;

pub fn handle(event: HelloMessage) {
    match event.payload.after {
        Some(after) => {
            println!("Hello object: id={:?} msg={:?}", after.id, after.message);
        }
        None => {
            println!("Some record has been deleted.");
        }
    }
}
