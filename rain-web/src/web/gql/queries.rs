use async_graphql::{MergedObject,Object};
use crate::web::gql::GraphqlResult;

/// 定义查询根节点
#[derive(MergedObject, Default)]
pub struct QueryRoot(PingQuery,UsersQuery);

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
    async fn ping(&self) -> GraphqlResult<String> {
        Ok("pong".to_string())
    }
}