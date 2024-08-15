use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Hello {
    pub id: u64,
    pub message: String,
}
