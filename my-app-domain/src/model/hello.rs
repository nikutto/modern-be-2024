use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Hello {
    pub id: u64,
    pub message: String,
}
