use crate::services::user_service::UserService;
use crate::models::user::User;
use crate::db::DbPool;
use actix_web::{web, HttpResponse, Responder};

pub async fn create_user(pool: web::Data<DbPool>, user: web::Json<User>) -> impl Responder {
    match UserService::create_user(&pool, &user.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_user(pool: web::Data<DbPool>, user_id: web::Path<i32>) -> impl Responder {
    match UserService::get_user(&pool, user_id.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn update_user(pool: web::Data<DbPool>, user_id: web::Path<i32>, user: web::Json<User>) -> impl Responder {
    match UserService::update_user(&pool, user_id.into_inner(), &user.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn delete_user(pool: web::Data<DbPool>, user_id: web::Path<i32>) -> impl Responder {
    match UserService::delete_user(&pool, user_id.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn list_users(pool: web::Data<DbPool>) -> impl Responder {
    println!("list_users function called");
    match UserService::list_users(&pool).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}