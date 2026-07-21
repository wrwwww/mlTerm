// messages.rs

use crate::terminal::session::TabId;

// 从 Actor 系统到 UI 的消息
#[derive(Clone)]
pub enum SystemEvent {
    Output { tab_id: TabId, bytes: Vec<u8> },
    Status { tab_id: String, text: String },
    Connected { tab_id: String },
    Error(String),
    CommandComplete(String),
    TitleUpdate { tab_id: String, title: String },
    ClearScreen,
    ProcessStarted(u32),
    ProcessTerminated,
}

pub enum SshMessage {
    Input(Vec<u8>),
    Resize(u16, u16),
    Disconnect,
}
