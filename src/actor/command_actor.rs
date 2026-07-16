use tokio_actors::{Actor, ActorContext, ActorResult};
// Command Actor - 执行命令
#[derive(Default)]
pub struct CommandActor {
    history: Vec<String>,
}

impl Actor for CommandActor {
    type Message = String;
    type Response = String;

    async fn handle(&mut self, msg: String, _ctx: &mut ActorContext<Self>) -> ActorResult<String> {
        self.history.push(msg.clone());
        // 这里应该实际执行命令
        Ok(format!("执行: {}", msg))
    }
}
