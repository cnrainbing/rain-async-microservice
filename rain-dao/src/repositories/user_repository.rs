use rain_model::users::User;
use anyhow::Result;
use sqlx::MySqlPool;

pub struct UserRepository {}

/*impl UserRepository {
    pub async fn api_version() -> &'static str {
        "1.0"
    }
}*/

#[async_trait]
pub trait IUsersRepository {
    /// 注册用户
    async fn create(pool: &MySqlPool, new_user: &NewUser, password_hash: &str) -> Result<User>;

    /// 根据用户名查询用户
    async fn find_by_username(pool: &MySqlPool, username: &str) -> Result<Option<User>>;

    /// 根据邮箱查询查询用户
    async fn find_by_email(pool: &MySqlPool, email: &str) -> Result<Option<Users>>;

    /// 检查用户是否存在
    async fn exists_by_username(pool: &MySqlPool, username: &str) -> Result<bool>;

    /// 检查用户是否存在
    async fn exists_by_email(pool: &MySqlPool, email: &str) -> Result<bool>;
}