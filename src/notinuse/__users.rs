use diesel::prelude::*;
use actix_web::{web, HttpResponse, Responder};
use crate::db::DbPool;
use crate::models::{User, NewUser};
use crate::schema::users;

pub async fn create_user(pool: web::Data<DbPool>, new_user: NewUser) -> impl Responder {
    let conn = &mut pool.get().expect("couldn't get db connection from pool");

    match diesel::insert_into(users::table)
        .values(new_user)
        .get_result(conn) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
pub async fn get_user(pool: web::Data<DbPool>, user_id: web::Path<i32>) -> impl Responder {
    let conn = &mut pool.get().expect("couldn't get db connection from pool");
    match users::table.find(user_id.into_inner()).first(conn) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn update_user(pool: web::Data<DbPool>, user_id: i32, updated_user: NewUser) -> impl Responder {
    let conn = &mut pool.get().expect("couldn't get db connection from pool");

    match diesel::update(users::table.find(user_id))
        .set(updated_user)
        .get_result(conn) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn delete_user(pool: web::Data<DbPool>, user_id: i32) -> impl Responder {
    let conn = &mut pool.get().expect("couldn't get db connection from pool");

    match diesel::delete(users::table.find(user_id))
        .execute(conn) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}