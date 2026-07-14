use gpui::*;

use crate::state::terminal_manager::TerminalManager;

pub struct Sidebar {
    manager: Entity<TerminalManager>,
}

impl Sidebar {
    pub fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
        manager: Entity<TerminalManager>,
    ) -> Self {
        Self { manager }
    }
}

impl Render for Sidebar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let manager = self.manager.read(cx);
        let sessions = &manager.session_manager.read(cx).sessions;

        div()
            .w_full()
            .h(px(400.0))
            .flex()
            .flex_col()
            .gap(px(4.0))
            .p(px(8.0))
            // .bg(rgb(0x1e1e1e))
            .child(
                div()
                    .child("会话列表")
                    // .text_color(rgb(0xffffff))
                    .mb(px(8.0)),
            )
            .children(
                sessions
                    .iter()
                    .enumerate()
                    .map(|(index, (session_id, session))| {
                        // let is_selected = Some(index) == manager.selected_index;
                        let is_selected = true;
                        let label = (*session).name.clone();
                        let hostname = session.get_hostname();

                        div()
                            .w_full()
                            .h(px(30.0))
                            .px(px(8.0))
                            .py(px(4.0))
                            .overflow_hidden()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .bg(if is_selected {
                                rgb(0x2d2d2d)
                            } else {
                                rgb(0x252525)
                            })
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener({
                                    let session_view_manager = self.manager.clone();
                                    let session_id = session_id.clone();
                                    let label = label.clone();
                                    move |a, event: &MouseDownEvent, c, cx| {
                                        if (*event).click_count == 2 {
                                            log::info!("{}", format!("双击了item,label:{}", label));
                                            session_view_manager.update(cx, |state, cx| {
                                                state.add(cx, session_id);
                                                cx.notify();
                                            })
                                        }
                                    }
                                }),
                            )
                            .hover(|style| style.bg(rgb(0x2d2d2d)))
                            .rounded(px(4.0))
                            .cursor_pointer()
                            // .on_click(cx.listener(move |this, _event, window, cx| {
                            //     // 更新选中的索引
                            //     let mut manager = this.manager.write(cx);
                            //     manager.selected_index = Some(index);
                            //     cx.notify(); // 通知 UI 更新
                            //     info!("Selected session: {}", label);
                            // }))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(4.0))
                                    .child(div().w(px(8.0)).h(px(8.0)).rounded_full().bg(
                                        if is_selected {
                                            rgb(0x4caf50)
                                        } else {
                                            rgb(0x666666)
                                        },
                                    ))
                                    .child(
                                        div()
                                            .text_color(if is_selected {
                                                rgb(0xffffff)
                                            } else {
                                                rgb(0xcccccc)
                                            })
                                            .child(label),
                                    )
                                    .child(
                                        div()
                                            .text_color(rgb(0x888888))
                                            .text_size(px(12.0))
                                            .child(format!("({})", hostname))
                                            .overflow_hidden(),
                                    ),
                            )
                            .into_any()
                    }),
            )
            .into_any()
    }
}
