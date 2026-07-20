use std::io::Read;

use log::info;
use ssh2::Channel;
use tokio::net::TcpStream;
use tokio_actors::{Actor, ActorResult};

use crate::terminal::session::Session;
pub enum SshMessage {
    Connect,
    Execute(String),
    Resize(u16, u16),
    Disconnect,
}
pub struct SshActor {
    session: Session,
    channel: Option<Channel>,
}
impl SshActor {
    pub fn new(session: Session) -> Self {
        Self {
            session: session,
            channel: None,
        }
    }
}
impl Actor for SshActor {
    type Message = SshMessage;

    type Response = String;

    async fn handle(
        &mut self,
        msg: Self::Message,
        ctx: &mut tokio_actors::ActorContext<Self>,
    ) -> ActorResult<String> {
        match msg {
            SshMessage::Connect => Ok({
                let hostname = self.session.get_hostname();
                let port = self.session.config.port;
                let addr = format!("{}:{}", hostname, port);
                let tcp = TcpStream::connect(addr).await.unwrap();
                let mut sess = ssh2::Session::new().unwrap();
                sess.set_tcp_stream(tcp);
                sess.handshake().unwrap();
                info!("ssh handshake");
                match &self.session.config.auth_method {
                    crate::terminal::session::AuthMethod::Password {
                        remember,
                        username,
                        password,
                    } => {
                        sess.userauth_password(username, password).unwrap();
                    }
                    crate::terminal::session::AuthMethod::PublicKey {
                        private_key,
                        passphrase_secret_id,
                    } => todo!(),
                    crate::terminal::session::AuthMethod::KeyboardInteractive => todo!(),
                    crate::terminal::session::AuthMethod::GssApi => todo!(),
                }

                info!("ssh get password");
                if sess.authenticated() {
                    let mut channel = sess.channel_session().unwrap();
                    let res = channel.exec("ls").unwrap();
                    let mut s = String::new();
                    channel.read_to_string(&mut s).unwrap();
                    self.channel = Some(channel);
                    info!("ssh authenticated");
                    info!("{}", format!("ssh output:{}", s));
                }

                "".to_string()
            }),
            SshMessage::Execute(_) => Ok("".to_string()),
            SshMessage::Resize(_, _) => Ok("".to_string()),
            SshMessage::Disconnect => Ok("".to_string()),
        }
    }
}
