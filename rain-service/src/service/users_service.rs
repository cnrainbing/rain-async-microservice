use anyhow::Result;
use async_trait::async_trait;
use rain_model::users::{User};
use rain_dao::repositories::{IUsersRepository,UserRepository};
use sqlx::MySqlPool;


pub struct UsersService;

#[async_trait]
pub trait IUsersService {
    /// 注册用户
    //async fn user_register(pool: &MySqlPool, new_user: &CreateUser, password_hash: &str) -> Result<User>;

    /// 根据用户名查询用户
    async fn find_by_username(pool: &MySqlPool, username: &str) -> Result<Option<User>>;

}

#[async_trait]
impl IUsersService for UsersService {
    /*async fn user_register(pool: &MySqlPool, new_user: &CreateUser, password_hash: &str) -> Result<User> {
        UserRepository::create(pool, new_user, password_hash).await?
    }*/

    async fn find_by_username(pool: &MySqlPool, username: &str) -> Result<Option<User>> {
        UserRepository::find_by_username(pool,username).await
    }
}