use my_app_domain::model::hello::Hello;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HelloMessage {
    pub payload: HelloMessagePayload,
}

#[derive(Debug, Deserialize)]
pub struct HelloMessagePayload {
    // pub before: Option<Hello>,
    pub after: Option<Hello>,
}
