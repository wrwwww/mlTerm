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
    inner: gpui::application::Application,
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
            inner: gpui_platform::application(),
        })
    }

    pub fn init_resources(&mut self) -> Result<()> {
        info!("Initializing resources...");
        self.inner.with_assets(gpui_component_assets::Assets);
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

        self.inner.run(move |cx| {
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
                        let view = cx.new(|cx| AppRoot::new(window, cx));
                        cx.new(|cx| Root::new(view, window, cx))
                    },
                )
                .expect("Failed to open window");
            })
            .detach();
        });

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
