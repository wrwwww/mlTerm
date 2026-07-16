use log::info;
mod actor;
mod app;
mod events;
mod models;
mod platform;
mod protocol;
mod services;
mod state;
mod terminal;
pub mod ui;
mod utils;
use app::*;

use crate::actor::ActorSystem;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    info!("🚀 初始化 Actor 系统...");
    let actor_system: std::sync::Arc<ActorSystem> = ActorSystem::new().await?;
    // 1. 初始化基础设施（日志最先）
    initialize()?;

    info!("Starting MyTerminal...");

    // 2. 创建 Application（程序大脑）
    let mut app = app::application::Application::new(actor_system.clone())?;

    // 3. 初始化资源（配置、主题、字体）
    app.init_resources()?;

    // 5. 运行事件循环
    app.run()
}
