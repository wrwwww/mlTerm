use tokio::sync::mpsc;
// messages.rs
use tokio_actors::ActorHandle;

use crate::{
    actor::{process_actor::ProcessActor, session_actor::SessionActor},
    terminal::session::Session,
};

#[derive(Clone)]
pub enum UIMessage {
    Input(String),
    DisplayOutput(String),
    SetSession(ActorHandle<SessionActor>),
}
// 从 Actor 系统到 UI 的消息
#[derive(Clone)]
pub enum SystemEvent {
    Output(Vec<u8>),
    Error(String),
    CommandComplete(String),
    TitleUpdate(String),
    ClearScreen,
    ProcessStarted(u32),
    ProcessTerminated,
}

// UI → Session 的消息
#[derive(Clone)]
pub enum SessionMessage {
    // SSH 相关
    ConnectSSH(Session),
    DisconnectSSH,
    ExecuteCommand(String),
    SendInput(Vec<u8>),

    // Actor 间通信
    SetProcess(ActorHandle<ProcessActor>),
    // SetUI(ActorHandle<UIActor>),
    SetUI(mpsc::UnboundedSender<SystemEvent>),
}

// Session → UI 的消息
#[derive(Debug, Clone)]
pub enum UIEvent {
    SSHConnected,
    SSHDisconnected,
    Output(Vec<u8>),
    Error(String),
    Status(String),
}

// Session → Process 的消息
#[derive(Clone)]
pub enum ProcessMessage {
    ExecuteCommand(String),
    SendInput(Vec<u8>),
    CloseSession,
    SetSession(ActorHandle<SessionActor>),
}
