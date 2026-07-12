use gpui::*;

// pub struct MySettings {
//     pub theme: String,
//     pub font_size: Pixels,
//     pub language: String,
// }

// // 实现 Global trait
// impl Global for MySettings {}

// // 或者在初始化时提供默认值
// impl MySettings {
//     fn default() -> Self {
//         MySettings {
//             theme: "dark".into(),
//             font_size: px(14.0),
//             language: "zh-CN".into(),
//         }
//     }
// }

// use log::{error, info, warn};
// // config.rs
// use serde::{Deserialize, Serialize};
// use std::path::PathBuf;
// use std::sync::{Arc, Mutex};

// // ---------- 核心配置（启动必需） ----------
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct CoreConfig {
//     pub app_name: String,
//     pub log_level: String, // "debug", "info", "error"
//     pub database: DatabaseConfig,
//     pub theme: Theme,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct DatabaseConfig {
//     pub host: String,
//     pub port: u16,
//     pub pool_size: usize,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Theme {
//     pub dark_mode: bool,
//     pub accent_color: String,
// }

// // ---------- 用户偏好（非核心，可异步加载） ----------
// #[derive(Debug, Clone, Serialize, Deserialize, Default)]
// pub struct UserPreferences {
//     pub window_size: (u32, u32),
//     pub recent_files: Vec<PathBuf>,
//     pub last_opened_project: Option<PathBuf>,
// }

// // ---------- 完整配置聚合 ----------
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct AppConfig {
//     #[serde(flatten)]
//     pub core: CoreConfig,

//     #[serde(default)]
//     pub preferences: UserPreferences,
// }

// // 实现默认值（出厂设置）
// impl Default for CoreConfig {
//     fn default() -> Self {
//         Self {
//             app_name: "MyApp".to_string(),
//             log_level: "info".to_string(),
//             database: DatabaseConfig {
//                 host: "127.0.0.1".to_string(),
//                 port: 5432,
//                 pool_size: 10,
//             },
//             theme: Theme {
//                 dark_mode: false,
//                 accent_color: "#007AFF".to_string(),
//             },
//         }
//     }
// }

// impl Default for AppConfig {
//     fn default() -> Self {
//         Self {
//             core: CoreConfig::default(),
//             preferences: UserPreferences::default(),
//         }
//     }
// }
// // config_manager.rs
// use anyhow::{Context, Result};
// use serde_json;
// use std::fs;
// use std::path::Path;

// pub struct ConfigManager {
//     config_dir: PathBuf,
//     main_path: PathBuf,
//     backup_path: PathBuf,
//     current: Arc<AppConfig>, // 不可变共享
// }

// impl ConfigManager {
//     /// 初始化管理器（不加载配置，仅设置路径）
//     pub fn new(app_name: &str) -> Result<Self> {
//         let config_dir = dirs::config_dir()
//             .context("无法获取系统配置目录")?
//             .join(app_name);

//         fs::create_dir_all(&config_dir)?;

//         let main_path = config_dir.join("config.json");
//         let backup_path = config_dir.join("config.bak.json");

//         Ok(Self {
//             config_dir,
//             main_path,
//             backup_path,
//             current: Arc::new(AppConfig::default()), // 先填充默认值
//         })
//     }

//     /// 加载配置（同步，在启动时调用）
//     pub fn load(&mut self) -> Result<()> {
//         // 1. 尝试加载主配置
//         let config = match self.load_from_path(&self.main_path) {
//             Ok(cfg) => {
//                 // 加载成功，更新备份
//                 if let Err(e) = self.save_backup(&cfg) {
//                     warn!("备份配置失败: {}", e);
//                 }
//                 cfg
//             }
//             Err(e) => {
//                 warn!("主配置加载失败: {}", e);
//                 // 2. 尝试加载备份
//                 match self.load_from_path(&self.backup_path) {
//                     Ok(cfg) => {
//                         info!("已从备份恢复配置");
//                         // 将备份写回主文件（修复启动）
//                         self.save_main(&cfg)?;
//                         cfg
//                     }
//                     Err(e) => {
//                         error!("备份配置也加载失败: {}", e);
//                         // 3. 终极兜底：使用默认值
//                         warn!("使用出厂默认配置启动");
//                         AppConfig::default()
//                     }
//                 }
//             }
//         };

//         self.current = Arc::new(config);
//         Ok(())
//     }

//     /// 从指定路径加载并校验配置
//     fn load_from_path(&self, path: &Path) -> Result<AppConfig> {
//         let content =
//             fs::read_to_string(path).with_context(|| format!("读取配置文件失败: {:?}", path))?;

//         let config: AppConfig =
//             serde_json::from_str(&content).with_context(|| format!("解析JSON失败: {:?}", path))?;

//         // 执行校验
//         self.validate(&config)?;
//         Ok(config)
//     }

//     /// 配置校验（自定义规则）
//     fn validate(&self, config: &AppConfig) -> Result<()> {
//         if config.core.database.port == 0 {
//             anyhow::bail!("数据库端口不能为0");
//         }
//         if config.core.database.pool_size == 0 || config.core.database.pool_size > 100 {
//             anyhow::bail!("连接池大小必须在 1-100 之间");
//         }
//         Ok(())
//     }

//     /// 保存配置到主文件
//     fn save_main(&self, config: &AppConfig) -> Result<()> {
//         let json = serde_json::to_string_pretty(config)?;
//         // 原子写入：先写临时文件，再重命名
//         let temp_path = self.main_path.with_extension("tmp");
//         fs::write(&temp_path, json)?;
//         fs::rename(&temp_path, &self.main_path)?;
//         Ok(())
//     }

//     /// 保存备份
//     fn save_backup(&self, config: &AppConfig) -> Result<()> {
//         let json = serde_json::to_string_pretty(config)?;
//         fs::write(&self.backup_path, json)?;
//         Ok(())
//     }

//     /// 获取当前配置（只读引用）
//     pub fn get_config(&self) -> Arc<AppConfig> {
//         Arc::clone(&self.current)
//     }

//     /// 热重载（在GPUI事件循环中调用）
//     pub fn reload(&mut self) -> Result<()> {
//         info!("正在热重载配置...");
//         self.load()?; // 复用load逻辑
//         Ok(())
//     }

//     /// 保存用户偏好（非核心配置更新）
//     pub fn save_preferences(&self, preferences: UserPreferences) -> Result<()> {
//         // 合并更新：只修改preferences字段，保留core不变
//         // let mut new_config = (*self.current).clone();
//         // new_config.preferences = preferences;

//         // 原子写入
//         // self.save_main(&new_config)?;

//         // 更新内存（使用Arc::make_mut进行写时复制）
//         // 注意：这里需要mut self，但我们可以通过内部可变性实现，见下文优化
//         Ok(())
//     }
// }
