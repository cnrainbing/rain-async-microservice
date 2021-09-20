use async_graphql::{MergedObject, Object,Context};
use crate::web::gql::GraphqlResult;
use crate::Context as rainContext;
use rain_service::service::{IUsersService,UsersService};
use rain_model::users::User;
use crate::error::errors::AppError;


/// 定义查询根节点
#[derive(MergedObject, Default)]
pub struct QueryRoot(PingQuery, UsersQuery);

/// ping Query
#[derive(Default)]
pub struct PingQuery;

/// 用户查询 queries
#[derive(Default)]
pub struct UsersQuery;

#[Object]
impl PingQuery {
    async fn ping(&self) -> GraphqlResult<String> {
        Ok("pong".to_string())
    }
}

#[Object]
impl UsersQuery {
    async fn user_ping(&self) -> GraphqlResult<String> {
        Ok("user_ping".to_string())
    }

    /// 根据用户名查询用户
    async fn find_by_username(
        &self,
        ctx: &Context<'_>,
        username: String,
    ) -> GraphqlResult<Option<User>> {
        let pool = rainContext::get_pool(ctx)?;
        Ok(UsersService::find_by_username(&pool, &username)
            .await
            .map_err(AppError::InternalError.log_extend())?)
    }
}