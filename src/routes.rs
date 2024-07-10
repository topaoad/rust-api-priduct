use actix_web::web;
use crate::controllers::hello_controller;
use crate::controllers::user_controller::{
    create_user, delete_user, get_user, list_users, update_user,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/hello", web::get().to(hello_controller::hello))
            .route("/greet/{name}", web::get().to(hello_controller::greet))
        );
    cfg.service(
        web::scope("/users")
            .route("", web::post().to(create_user))
            .route("", web::get().to(list_users))
            .route("/{user_id}", web::get().to(get_user))
            .route("/{user_id}", web::put().to(update_user))
            .route("/{user_id}", web::delete().to(delete_user))
    );
}