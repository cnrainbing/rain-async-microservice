use anyhow::Result;
use async_trait::async_trait;
use rain_model::users::{User, CreateUser};
use rain_dao::repositories::{IUsersRepository,UserRepository};
use sqlx::MySqlPool;


pub struct UsersService;

#[async_trait]
pub trait IUsersService {
    /// 注册用户
    async fn user_register(pool: &MySqlPool, new_user: &CreateUser, password_hash: &str) -> Result<u64>;

    /// 根据用户名查询用户
    async fn find_by_username(pool: &MySqlPool, username: &str) -> Result<Option<User>>;

    /// 根据用户名查询用户2
    async fn find_by_username2(pool: &MySqlPool, username: &str) -> Result<User>;

    /// 根据邮箱查询查询用户
    async fn find_by_email(pool: &MySqlPool, email: &str) -> Result<Option<User>>;

    /// 检查用户是否存在
    async fn exists_by_username(pool: &MySqlPool, username: &str) -> Result<bool>;

    /// 检查邮箱是否存在
    async fn exists_by_email(pool: &MySqlPool, email: &str) -> Result<bool>;
}

#[async_trait]
impl IUsersService for UsersService {
    async fn user_register(pool: &MySqlPool, new_user: &CreateUser, password_hash: &str) -> Result<u64> {
        UserRepository::create(pool, new_user, password_hash).await
    }

    async fn find_by_username(pool: &MySqlPool, username: &str) -> Result<Option<User>> {
        UserRepository::find_by_username(pool,username).await
    }

    async fn find_by_username2(pool: &MySqlPool, username: &str) -> Result<User> {
        UserRepository::find_by_username2(pool, username).await
    }

    async fn find_by_email(pool: &MySqlPool, email: &str) -> Result<Option<User>> {
        UserRepository::find_by_email(pool, email).await
    }

    async fn exists_by_username(pool: &MySqlPool, username: &str) -> Result<bool> {
        UserRepository::exists_by_username(pool, username).await
    }

    async fn exists_by_email(pool: &MySqlPool, email: &str) -> Result<bool> {
        UserRepository::exists_by_email(pool, email).await
    }
}