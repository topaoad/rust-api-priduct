use actix_web::{web, HttpResponse, Responder};
use crate::services::hello_service;

pub async fn hello() -> impl Responder {
    let message = hello_service::get_hello_message();
    HttpResponse::Ok().body(message)
}

pub async fn greet(path: web::Path<String>) -> impl Responder {
    let name = path.into_inner();
    let message = hello_service::get_greeting(&name);
    HttpResponse::Ok().body(message)
}