use std::io;
use std::sync::Arc;
use sqlx::{MySqlPool, MySql, Pool};
use async_graphql::Context as GraphQLContext;

use actix_web::dev::Server;
use actix_web::middleware::{Logger,Compress};
use actix_web::web::{resource, ServiceConfig, Data};
use actix_web::{App, middleware,guard, HttpServer};
use actix_web_requestid::{RequestIDService};
use guard::{Get, Post};

use crate::web::gql::{GraphqlResult, graphiql, graphql, ServiceSchema};
use crate::web::gql;
use crate::config::configs::{Configs, DatabaseConfig};
use rain_model::constant::{USERNAME_REGEX, EMAIL_REGEX};
use crate::web::router::health_check::health_check;


/// 工程内部mod
pub mod config;
pub mod web;
pub mod error;

/// 全局的 state
pub struct Context {
    // 数据库连接池
    pool: Arc<MySqlPool>,
}

impl Context {
    // 通过 GraphQLContext 获取 数据库连接池
    pub fn get_pool(ctx: &GraphQLContext<'_>) -> GraphqlResult<Arc<Pool<MySql>>> {
        Ok(ctx.data::<Arc<Context>>()?.pool.clone())
    }
}

/// http server application
pub struct Application {
    server: Server,
}

impl Application {
    /// 构建 服务器
    pub async fn build(configs: Arc<Configs>) -> anyhow::Result<Application> {
        // 初始化静态常量
        lazy_static::initialize(&EMAIL_REGEX);
        lazy_static::initialize(&USERNAME_REGEX);
        log::info!("初始化 '静态常量' 完成");

        // 链接数据库
        let pool: Arc<Pool<MySql>> = DatabaseConfig::init(&configs.database).await?;
        let context: Arc<Context> = Arc::new(Context { pool });

        // 初始化 GraphQL schema.
        let schema = gql::build_schema(context.clone(), &configs.graphql).await;
        log::info!(r#"初始化 'GraphQL Schema' 完成! "#);

        let address = configs.server.get_address();
        let enable = &configs.graphql.graphiql.enable;
        if enable.unwrap_or(false) {
            log::info!(
                "🚀GraphQL UI: http://{}{}",
                address,
                &configs.graphql.graphiql.path
            );
        }

        let server: Server = build_actix_server(configs, address, context, schema)?;

        Ok(Application { server })
    }

    /// 启动
    pub async fn run(self) -> anyhow::Result<(), io::Error> {
        self.server.await
    }
}

/// 构建 服务器
fn build_actix_server(
    configs: Arc<Configs>,
    address: String,
    context: Arc<Context>,
    schema: ServiceSchema,
) -> anyhow::Result<Server> {
    let server: Server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "1.0.0"))
            .wrap(RequestIDService::default())
            .wrap(Compress::default())
            .wrap(Logger::default())
            .app_data(Data::new(configs.clone()))
            .app_data(Data::new(context.clone()))
            .app_data(Data::new(schema.clone()))
            .configure(|cfg| register_service(cfg, configs.clone()))
    })
        .bind(address)?
        .workers(64)
        .max_connections(65535)
        .run();
    Ok(server)
}

/// 注册路由 每一个worker都会注册一下
fn register_service(cfg: &mut ServiceConfig, configs: Arc<Configs>) {
    let graphql_config = &configs.graphql;

    // graphql 入口
    cfg.service(resource(&graphql_config.path).guard(Post()).to(graphql));

    // rest 健康检查
    cfg.service(
        resource(configs.server.get_health_check())
            .guard(Get())
            .to(health_check),
    );

    // 开发环境的工具
    let enable = graphql_config.graphiql.enable;
    if enable.unwrap_or(false) {
        cfg.service(
            resource(&graphql_config.graphiql.path)
                .guard(Get())
                .to(graphiql),
        );
    }
}