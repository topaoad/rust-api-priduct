mod routes;
mod controllers;
mod services;
mod db;
mod models;
mod schema;
mod repositories;
// mod users;

use crate::controllers::user_controller::{
    create_user, get_user, update_user, delete_user, list_users,
};
use crate::db::DbPool;
use actix_web::{web, App, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://0.0.0.0:8088");
    HttpServer::new(|| {
        App::new()
            .configure(routes::config)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}