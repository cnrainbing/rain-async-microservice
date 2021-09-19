use async_graphql::{MergedObject,Object};
use crate::web::gql::GraphqlResult;

/// 变更根节点
#[derive(MergedObject, Default)]
pub struct MutationRoot(UsersMutation);

/// 用户变更 Mutation
#[derive(Default)]
pub struct UsersMutation;

#[Object]
impl UsersMutation {
    async fn ping(&self) -> GraphqlResult<String> {
        Ok("pong".to_string())
    }
}