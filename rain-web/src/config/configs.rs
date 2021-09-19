use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use std::sync::Arc;
use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
use sqlx::{ConnectOptions, MySql, Pool};
use log::LevelFilter;
use std::{env::current_dir};
use std::{path::PathBuf, time::Duration};
use anyhow::Context;


/// 配置文件目录
pub const CONFIG_PATH: &str = "resources/";
pub const SERVER_CONFIG_PATH: &str = "rain-web/resources/";

/// 配置文件默认文件
pub const DEFAULT_CONFIG: &str = "base";

/// 配置环境标识
pub const SERVER_ENVIRONMENT: &str = "SERVER_ENVIRONMENT";

/// 环境变量覆盖配置文件前缀
pub const SERVER_PREFIX: &str = "SERVER";

/// 环境变量覆盖配置文件分隔符
pub const SEPARATOR: &str = "_";

/// 默认健康检查地址
pub const HEALTH_CHECK: &str = "/health_check";

/// 配置项结构体
#[derive(Deserialize, Clone, Debug)]
pub struct Configs {
    pub server: ServerConfig,
    pub graphql: GraphQlConfig,
    pub database: DatabaseConfig,
}

impl Configs {
    /// 初始化配置文件
    pub fn init_config() -> anyhow::Result<Arc<Configs>> {
        // 加载环境变量
        dotenv::dotenv().ok();

        // 加载配置文件
        let mut settings = config::Config::default();

        let config_dir = get_config_dir()?;

        settings
            .merge(config::File::from(config_dir.join(DEFAULT_CONFIG)))
            .context(format!("加载默认配置文件:[{}] 失败!", DEFAULT_CONFIG))?;

        // 读取当前环境标志
        let environment = dotenv::var(SERVER_ENVIRONMENT)
            .context(format!("读取当前环境标志:[{}] 失败!", SERVER_ENVIRONMENT))?;

        settings
            .merge(config::File::from(config_dir.join(&environment)))
            .context(format!("加载自定义配置文件:[{}] 失败!", &environment))?;

        // 从环境变量或.env中添加设置（以APP前缀和'__'作为分隔符）
        // APP_SERVER_PORT = 5001 将覆盖 ApplicationConfig.server.port
        settings.merge(config::Environment::with_prefix(SERVER_PREFIX).separator(SEPARATOR))?;

        // 将读取的配置文件转换为配置文件结构体
        let config = settings.try_into().context("配置文件转换错误!")?;

        Ok(Arc::new(config))
    }
}

/// 服务配置
#[derive(Deserialize, Clone, Debug)]
pub struct ServerConfig {
    pub name: String,
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub context_path: Option<String>,
    pub health_check: Option<String>,
}

impl ServerConfig {
    /// 获取服务地址
    pub fn get_address(&self) -> String {
        format!("{}:{}", &self.host, &self.port)
    }

    /// 获取健康检查地址
    pub fn get_health_check(&self) -> String {
        if let Some(path) = &self.health_check {
            path.clone()
        } else {
            String::from(HEALTH_CHECK)
        }
    }
}

/// Graphql配置
#[derive(Deserialize, Clone, Debug)]
pub struct GraphQlConfig {
    pub path: String,
    pub tracing: Option<bool>,
    pub graphiql: GraphiQlConfig,
}

/// Graphiql配置
#[derive(Deserialize, Clone, Debug)]
pub struct GraphiQlConfig {
    pub path: String,
    pub enable: Option<bool>,
}

/// 数据库配置
#[derive(Deserialize, Clone, Debug)]
pub struct DatabaseConfig {
    pub username: String,
    pub password: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

impl DatabaseConfig {
    /// 初始化数据库连接池
    pub async fn init(config: &DatabaseConfig) -> anyhow::Result<Arc<Pool<MySql>>> {
        let mut options = MySqlConnectOptions::new()
            .username(&config.username)
            .password(&config.password)
            .host(&config.host)
            .port(config.port)
            .database(&config.database_name);
        // 设置 sql 日志级别
        options.log_statements(LevelFilter::Debug);
        let pool: Pool<MySql> = MySqlPoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .idle_timeout(Duration::from_secs(600))//10 分钟
            .max_lifetime(Duration::from_secs(1800))//30 分钟
            .connect_timeout(Duration::from_secs(30))//30 秒
            .connect_with(options)
            .await?;
        log::info!("初始化 '数据库' 完成");
        Ok(Arc::new(pool))
    }
}

/// 获取配置文件路径
fn get_config_dir() -> anyhow::Result<PathBuf> {
    let base_path = current_dir().context("无法确定当前目录")?;

    let mut config_dir = base_path.join(CONFIG_PATH);

    if !config_dir.as_path().exists() {
        config_dir = base_path.join(SERVER_CONFIG_PATH);
    };
    Ok(config_dir)
}