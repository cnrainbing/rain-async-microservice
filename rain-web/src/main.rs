use actix_web::rt::time::Instant;
use rain_web::config::configs::{Configs,LogConfig};
use rain_web::Application;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let instant = Instant::now();
    // 初始化配置
    let configs = Configs::init_config()?;

    // 初始日志
    LogConfig::init(&configs.log)?;

    // 初始化服务器
    let application = Application::build(configs).await?;

    log::info!("🎉Started Application in {:.3?}", instant.elapsed());
    // 启动服务器
    application.run().await?;
    Ok(())
}
