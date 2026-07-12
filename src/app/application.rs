use std::sync::Arc;

use anyhow::Ok;
use gpui::*;
use gpui_component::{Root, TitleBar};
use log::info;

use crate::{app::config::ConfigManager, ui::window::main_window::AppRoot};
/// Application - 程序的生命周期管理者
///
/// 职责：
/// - 管理全局配置
/// - 管理资源（主题、字体）
/// - 管理主窗口
/// - 管理会话管理器
pub struct Application {
    inner: Option<gpui::Application>,
}

impl Application {
    pub fn new() -> Result<Self> {
        info!("Creating Application...");

        Ok(Self {
            // config: config,
            inner: Some(gpui_platform::application()),
            // state: Arc::new(state),
        })
    }

    pub fn init_resources(&mut self) -> Result<()> {
        info!("Initializing resources...");
        self.inner = self
            .inner
            .take()
            .map(|e| e.with_assets(gpui_component_assets::Assets));
        // 加载配置
        // self.load_config()?;

        Ok(())
    }

    pub fn create_main_window(&mut self) -> Result<()> {
        info!("Creating main window...");

        // let window = MainWindow::new(
        //     self.config.clone(),
        //     self.theme_manager.clone(),
        //     self.font_manager.clone(),
        //     self.session_manager.clone(),
        // )?;

        // self.main_window = Some(window);
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        info!("Starting event loop...");
        let config_manager = crate::app::application::Application::load_config().unwrap();
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
                            let view = cx.new(|cx| AppRoot::new(window, cx, config_manager));
                            cx.new(|cx| Root::new(view, window, cx))
                        },
                    )
                    .expect("Failed to open window");
                })
                .detach();
            })
        });
        Ok(())
        // // 显示主窗口
        // if let Some(window) = &self.main_window {
        //     window.show()?;
        // }

        // // 进入事件循环
        // self.event_loop.run()
    }

    // /// 更新配置并触发通知
    // pub fn update_config(&mut self, new_config: Arc<AppConfig>, cx: &mut Context<Self>) {
    //     // let old_config = self.current_config.clone();

    //     // // 更新内存中的配置
    //     // self.current_config = new_config.clone();

    //     // // 同时更新管理器中的配置（保持一致性）
    //     // if let Ok(mut mgr) = self.config_manager.lock() {
    //     //     mgr.update_memory_config(new_config.clone());
    //     // }

    //     // 发出事件通知所有订阅者
    //     // cx.emit(ConfigChangedEvent {
    //     //     old_config,
    //     //     new_config,
    //     // });

    //     // 标记模型已变更（触发重绘）
    //     cx.notify();
    // }

    // /// 热重载配置（从磁盘重新加载）
    // pub fn reload_config(&mut self, cx: &mut ModelContext<Self>) -> Result<()> {
    //     // 1. 让 ConfigManager 重新加载磁盘文件
    //     let new_config = {
    //         let mut mgr = self
    //             .config_manager
    //             .lock()
    //             .map_err(|_| anyhow::anyhow!("Failed to lock config manager"))?;

    //         mgr.reload()?; // 重新读取磁盘
    //         mgr.get_config() // 获取新配置
    //     };

    //     // 2. 验证配置变化（可选：如果没变化，避免不必要的刷新）
    //     if Arc::ptr_eq(&self.current_config, &new_config) {
    //         return Ok(()); // 配置没变，不触发更新
    //     }

    //     // 3. 更新并通知
    //     self.update_config(new_config, cx);

    //     Ok(())
    // }

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
