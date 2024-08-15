use crate::service::hello_service;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HelloMsg {
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateHelloMsg {
    id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ErrorMsg {
    message: String,
}

#[get("/api/hello/{id}")]
pub async fn get_hello(data: web::Data<mysql::Pool>, path: web::Path<u64>) -> impl Responder {
    let id = path.into_inner();
    let hello = hello_service::get_hello(&data, id);
    match hello {
        Ok(hello) => match hello {
            Some(hello) => HttpResponse::Ok().json(HelloMsg {
                message: hello.message,
            }),
            None => HttpResponse::NotFound().json(ErrorMsg {
                message: "Data not found".to_string(),
            }),
        },
        Err(_) => HttpResponse::InternalServerError().json(ErrorMsg {
            message: "Failed to get data".to_string(),
        }),
    }
}

#[derive(Deserialize)]
struct PostHelloRequest {
    message: String,
}

#[post("/api/hello")]
pub async fn post_hello(
    data: web::Data<mysql::Pool>,
    post_hello_request: web::Json<PostHelloRequest>,
) -> impl Responder {
    let result = hello_service::create_hello(&data, &(post_hello_request.message));
    match result {
        Ok(id) => HttpResponse::Created().json(CreateHelloMsg { id: id }),
        Err(_) => HttpResponse::InternalServerError().json(ErrorMsg {
            message: "Failed to insert data".to_string(),
        }),
    }
}
