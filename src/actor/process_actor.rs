use tokio_actors::{Actor, ActorContext, ActorHandle, ActorResult};

use crate::actor::{messages::ProcessMessage, session_actor::SessionActor};
// Process Actor - 管理子进程
#[derive(Default)]
pub struct ProcessActor {
    session_actor: Option<ActorHandle<SessionActor>>,
}
impl ProcessActor {
    pub fn new() -> Self {
        Self {
            session_actor: None,
        }
    }
}

impl Actor for ProcessActor {
    type Message = ProcessMessage;
    type Response = String;

    async fn handle(
        &mut self,
        msg: ProcessMessage,
        _ctx: &mut ActorContext<Self>,
    ) -> ActorResult<String> {
        match msg {
            ProcessMessage::ExecuteCommand(_) => todo!(),
            ProcessMessage::SendInput(items) => todo!(),
            ProcessMessage::CloseSession => todo!(),
            ProcessMessage::SetSession(actor_handle) => {
                self.session_actor = Some(actor_handle);
                Ok("✅ Session Actor 已连接".to_string())
            }
        }
    }
}
