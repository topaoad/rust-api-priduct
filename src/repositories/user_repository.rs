use crate::db::DbPool;
use crate::models::user::{NewUser, User, UpdateUser};
use crate::schema::users;
use diesel::prelude::*;

pub struct UserRepository {}

impl UserRepository {
    pub async fn create_user(pool: &DbPool, user: &NewUser) -> QueryResult<User> {
        let conn = &mut pool.get().expect("couldn't get db connection from pool");
        diesel::insert_into(users::table)
            .values(user)
            .get_result(conn)
    }

    pub async fn get_user(pool: &DbPool, user_id: i32) -> QueryResult<User> {
        let conn = &mut pool.get().expect("couldn't get db connection from pool");
        users::table.find(user_id).first(conn)
    }

    pub async fn update_user(pool: &DbPool, user_id: i32, user: &UpdateUser) -> QueryResult<User> {
        let conn = &mut pool.get().expect("couldn't get db connection from pool");
        diesel::update(users::table.find(user_id))
            .set(user)
            .get_result(conn)
    }

    pub async fn delete_user(pool: &DbPool, user_id: i32) -> QueryResult<usize> {
        let conn = &mut pool.get().expect("couldn't get db connection from pool");
        diesel::delete(users::table.find(user_id))
            .execute(conn)
    }

    pub async fn list_users(pool: &DbPool) -> QueryResult<Vec<User>> {
        let conn = &mut pool.get().expect("couldn't get db connection from pool");
        users::table.load(conn)
    }
}