use gpui::*;
use gpui_component::{button::Button, input::InputState};

use crate::state::{app_state::AppState, terminal_manager::TerminalManager};

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
