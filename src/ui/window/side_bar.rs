use gpui::*;
use log::info;

pub trait Session {
    fn get_label(&self) -> SharedString;
    fn get_hostname(&self) -> SharedString;
}

pub struct SessionManager {
    sessions: Vec<Box<dyn Session>>,
    selected_index: Option<usize>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: vec![],
            selected_index: None,
        }
    }

    pub fn insert(&mut self, session: impl Session + 'static) {
        self.sessions.push(Box::new(session));
    }

    pub fn get_sessions(&self) -> &Vec<Box<dyn Session>> {
        &self.sessions
    }
}

struct SshSession {
    hostname: SharedString,
    label: SharedString,
}

impl SshSession {
    pub fn new(hostname: impl AsRef<str>, label: impl AsRef<str>) -> Self {
        Self {
            hostname: SharedString::new(hostname),
            label: SharedString::new(label),
        }
    }
}

impl Session for SshSession {
    fn get_label(&self) -> SharedString {
        if self.label.is_empty() {
            self.hostname.clone()
        } else {
            self.label.clone()
        }
    }

    fn get_hostname(&self) -> SharedString {
        self.hostname.clone()
    }
}

pub struct Sidebar {
    manager: Entity<SessionManager>,
}

impl Sidebar {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let manager = cx.new(|_cx| {
            let mut manager = SessionManager::new();
            manager.insert(SshSession::new("192.168.1.1", "Server 1"));
            manager.insert(SshSession::new("192.168.1.2", "Server 2"));
            manager.insert(SshSession::new("192.168.1.3", "DB Server"));
            manager.insert(SshSession::new("192.168.1.4", "Web Server"));
            manager.insert(SshSession::new("192.168.1.5", "Backup Server"));
            manager.insert(SshSession::new("192.168.1.6", "Test Server"));
            manager.insert(SshSession::new("192.168.1.7", ""));
            manager
        });

        Self { manager }
    }
}

impl Render for Sidebar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let manager = self.manager.read(cx);
        let sessions = manager.get_sessions();

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
            .children(sessions.iter().enumerate().map(|(index, session)| {
                let is_selected = Some(index) == manager.selected_index;
                let label = session.get_label();
                let hostname = session.get_hostname();

                div()
                    .w_full()
                    .h(px(30.0))
                    .px(px(8.0))
                    .py(px(4.0))
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .bg(if is_selected {
                        rgb(0x2d2d2d)
                    } else {
                        rgb(0x252525)
                    })
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
                            .child(
                                div()
                                    .w(px(8.0))
                                    .h(px(8.0))
                                    .rounded_full()
                                    .bg(if is_selected {
                                        rgb(0x4caf50)
                                    } else {
                                        rgb(0x666666)
                                    }),
                            )
                            .child(
                                div()
                                    .text_color(if is_selected {
                                        rgb(0xffffff)
                                    } else {
                                        rgb(0xcccccc)
                                    })
                                    .child(label.clone()),
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
            }))
            .into_any()
    }
}
