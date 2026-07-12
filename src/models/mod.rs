use gpui::*;
use gpui_component::{button::Button, *};

mod profile;
mod session_config;
mod theme;

// 1. 定义状态
struct ModelState {
    is_display: bool,
}

// 为了能发送更丰富的事件，可以实现 EventEmitter
impl EventEmitter<()> for ModelState {}

pub struct SshModels {}
impl SshModels {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for SshModels {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::prelude::Context<Self>,
    ) -> impl gpui::prelude::IntoElement {
        div()
    }
}
