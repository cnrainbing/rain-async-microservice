use async_graphql::{Schema, EmptySubscription};
use async_graphql::extensions::{ApolloTracing, Logger};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{Request, Response};
use actix_web::{web, HttpResponse, Result};
use queries::QueryRoot;
use mutations::MutationRoot;
use std::sync::Arc;

use crate::config::configs::{Configs, GraphQlConfig};
use crate::Context;

pub mod queries;
pub mod mutations;

/// 为了代码简洁, 定义 `ServiceSchema`
pub type ServiceSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

/// 定义返回
pub type GraphqlResult<T> = std::result::Result<T, async_graphql::Error>;

/// 创建 Schema
pub async fn build_schema(ctx: Arc<Context>, config: &GraphQlConfig) -> ServiceSchema {
    let builder = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
        .data(ctx)
        .extension(Logger);
    if config.tracing.unwrap_or(false) {
        builder.extension(ApolloTracing).finish()
    } else {
        builder.finish()
    }
}

/// Schema 执行
pub async fn graphql(schema: web::Data<ServiceSchema>, req: Request) -> Response {
    // 可以从actix的HttpRequest中手动取token到 graphql的request
    schema.execute(req.into_inner()).await.into()
}

/// 创建 GraphQLPlayground
pub async fn graphiql(config: web::Data<Arc<Configs>>) -> Result<HttpResponse> {
    let path = &config.graphql.path;
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new(path).subscription_endpoint(path),
        )))
}