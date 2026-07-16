use anyhow::Ok;
use gpui::*;
use gpui_component::{Root, TitleBar};
use log::info;

use crate::{actor::ActorSystem, state::config_manager::ConfigManager, ui::window::index::AppRoot};
/// Application - 程序的生命周期管理者
///
/// 职责：
/// - 管理全局配置
/// - 管理资源（主题、字体）
/// - 管理主窗口
/// - 管理会话管理器
pub struct Application {
    inner: Option<gpui::Application>,
    actor_system: std::sync::Arc<ActorSystem>,
}

impl Application {
    pub fn new(actor_system: std::sync::Arc<ActorSystem>) -> Result<Self> {
        info!("Creating Application...");

        let this = Self {
            inner: Some(gpui_platform::application()),

            actor_system,
        };
        Ok(this)
    }

    pub fn init_resources(&mut self) -> Result<()> {
        info!("Initializing resources...");
        self.inner = self
            .inner
            .take()
            .map(|e| e.with_assets(gpui_component_assets::Assets));
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        info!("Starting event loop...");
        let config_manager = crate::app::application::Application::load_config().unwrap();
        let actor_system = self.actor_system.clone();
        self.inner.take().map(|e| {
            e.run(move |cx| {
                gpui_component::init(cx);
                let bounds = Bounds::centered(None, size(px(1000.), px(600.0)), cx);
                cx.spawn(async move |cx| {
                    cx.open_window(
                        WindowOptions {
                            window_bounds: Some(WindowBounds::Windowed(bounds)),
                            titlebar: Some(TitleBar::title_bar_options()),
                            ..Default::default()
                        },
                        |window, cx| {
                            let view = cx.new(|cx| {
                                AppRoot::new(window, cx, config_manager, actor_system.clone())
                            });
                            cx.new(|cx| Root::new(view, window, cx))
                        },
                    )
                    .expect("Failed to open window");
                })
                .detach();
            })
        });
        Ok(())
    }

    pub fn load_config() -> Result<ConfigManager> {
        // -------- 第3步：创建配置管理器 --------

        let mut config_manager = ConfigManager::new("my_app")?;

        // -------- 第4步：同步加载核心配置 --------
        // 这个load很快（<10ms），可以阻塞启动
        if let Err(e) = config_manager.load() {
            // 即使加载失败，由于内部有兜底默认值，可以继续运行
            eprintln!("配置加载警告: {}", e);
        }

        Ok(config_manager)
    }
}
