use std::sync::mpsc;

use gpui::*;
use gpui_component::{
    Root, TitleBar,
    button::{Button, ButtonVariants},
};
use log::info;

use crate::{
    actor::{ActorSystem, messages::SessionMessage},
    state::terminal_manager::TerminalManager,
    ui::dialogs::UserDialogView,
};

pub struct Sidebar {
    terminal_manager: Entity<TerminalManager>,
    actor_system: std::sync::Arc<ActorSystem>,
}

impl Sidebar {
    pub fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
        terminal_manager: Entity<TerminalManager>,
        actor_system: std::sync::Arc<ActorSystem>,
    ) -> Self {
        Self {
            terminal_manager,
            actor_system,
        }
    }
}

impl Render for Sidebar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let manager = self.terminal_manager.read(cx);
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
                                    let session_view_manager = self.terminal_manager.clone();
                                    let actor_system = self.actor_system.clone();
                                    let session_id = session_id.clone();
                                    let label = label.clone();
                                    move |a, event: &MouseDownEvent, c, cx| {
                                        if (*event).click_count == 2 {
                                            log::info!("{}", format!("双击了item,label:{}", label));

                                            session_view_manager.update(cx, |state, cx| {
                                                let actor_system = actor_system.clone();
                                                let session = state.add(cx, session_id).unwrap();
                                                cx.spawn(async move |_, _| {
                                                    actor_system
                                                        .session_actor
                                                        .notify(SessionMessage::ConnectSSH(session))
                                                        .await
                                                        .unwrap();
                                                })
                                                .detach();
                                                cx.notify();
                                            })
                                        }
                                    }
                                }),
                            )
                            .hover(|style| style.bg(rgb(0x2d2d2d)))
                            .rounded(px(4.0))
                            .cursor_pointer()
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
            .child(
                div()
                    .w_full()
                    // .h(px(30.0))
                    // .px(px(8.0))
                    // .py(px(4.0))
                    .overflow_hidden()
                    .flex()
                    .items_center()
                    // .gap(px(8.0))
                    // .bg(rgb(0x2d2d2d))
                    .rounded(px(4.0))
                    .cursor_pointer()
                    .child(
                        Button::new("id")
                            .w_full()
                            .label("新建会话")
                            .on_click(cx.listener({
                                let terminal_manager = self.terminal_manager.clone();
                                move |e, eq, ew, cx| {
                                    let manager = terminal_manager.clone(); // 关键：在这里克隆
                                    open_session_config_window(cx, manager);
                                }
                            })),
                    )
                    .into_any(),
            )
    }
}

fn open_session_config_window(cx: &mut App, terminal_manager: Entity<TerminalManager>) {
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
                let view = cx.new(|cx| UserDialogView::new(window, cx, terminal_manager));
                cx.new(|cx| Root::new(view, window, cx))
            },
        )
        .expect("Failed to open window");
    })
    .detach();
}
