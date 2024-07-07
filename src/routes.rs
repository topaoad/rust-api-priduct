use actix_web::web;
use crate::controllers::hello_controller;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/hello", web::get().to(hello_controller::hello))
            .route("/greet/{name}", web::get().to(hello_controller::greet))
    );
}