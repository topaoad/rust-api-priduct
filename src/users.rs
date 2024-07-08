use diesel::prelude::*;
use crate::db::DbPool;
use crate::models::{User, NewUser};
use crate::schema::users;

pub fn create_user(pool: &DbPool, new_user: NewUser) -> QueryResult<User> {
    let conn = &mut pool.get().expect("couldn't get db connection from pool");

    diesel::insert_into(users::table)
        .values(new_user)
        .get_result(conn)
}


pub fn read_user(pool: &DbPool, user_id: i32) -> QueryResult<User> {
    let conn = &mut pool.get().expect("couldn't get db connection from pool");

    users::table.find(user_id).first(conn)
}

pub fn update_user(pool: &DbPool, user_id: i32, updated_user: NewUser) -> QueryResult<User> {
    let conn = &mut pool.get().expect("couldn't get db connection from pool");

    diesel::update(users::table.find(user_id))
        .set(updated_user)
        .get_result(conn)
}

pub fn delete_user(pool: &DbPool, user_id: i32) -> QueryResult<usize> {
    let conn = &mut pool.get().expect("couldn't get db connection from pool");

    diesel::delete(users::table.find(user_id))
        .execute(conn)
}

pub fn list_users(pool: &DbPool) -> QueryResult<Vec<User>> {
    let conn = &mut pool.get().expect("couldn't get db connection from pool");

    users::table.load::<User>(conn)
}