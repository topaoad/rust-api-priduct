use crate::repositories::user_repository::UserRepository;
use crate::models::user::User;
use crate::db::DbPool;

pub struct UserService {}

impl UserService {
    pub async fn create_user(pool: &DbPool, user: &User) -> Result<User, diesel::result::Error> {
        UserRepository::create_user(pool, user).await
    }

    pub async fn get_user(pool: &DbPool, user_id: i32) -> Result<User, diesel::result::Error> {
        UserRepository::get_user(pool, user_id).await
    }

    pub async fn update_user(pool: &DbPool, user_id: i32, user: &User) -> Result<User, diesel::result::Error> {
        UserRepository::update_user(pool, user_id, user).await
    }

    pub async fn delete_user(pool: &DbPool, user_id: i32) -> Result<usize, diesel::result::Error> {
        UserRepository::delete_user(pool, user_id).await
    }

    pub async fn list_users(pool: &DbPool) -> Result<Vec<User>, diesel::result::Error> {
        UserRepository::list_users(pool).await
    }
}