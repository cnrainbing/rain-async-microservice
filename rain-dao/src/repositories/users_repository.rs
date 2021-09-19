use rain_model::users::{User};
use anyhow::*;
use sqlx::{MySqlPool};
use async_trait::async_trait;

pub struct UserRepository {}

impl UserRepository {
    pub async fn api_version() -> &'static str {
        "1.0"
    }
}

#[async_trait]
pub trait IUsersRepository {
    /// 注册用户
    ///async fn create(pool: &MySqlPool, new_user: &CreateUser, password_hash: &str) -> Result<User>;

    /// 根据用户名查询用户
    async fn find_by_username(pool: &MySqlPool, username: &str) -> Result<Option<User>>;
}

#[async_trait]
impl IUsersRepository for UserRepository {
    /*    async fn create(pool: &MySqlPool, new_user: &CreateUser, password_hash: &str) -> Result<User> {
            let row  = sqlx::query_as!(
                User,
                "INSERT INTO t_users(username, nickname, email, password_hash) VALUES (?, ?, ?, ?)",
                &new_user.username,
                &new_user.nickname,
                &new_user.email,
                password_hash
            )
                .fetch_one(pool)
                .await
                .context("创建用户")?;
                Ok(row)
        }*/

    /// 根据用户名查询用户
    async fn find_by_username(pool: &MySqlPool, username: &str) -> Result<Option<User>> {
        let row = sqlx::query_as!(
            User,
            "SELECT * FROM t_users WHERE username = ?",
            username
        ).fetch_optional(pool).await.context("查询用户")?;
        Ok(row)
    }
}