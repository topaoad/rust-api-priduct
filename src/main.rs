mod routes;
mod controllers;
mod services;
mod db;
mod models;
mod schema;
mod users;

use crate::models::{NewUser};
use crate::db::DbPool;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
// use diesel::r2d2::{self, ConnectionManager};
// use diesel::PgConnection;

async fn create_user(pool: web::Data<DbPool>, new_user: web::Json<NewUser>) -> impl Responder {
    match users::create_user(&pool, new_user.into_inner()) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn get_user(pool: web::Data<DbPool>, user_id: web::Path<i32>) -> impl Responder {
    match users::read_user(&pool, user_id.into_inner()) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

async fn update_user(pool: web::Data<DbPool>, user_id: web::Path<i32>, updated_user: web::Json<NewUser>) -> impl Responder {
    match users::update_user(&pool, user_id.into_inner(), updated_user.into_inner()) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

async fn delete_user(pool: web::Data<DbPool>, user_id: web::Path<i32>) -> impl Responder {
    match users::delete_user(&pool, user_id.into_inner()) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

async fn list_users(pool: web::Data<DbPool>) -> impl Responder {
    match users::list_users(&pool) {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}


#[derive(Serialize, Deserialize)]
struct Message {
    content: String,
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix!")
}

async fn echo(msg: web::Json<Message>) -> impl Responder {
    HttpResponse::Ok().json(msg.0)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://0.0.0.0:8088");
    HttpServer::new(|| {
        App::new()
            .configure(routes::config)
            // .route("/", web::get().to(hello))
            // .route("/echo", web::post().to(echo))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

// 公式チュートリアル
// use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(hello)
//             .service(echo)
//             .route("/hey", web::get().to(manual_hello))
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }