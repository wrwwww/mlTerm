use gpui::Context;

use crate::state::config_manager::ConfigManager;

pub struct AppState {
    pub config_manager: ConfigManager,
}

impl AppState {
    pub fn new(cx: &mut Context<'_, AppState>, config_manager: ConfigManager) -> Self {
        Self { config_manager }
    }
}
