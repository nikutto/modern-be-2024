use std::collections::HashMap;

use crate::service::hello_service;
use actix_web::{get, post, web, HttpResponse, Responder};
use my_app_domain::model::hello::Hello;
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

#[derive(Debug, Serialize, Deserialize)]
struct SearchHelloResponse {
    hits: Vec<Hello>,
}

#[get("/api/search/hello")]
pub async fn search_hello(
    pool: web::Data<mysql::Pool>, 
    client: web::Data<opensearch::OpenSearch>,
    query_params: web::Query<HashMap<String, String>>,
) -> impl Responder {
    let query = query_params.get("query").unwrap();
    let result = hello_service::search_hello(&client, &pool, query).await;
    match result {
        Ok(hellos) => HttpResponse::Ok().json(
            SearchHelloResponse {
                hits: hellos
            }
        ),
        Err(e) => HttpResponse::InternalServerError().json(ErrorMsg {
            message: format!("Failed to search data {:?}", e).to_string(),
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
