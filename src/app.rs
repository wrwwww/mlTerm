use gpui::*;

use crate::gui::main_window::MainWindow;
use crate::infrastructure::event_loop::EventLoop;
use crate::models::session_config::SessionConfig;
use crate::terminal::session_manager::SessionManager;
use crate::{
    appbar::AppBar, components::splitter, menus::AppMenuBar, sidebar::Sidebar, tabbar::TabBar,
    terminal::TerminalView,
};
use anyhow::Result;
use log::{debug, info};
use std::sync::Arc;
pub struct AppState {
    pub layout: LayoutState,
    pub sidebar: SidebarState,
    pub splitter: SplitterState,
    // pub tabs: TabsState,
    // pub terminal: TerminalState,
}
pub struct LayoutState {
    pub sidebar_width: f32,
    pub splitter_position: f32,
}
pub struct SidebarState {
    pub collapsed: bool,
    pub active_session: Option<String>,
}
pub struct SplitterState {
    pub dragging: bool,
    pub hover: bool,
}
// pub struct TabsState {
//     pub active: usize,
//     pub tabs: Vec<Tab>,
// }
// pub struct TerminalState {
//     pub buffers: Vec<Buffer>,
//     pub cursor: Cursor,
// }
pub struct AppRoot {
    sidebar: Entity<Sidebar>,
    tabs: Entity<TabBar>,
    terminal: Entity<TerminalView>,
    appbar: Entity<AppBar>,
    state: Entity<AppState>,
    menus: Entity<AppMenuBar>,
}
impl AppRoot {
    pub fn new(window: &Window, cx: &mut App) -> Self {
        Self {
            sidebar: cx.new(|cx| Sidebar::new(cx)),
            tabs: cx.new(|cx| TabBar::new(cx)),
            terminal: cx.new(|cx| TerminalView::new(cx)),
            appbar: cx.new(|cx| AppBar::new(cx)),
            state: cx.new(|cx| AppState {
                layout: LayoutState {
                    sidebar_width: 240.0,
                    splitter_position: 0.0,
                },
                sidebar: SidebarState {
                    collapsed: false,
                    active_session: None,
                },
                splitter: SplitterState {
                    dragging: false,
                    hover: false,
                },
                // tabs: TabsState {
                //     active: 0,
                //     tabs: Vec::new(),
                // },
                // terminal: TerminalState {
                //     buffers: Vec::new(),
                //     cursor: Cursor::default(),
                // },
            }),
            menus: cx.new(|cx| AppMenuBar::new(cx)),
        }
    }
}
impl Render for AppRoot {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .child(self.appbar.clone())
            .child(div().h_10().w_full().child(self.menus.clone()))
            .child(
                div()
                    .w_full()
                    .h_full()
                    .flex()
                    .flex_row()
                    .child(
                        // 左侧 session tree
                        div().w(px(240.0)).h_full().child(self.sidebar.clone()),
                    )
                    // .child()
                    .child(
                        // 右侧 main
                        div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .child(div().h(px(36.0)))
                            .child(div().child(self.tabs.clone()))
                            .child(div().flex_1().child(div().child(self.terminal.clone()))),
                    ),
            )
            .flex_auto()
    }
}

/// Application - 程序的生命周期管理者
///
/// 职责：
/// - 管理全局配置
/// - 管理资源（主题、字体）
/// - 管理主窗口
/// - 管理会话管理器
pub struct Application {
    config: Arc<AppConfig>,
    theme_manager: ThemeManager,
    font_manager: FontManager,
    main_window: Option<MainWindow>,
    session_manager: Arc<SessionManager>,
    event_loop: EventLoop,
}

impl Application {
    pub fn new() -> Result<Self> {
        info!("Creating Application...");

        Ok(Self {
            config: Arc::new(AppConfig::default()),
            theme_manager: ThemeManager::new(),
            font_manager: FontManager::new(),
            main_window: None,
            session_manager: Arc::new(SessionManager::new()),
            event_loop: EventLoop::new(),
        })
    }

    pub fn init_resources(&mut self) -> Result<()> {
        info!("Initializing resources...");

        // 加载配置
        self.load_config()?;

        // 加载主题
        self.theme_manager.load_theme("dark")?;
        debug!("Theme loaded: {}", self.theme_manager.current_theme());

        // 加载字体
        self.font_manager.load_fonts()?;
        debug!(
            "Fonts loaded: {} families",
            self.font_manager.families().len()
        );

        Ok(())
    }

    pub fn create_main_window(&mut self) -> Result<()> {
        info!("Creating main window...");

        let window = MainWindow::new(
            self.config.clone(),
            self.theme_manager.clone(),
            self.font_manager.clone(),
            self.session_manager.clone(),
        )?;

        self.main_window = Some(window);
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        info!("Starting event loop...");

        // 显示主窗口
        if let Some(window) = &self.main_window {
            window.show()?;
        }

        // 进入事件循环
        self.event_loop.run()
    }

    fn load_config(&mut self) -> Result<()> {
        // 从配置文件加载
        let config_path = dirs::config_dir()
            .unwrap()
            .join("myterminal")
            .join("config.toml");

        if config_path.exists() {
            let content = std::fs::read_to_string(config_path)?;
            self.config = Arc::new(toml::from_str(&content)?);
            info!("Configuration loaded successfully");
        } else {
            // 使用默认配置
            info!("Using default configuration");
        }

        Ok(())
    }
}

/// 应用配置
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct AppConfig {
    pub theme: String,
    pub font_family: String,
    pub font_size: f32,
    pub default_shell: String,
    pub start_on_login: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            font_family: "JetBrains Mono".to_string(),
            font_size: 14.0,
            default_shell: "/bin/bash".to_string(),
            start_on_login: false,
        }
    }
}

/// 主题管理器
#[derive(Clone)]
pub struct ThemeManager {
    current: String,
    colors: ThemeColors,
}

impl ThemeManager {
    pub fn new() -> Self {
        Self {
            current: "dark".to_string(),
            colors: ThemeColors::default(),
        }
    }

    pub fn load_theme(&mut self, name: &str) -> Result<()> {
        // 从 resources/themes/ 加载主题
        let theme_path = format!("resources/themes/{}.toml", name);
        // 实际实现中从文件加载
        self.current = name.to_string();
        Ok(())
    }

    pub fn current_theme(&self) -> &str {
        &self.current
    }

    pub fn colors(&self) -> &ThemeColors {
        &self.colors
    }
}

/// 主题颜色
#[derive(Debug, Clone)]
pub struct ThemeColors {
    pub background: String,
    pub foreground: String,
    pub cursor: String,
    pub selection: String,
    pub black: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub magenta: String,
    pub cyan: String,
    pub white: String,
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self {
            background: "#1e1e1e".to_string(),
            foreground: "#d4d4d4".to_string(),
            cursor: "#ffffff".to_string(),
            selection: "#264f78".to_string(),
            black: "#000000".to_string(),
            red: "#cd3131".to_string(),
            green: "#0dbc79".to_string(),
            yellow: "#e5e510".to_string(),
            blue: "#2472c8".to_string(),
            magenta: "#bc3fbc".to_string(),
            cyan: "#11a8cd".to_string(),
            white: "#e5e5e5".to_string(),
        }
    }
}

/// 字体管理器
#[derive(Clone)]
pub struct FontManager {
    families: Vec<String>,
    current_family: String,
    size: f32,
}

impl FontManager {
    pub fn new() -> Self {
        Self {
            families: vec!["JetBrains Mono".to_string(), "Fira Code".to_string()],
            current_family: "JetBrains Mono".to_string(),
            size: 14.0,
        }
    }

    pub fn load_fonts(&mut self) -> Result<()> {
        // 实际实现中加载系统字体
        Ok(())
    }

    pub fn families(&self) -> &[String] {
        &self.families
    }

    pub fn current_family(&self) -> &str {
        &self.current_family
    }

    pub fn size(&self) -> f32 {
        self.size
    }
}
