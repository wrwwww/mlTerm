use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
#[derive(Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub theme: ThemeConfig,

    pub terminal: TerminalConfig,

    pub ssh: SshConfig,

    pub appearance: AppearanceConfig,

    pub editor: EditorConfig,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub font_size: f32,
}
impl Default for ThemeConfig {
    fn default() -> Self {
        Self { font_size: 45. }
    }
}
#[derive(Clone, Serialize, Deserialize)]
pub struct TerminalConfig {}
impl Default for TerminalConfig {
    fn default() -> Self {
        Self {}
    }
}
#[derive(Clone, Serialize, Deserialize)]
pub struct SshConfig {}
impl Default for SshConfig {
    fn default() -> Self {
        Self {}
    }
}
#[derive(Clone, Serialize, Deserialize)]
pub struct AppearanceConfig {}
impl Default for AppearanceConfig {
    fn default() -> Self {
        Self {}
    }
}
#[derive(Clone, Serialize, Deserialize)]
pub struct EditorConfig {}
impl Default for EditorConfig {
    fn default() -> Self {
        Self {}
    }
}
pub struct ConfigManager {
    config_dir: PathBuf,
    main_path: PathBuf,
    backup_path: PathBuf,
    pub current: AppConfig,
}
impl ConfigManager {
    /// 初始化管理器（不加载配置，仅设置路径）
    pub fn new(app_name: &str) -> Result<Self> {
        let config_dir = dirs::config_dir()
            .context("无法获取系统配置目录")?
            .join(app_name);

        fs::create_dir_all(&config_dir)?;

        let main_path = config_dir.join("config.json");
        let backup_path = config_dir.join("config.bak.json");

        Ok(Self {
            config_dir,
            main_path,
            backup_path,
            current: AppConfig::default(), // 先填充默认值
        })
    }
    /// 从指定路径加载并校验配置
    fn load_from_path(&self, path: &Path) -> Result<AppConfig> {
        let content =
            fs::read_to_string(path).with_context(|| format!("读取配置文件失败: {:?}", path))?;

        let config: AppConfig =
            serde_json::from_str(&content).with_context(|| format!("解析JSON失败: {:?}", path))?;

        // 执行校验
        self.validate(&config)?;
        Ok(config)
    }

    /// 配置校验（自定义规则）
    fn validate(&self, config: &AppConfig) -> Result<()> {
        Ok(())
    }

    /// 加载配置（同步，在启动时调用）
    pub fn load(&mut self) -> Result<()> {
        // 1. 尝试加载主配置
        let config = match self.load_from_path(&self.main_path) {
            Ok(cfg) => {
                // 加载成功，更新备份
                if let Err(e) = self.save_backup(&cfg) {
                    warn!("备份配置失败: {}", e);
                }
                cfg
            }
            Err(e) => {
                warn!("主配置加载失败: {}", e);
                // 2. 尝试加载备份
                match self.load_from_path(&self.backup_path) {
                    Ok(cfg) => {
                        info!("已从备份恢复配置");
                        // 将备份写回主文件（修复启动）
                        self.save_main(&cfg)?;
                        cfg
                    }
                    Err(e) => {
                        error!("备份配置也加载失败: {}", e);
                        // 3. 终极兜底：使用默认值
                        warn!("使用出厂默认配置启动");
                        AppConfig::default()
                    }
                }
            }
        };

        self.current = config;
        Ok(())
    }
    /// 保存配置到主文件
    fn save_main(&self, config: &AppConfig) -> Result<()> {
        let json = serde_json::to_string_pretty(config)?;
        // 原子写入：先写临时文件，再重命名
        let temp_path = self.main_path.with_extension("tmp");
        fs::write(&temp_path, json)?;
        fs::rename(&temp_path, &self.main_path)?;
        info!("保存配置到主文件");
        Ok(())
    }

    /// 保存备份
    fn save_backup(&self, config: &AppConfig) -> Result<()> {
        let json = serde_json::to_string_pretty(config)?;
        fs::write(&self.backup_path, json)?;
        Ok(())
    }

    /// 获取当前配置（只读引用）
    pub fn get_config(&self) -> AppConfig {
        self.current.clone()
    }

    /// 热重载（在GPUI事件循环中调用）
    pub fn reload(&mut self) -> Result<()> {
        info!("正在热重载配置...");
        self.load()?; // 复用load逻辑
        Ok(())
    }
    pub fn save() {}
    pub fn update(&mut self, f: impl FnOnce(&mut AppConfig)) {
        let _ = self.save_backup(&self.current);
        f(&mut self.current);
        let _ = self.save_main(&self.current);
    }
    pub fn config() {}

    pub fn subscribe() {}
}
