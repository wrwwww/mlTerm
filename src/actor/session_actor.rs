use std::collections::HashMap;

use crate::{
    actor::{
        messages::{SessionMessage, SystemEvent, UIEvent::SSHConnected},
        process_actor::ProcessActor,
        ssh_actor::{SshActor, SshMessage},
    },
    terminal::session::SessionId,
};
use log::info;
use tokio::sync::mpsc;
use tokio_actors::{Actor, ActorContext, ActorExt, ActorHandle, ActorResult};

// Session Actor - 管理会话状态
pub struct SessionActor {
    current_dir: String,
    process_actor: Option<ActorHandle<ProcessActor>>,
    ui_tx: Option<mpsc::UnboundedSender<SystemEvent>>,
    sessions: HashMap<SessionId, ActorHandle<SshActor>>,
}
impl SessionActor {
    pub fn new() -> Self {
        Self {
            current_dir: "".to_string(),
            process_actor: None,
            ui_tx: None,
            sessions: HashMap::new(),
        }
    }
}

impl Actor for SessionActor {
    type Message = SessionMessage;
    type Response = String;

    async fn handle(
        &mut self,
        msg: SessionMessage,
        _ctx: &mut ActorContext<Self>,
    ) -> ActorResult<String> {
        match msg {
            SessionMessage::ConnectSSH(session) => {
                info!("后台接受到前端发送过来的请求,{:#?}", session);
                let session_id = session.id.clone();
                let ssh_actor = SshActor::new(session);

                let ssh_handle = ssh_actor.spawn().await.unwrap();
                ssh_handle.notify(SshMessage::Connect).await.unwrap();
                self.sessions.insert(session_id, ssh_handle);
                Ok("✅ Session Actor 已连接".to_string())
            }
            SessionMessage::DisconnectSSH => Ok("✅ Session Actor 已连接".to_string()),
            SessionMessage::ExecuteCommand(_) => Ok("✅ Session Actor 已连接".to_string()),
            SessionMessage::SendInput(items) => Ok("✅ Session Actor 已连接".to_string()),
            SessionMessage::SetProcess(actor_handle) => Ok("✅ Session Actor 已连接".to_string()),
            SessionMessage::SetUI(ui_tx) => {
                self.ui_tx = Some(ui_tx);
                Ok("✅ Session Actor 已连接".to_string())
            }
        }
    }
}
