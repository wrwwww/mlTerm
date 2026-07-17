use std::sync::mpsc::{Sender, SyncSender};

use gpui::*;

use gpui_component::button::{Button, ButtonVariants};
use gpui_component::checkbox::Checkbox;

use gpui_component::input::{Input, InputState};
use gpui_component::label::Label;
use gpui_component::scroll::ScrollableElement;
use gpui_component::tab::{Tab, TabBar};
use gpui_component::{GlobalState, TitleBar, WindowExt, h_flex, v_flex};
use gpui_rsx::rsx;

use crate::state::terminal_manager::TerminalManager;
use crate::terminal::session::{Session, SessionConfig, SessionId, SessionKind};

// ============================================================================
// Protocol Configs
// ============================================================================

pub struct SshConfig {
    pub host: Entity<InputState>,
    pub port: Entity<InputState>,
    pub username: Entity<InputState>,
    pub password: Entity<InputState>,
    pub remember_password: bool,
}

impl SshConfig {
    pub fn new(window: &mut Window, cx: &mut Context<UserDialogView>) -> Self {
        let host = cx.new(|cx| InputState::new(window, cx).placeholder("e.g. 192.168.1.100"));
        let port = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value("22")
                .placeholder("22")
        });
        let username = cx.new(|cx| InputState::new(window, cx).placeholder("e.g. root"));
        let password = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Password")
                .masked(true)
        });

        Self {
            host,
            port,
            username,
            password,
            remember_password: false,
        }
    }
}

pub struct TelnetConfig {
    pub host: Entity<InputState>,
    pub port: Entity<InputState>,
}

impl TelnetConfig {
    pub fn new(window: &mut Window, cx: &mut Context<UserDialogView>) -> Self {
        let host = cx.new(|cx| InputState::new(window, cx).placeholder("e.g. 192.168.1.100"));
        let port = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value("23")
                .placeholder("23")
        });

        Self { host, port }
    }
}

pub struct SerialConfig {
    pub port: Entity<InputState>,
    pub baud: Entity<InputState>,
}

impl SerialConfig {
    pub fn new(window: &mut Window, cx: &mut Context<UserDialogView>) -> Self {
        let port =
            cx.new(|cx| InputState::new(window, cx).placeholder("e.g. COM1 or /dev/ttyUSB0"));
        let baud = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value("115200")
                .placeholder("115200")
        });

        Self { port, baud }
    }
}

// ============================================================================
// UserDialogView
// ============================================================================

pub struct UserDialogView {
    selected_tab: usize,

    // appbar: Entity<AppBar>,
    session_name: Entity<InputState>,

    terminal_manager: Entity<TerminalManager>,
    ssh: SshConfig,
    telnet: TelnetConfig,
    serial: SerialConfig,
}

impl UserDialogView {
    pub fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
        terminal_manager: Entity<TerminalManager>,
    ) -> Self {
        let session_name =
            cx.new(|cx| InputState::new(window, cx).placeholder("Session Name (optional)"));

        Self {
            selected_tab: 0,
            terminal_manager,
            // appbar: cx.new(|cx| AppBar::new(cx)),
            session_name,

            ssh: SshConfig::new(window, cx),
            telnet: TelnetConfig::new(window, cx),
            serial: SerialConfig::new(window, cx),
        }
    }

    // ------------------------------------------------------------------------
    // Render Methods
    // ------------------------------------------------------------------------

    fn render_ssh_form(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        rsx! {
            <div class="flex flex-col gap-3 px-4 py-3">

                <div class="flex flex-col gap-1">
                        {self.labeled_input("Host:", self.ssh.host.clone(), window, cx)}
                        {self.labeled_input("Port:", self.ssh.port.clone(), window, cx)}
                        {self.labeled_input("Username:", self.ssh.username.clone(), window, cx)}
                        <div flex gap_1 >
                            <div>"密码"</div>
                            <div>{Input::new(&self.ssh.password)}</div>
                        </div>
                </div>

                <div class="flex flex-row gap-2 items-center">
                    {Checkbox::new("remember-pw")}
                    {Label::new("Remember password")}
                </div>
            </div>
        }
    }

    fn render_telnet_form(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_3()
            .px_4()
            .py_3()
            .child(self.labeled_input("Host:", self.telnet.host.clone(), window, cx))
            .child(self.labeled_input("Port:", self.telnet.port.clone(), window, cx))
    }

    fn render_serial_form(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_3()
            .px_4()
            .py_3()
            .child(self.labeled_input("Port:", self.serial.port.clone(), window, cx))
            .child(self.labeled_input("Baud Rate:", self.serial.baud.clone(), window, cx))
    }

    // ------------------------------------------------------------------------
    // Helpers
    // ------------------------------------------------------------------------

    fn labeled_input(
        &self,
        label: &str,
        input: Entity<InputState>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        v_flex()
            .gap_1()
            .child(Label::new(label))
            .child(Input::new(&input))
    }

    // ------------------------------------------------------------------------
    // Actions
    // ------------------------------------------------------------------------

    fn on_connect(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        // TODO: collect values from InputState and dispatch connection
        println!("Connect clicked — protocol index: {}", self.selected_tab);
        let session = Session {
            id: SessionId::new(),
            name: self.session_name.read(cx).text().to_string(),
            kind: SessionKind::Ssh,
            config: SessionConfig {
                hostname: self.ssh.host.read(cx).text().to_string(),
                port: 22,
                auth_method: crate::terminal::session::AuthMethod::Password {
                    remember: true,
                    username: "".to_string(),
                    password: "".to_string(),
                },
            },
            status: crate::terminal::session::SessionStatus::Connected,
        };
        self.terminal_manager.update(cx, |this, cx| {
            this.new_session(session, cx);
        });
        cx.notify();
        _window.remove_window();
    }

    fn on_cancel(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        // TODO: close dialog / remove view
        println!("Cancel clicked");
        cx.notify();
        _window.remove_window();
    }
}

// ============================================================================
// Render Implementation
// ============================================================================

impl Render for UserDialogView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let session_name = self.session_name.clone();

        div()
            .size_full()
            // .bg(rgb(0x1e1e2e))
            // .text_color(rgb(0xcdd6f4))
            .flex()
            .flex_col()
            .child(TitleBar::new().child(div().flex().items_center().gap_3().child("新建会话")))
            .child(
                v_flex()
                    .overflow_y_scrollbar()
                    .flex_1()
                    .gap_4()
                    .p_4()
                    // Session Name (shared)
                    .child(
                        v_flex()
                            .gap_1()
                            .child(Label::new("会话名称"))
                            .child(Input::new(&session_name)),
                    )
                    // Tab Bar
                    .child(
                        TabBar::new("protocol-tabs")
                            .selected_index(self.selected_tab)
                            .on_click(cx.listener(|this, index, _, cx| {
                                this.selected_tab = *index;
                                cx.notify();
                            }))
                            .child(Tab::new().label("SSH"))
                            .child(Tab::new().label("Telnet"))
                            .child(Tab::new().label("Serial")),
                    )
                    // Protocol-specific form
                    .child(match self.selected_tab {
                        0 => self.render_ssh_form(window, cx).into_any_element(),
                        1 => self.render_telnet_form(window, cx).into_any_element(),
                        2 => self.render_serial_form(window, cx).into_any_element(),
                        _ => div().into_any_element(),
                    })
                    // Bottom buttons
                    .child(
                        h_flex()
                            .justify_end()
                            .gap_3()
                            .child(
                                Button::new("cancel-btn")
                                    .label("取消")
                                    .on_click(cx.listener(|this, e, _window, cx| {
                                        this.on_cancel(_window, cx);
                                    })),
                            )
                            .child(Button::new("connect-btn").label("连接").primary().on_click(
                                cx.listener(|this, e, _window, cx| {
                                    this.on_connect(_window, cx);
                                }),
                            )),
                    ),
            )
    }
}
