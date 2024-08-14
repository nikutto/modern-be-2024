use crate::service::hello_service;
use actix_web::{get, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HelloMsg {
    msg: String,
}

#[get("/api/hello")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().json(HelloMsg {
        msg: hello_service::get_hello().msg,
    })
}
