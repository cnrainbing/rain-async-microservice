use async_graphql::{MergedObject, Object, Context, ErrorExtensions};
use crate::web::gql::GraphqlResult;
use rain_model::users::{CreateUser};
use crate::error::errors::AppError;
use crate::Context as rainContext;
use sqlx::{Pool, MySql};
use std::sync::Arc;
use rain_service::service::{UsersService, IUsersService};
use validator::Validate;

/// 变更根节点
#[derive(MergedObject, Default)]
pub struct MutationRoot(UsersMutation);

/// 用户变更 Mutation
#[derive(Default)]
pub struct UsersMutation;

#[Object]
impl UsersMutation {
    async fn pong(&self) -> GraphqlResult<String> {
        Ok("pong".to_string())
    }

    /// 注册用户
    async fn user_register(&self, ctx: &Context<'_>, mut new_user: CreateUser) -> GraphqlResult<u64> {
        // 参数校验
        new_user.validate().map_err(AppError::RequestParameterError.validation_extend())?;

        let pool: Arc<Pool<MySql>> = rainContext::get_pool(ctx)?;

        // 处理为 小写
        new_user.username.make_ascii_lowercase();
        new_user.email.make_ascii_lowercase();

        // 检查用户名重复
        let exists = UsersService::exists_by_username(&pool, &new_user.username).await?;
        if exists {
            return Err(AppError::UsernameAlreadyExists.extend());
        }

        // 检查邮箱重复
        let exists = UsersService::exists_by_email(&pool, &new_user.email).await?;
        if exists {
            return Err(AppError::EmailAlreadyExists.extend());
        }

        // 密码哈希
        let password_hash = "123456";

        let id: u64 = UsersService::user_register(&pool, &new_user, &password_hash).await?;
        Ok(id)
    }
}