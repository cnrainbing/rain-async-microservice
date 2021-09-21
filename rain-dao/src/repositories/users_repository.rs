use rain_model::users::{User, CreateUser};
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
    async fn create(pool: &MySqlPool, new_user: &CreateUser, password_hash: &str) -> Result<u64>;

    /// 根据用户名查询用户
    async fn find_by_username(pool: &MySqlPool, username: &str) -> Result<Option<User>>;

    /// 根据用户名查询用户2
    async fn find_by_username2(pool: &MySqlPool, username: &str) -> Result<User>;

    /// 根据邮箱查询查询用户
    async fn find_by_email(pool: &MySqlPool, email: &str) -> Result<Option<User>>;
}

#[async_trait]
impl IUsersRepository for UserRepository {
    async fn create(pool: &MySqlPool, new_user: &CreateUser, password_hash: &str) -> Result<u64> {
        let sql: &str = r#"INSERT INTO t_users(username,nickname) VALUES (?,?,?,?)"#;
        let id = sqlx::query(sql)
            .bind(&new_user.username)
            .bind(&new_user.nickname)
            .bind(&new_user.email)
            .bind(password_hash)
            .execute(pool)
            .await
            .context("创建用户")?
            .last_insert_id();
        Ok(id)
    }

    /// 根据用户名查询用户
    async fn find_by_username(pool: &MySqlPool, username: &str) -> Result<Option<User>> {
        let row = sqlx::query_as!(
            User,
            "SELECT * FROM t_users WHERE username = ?",
            username
        ).fetch_optional(pool).await.context("查询用户")?;
        Ok(row)
    }

    /// 根据用户名查询用户2
    async fn find_by_username2(pool: &MySqlPool, username: &str) -> Result<User> {
        let row = sqlx::query_as!(
            User,
            "SELECT * FROM t_users WHERE username = ?",
            username
        )
            .fetch_one(pool)
            .await
            .context("根据用户名查询用户2")?;

        Ok(row)
    }

    /// 根据邮箱查询用户
    async fn find_by_email(pool: &MySqlPool, email: &str) -> Result<Option<User>> {
        let row = sqlx::query_as!(
            User,
            "SELECT * FROM t_users WHERE email = ?",
            email
        )
            .fetch_optional(pool)
            .await
            .context("查询用户")?;

        Ok(row)
    }
}