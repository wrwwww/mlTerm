use log::info;
mod actions;
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
mod workspace;
use app::*;

fn main() -> anyhow::Result<()> {
    // 1. 初始化基础设施（日志最先）
    initialize()?;

    info!("Starting MyTerminal...");

    // 2. 创建 Application（程序大脑）
    let mut app = app::application::Application::new()?;

    // 3. 初始化资源（配置、主题、字体）
    app.init_resources()?;

    // 4. 创建主窗口
    app.create_main_window()?;

    // 5. 运行事件循环
    app.run()
}
