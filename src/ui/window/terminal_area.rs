use gpui::*;
use gpui_component::{
    Root, TitleBar, WindowExt,
    button::Button,
    dialog::Dialog,
    input::{Input, InputState},
    v_flex,
};

use crate::{
    state::{
        app_state::AppState,
        terminal_manager::{self, TerminalManager},
    },
    ui::dialogs::UserDialogView,
};

pub struct TerminalArea {
    hostname: Entity<InputState>,
    port: Entity<InputState>,
    lable: Entity<InputState>,
    config: Entity<AppState>,
    terminal_manager: Entity<TerminalManager>,
}
impl TerminalArea {
    pub fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
        config_manager: Entity<AppState>,
        terminal_manager: Entity<TerminalManager>,
    ) -> Self {
        Self {
            hostname: cx.new(|cx| InputState::new(window, cx)),
            port: cx.new(|cx| InputState::new(window, cx)),
            lable: cx.new(|cx| InputState::new(window, cx)),
            terminal_manager,
            config: config_manager,
        }
    }
}
impl Render for TerminalArea {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // let view = cx.entity();
        let config = self.config.read(cx).config_manager.current.theme.font_size;
        let list = self.terminal_manager.read(cx).session_manager.read(cx);

        div()
            .child(
                div().h_8().w_full().flex().flex_row().children(
                    list.sessions
                        .iter()
                        .enumerate()
                        .map(|(_idx, (_sesssion_id, session))| {
                            let label = (*session).name.clone();
                            div().child(label)
                        }),
                ),
            )
            .child(
                Button::new("")
                    .label("label")
                    .on_click(cx.listener(|e, eq, ew, cx| {
                        open_session_config_window(cx);
                    })),
            )
            .child(div().child(config.to_string()))
            .child(
                Button::new("qew")
                    .label("字体增大")
                    .text_size(px(config))
                    .on_click(cx.listener(|this, e, window, cx| {
                        this.config.update(cx, |c, cx| {
                            c.config_manager.update(|config| {
                                config.theme.font_size += 1.;
                            });

                            cx.notify();
                        })
                    })),
            )
            .child(div().text_size(px(config)).child(text!("this is text !!!")))
    }
}
fn open_session_config_window(cx: &mut App) {
    // 使用 cx.on_window_opened 或直接 open_window

    let bounds = Bounds::centered(None, size(px(800.), px(400.0)), cx);
    cx.spawn(async move |cx| {
        cx.open_window(
            WindowOptions {
                // ⭐ 关键：设置窗口层级为浮动（始终在普通窗口上方）
                kind: WindowKind::Dialog,
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitleBar::title_bar_options()),
                ..Default::default()
            },
            |window, cx| {
                let view = cx.new(|cx| UserDialogView::new(window, cx));
                cx.new(|cx| Root::new(view, window, cx))
            },
        )
        .expect("Failed to open window");
    })
    .detach();
}
