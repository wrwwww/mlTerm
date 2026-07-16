// actor_system.rs
pub mod command_actor;
pub mod messages;
pub mod process_actor;
pub mod session_actor;
pub mod ssh_actor;

use std::sync::Arc;
use tokio_actors::{ActorExt, ActorHandle};

use crate::actor::{
    command_actor::CommandActor, messages::SessionMessage, process_actor::ProcessActor,
    session_actor::SessionActor,
};

pub struct ActorSystem {
    pub session_actor: ActorHandle<SessionActor>,
    pub command_actor: ActorHandle<CommandActor>,
    pub process_actor: ActorHandle<ProcessActor>,
}

impl ActorSystem {
    pub async fn new() -> Result<Arc<Self>, anyhow::Error> {
        // 启动所有 Actor
        let session = SessionActor::new().spawn().named("session_actor").await?;

        let command = CommandActor::default()
            .spawn()
            .named("command_actor")
            .await?;

        let process = ProcessActor::default()
            .spawn()
            .named("process_actor")
            .await?;
        session
            .notify(SessionMessage::SetProcess(process.clone()))
            .await?;
        let system = Arc::new(Self {
            session_actor: session,
            command_actor: command,
            process_actor: process,
        });

        // 建立 Actor 之间的连接
        Ok(system)
    }
}
