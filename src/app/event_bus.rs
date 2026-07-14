use tokio::sync::broadcast::Sender;

use crate::terminal::session::{SessionConfig, SessionId};

pub enum AppEvent {
    // UI事件
    CreateSession(SessionConfig),

    CloseSession(SessionId),

    Connect(SessionId),

    Disconnect(SessionId),

    // Terminal输入
    TerminalInput { id: SessionId, data: String },

    // Backend事件
    Connected { id: SessionId },

    Disconnected { id: SessionId },

    Output { id: SessionId, data: String },

    Error { id: SessionId, message: String },
}
pub struct EventBus {
    sender: Sender<AppEvent>,
}
