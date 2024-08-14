use crate::service::hello_service;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HelloMsg {
    msg: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateHelloMsg {
    id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ErrorMsg {
    msg: String,
}

#[get("/api/hello/{id}")]
pub async fn get_hello(
    data: web::Data<mysql::Pool>,
    path: web::Path<(u64)>,
) -> impl Responder {
    let (id) = path.into_inner();
    let hello = hello_service::get_hello(&data, id);
    match hello {
        Ok(hello) => match hello {
            Some(hello) => HttpResponse::Ok().json(
                HelloMsg {
                    msg: hello.message,
                },
            ),
            None => HttpResponse::NotFound().json(
                ErrorMsg {
                    msg: "Data not found".to_string(),
                },
            ),
        },
        Err(_) => HttpResponse::InternalServerError().json(
            ErrorMsg {
                msg: "Failed to get data".to_string(),
            },
        ),
    }
}


#[post("/api/hello")]
pub async fn post_hello(
    data: web::Data<mysql::Pool>,
) -> impl Responder {
    let result = hello_service::create_hello(&data, "Hello, World!".to_string());
    match result {
        Ok(id) => HttpResponse::Created().json(
            CreateHelloMsg {
                id: id,
            },
        ),
        Err(_) => HttpResponse::InternalServerError().json(
            ErrorMsg {
                msg: "Failed to insert data".to_string(),
            },
        ),
    }
}
