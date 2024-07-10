mod routes;
mod controllers;
mod services;
mod db;
mod models;
mod schema;
mod repositories;

use crate::db::establish_connection;
use env_logger;
use log::info;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    println!("Starting server at http://0.0.0.0:8088");
    let pool = establish_connection();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(crate::routes::config)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}