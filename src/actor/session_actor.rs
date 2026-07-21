use crate::{
    actor::messages::{SshMessage, SystemEvent},
    terminal::session::{Session, TabId},
};
use std::sync::Arc;

use anyhow::Context;

use russh::{
    ChannelMsg,
    client::{self},
    keys::ssh_key,
};
use tokio::sync::{
    Mutex,
    mpsc::{self, Sender, UnboundedReceiver},
};

pub fn open_session(
    session: Session,
    tab_id: TabId,
    events_tx: mpsc::Sender<SystemEvent>,
) -> anyhow::Result<BackendTx> {
    // let (tx, mut rx) = mpsc::unbounded_channel::<SystemEvent>();
    // let (, events_rx) = mpsc::channel::<SystemEvent>(10);

    let (cmd_tx, cmd_rx) = mpsc::unbounded_channel::<SshMessage>();
    open_session_terminal(events_tx, session, tab_id, cmd_rx);
    Ok(BackendTx::Ssh(cmd_tx))
}
#[derive(Clone)]
pub enum BackendTx {
    Local(Sender<SshMessage>),
    Ssh(tokio::sync::mpsc::UnboundedSender<SshMessage>),
    Serial(tokio::sync::mpsc::UnboundedSender<SshMessage>),
}

impl BackendTx {
    pub fn send(&self, command: SshMessage) {
        match self {
            Self::Local(tx) => {
                let _ = tx.send(command);
            }
            Self::Ssh(tx) => {
                let _ = tx.send(command);
            }
            Self::Serial(tx) => {
                let _ = tx.send(command);
            }
        }
    }
}

pub fn open_session_terminal(
    events: mpsc::Sender<SystemEvent>,
    session: Session,
    tab_id: TabId,
    mut cmd_rx: UnboundedReceiver<SshMessage>,
) {
    tokio::spawn(async move {
        // connect
        let addr = format!("{}:{}", session.config.hostname, session.config.port);
        let stream = tokio::net::TcpStream::connect(&addr)
            .await
            .map_err(|e| anyhow::anyhow!("HTTP proxy connection failed: {}", e))
            .unwrap();
        let config = Arc::new(client::Config::default());
        let mut handle = client::connect_stream(config, stream, ClientHandler)
            .await
            .unwrap();

        // auth
        let a = match session.config.auth_method {
            crate::terminal::session::AuthMethod::Password {
                remember,
                username,
                password,
            } => handle
                .authenticate_password(username, password)
                .await
                .context("context")
                .unwrap(),
            crate::terminal::session::AuthMethod::PublicKey {
                private_key,
                passphrase_secret_id,
            } => handle
                .authenticate_password("".to_string(), "".to_string())
                .await
                .context("context")
                .unwrap(),
            crate::terminal::session::AuthMethod::KeyboardInteractive => handle
                .authenticate_password("".to_string(), "".to_string())
                .await
                .context("context")
                .unwrap(),
            crate::terminal::session::AuthMethod::GssApi => handle
                .authenticate_password("".to_string(), "".to_string())
                .await
                .context("context")
                .unwrap(),
        };
        if !a.success() {
            // 没有成功，发送授权失败事件
        }
        let handle = Arc::new(Mutex::new(handle));
        let mut channel = handle.lock().await.channel_open_session().await.unwrap();
        channel
            .request_pty(true, "xterm-256color", 0, 0, 0, 0, &[])
            .await
            .context("")
            .unwrap();
        channel
            .request_shell(true)
            .await
            .context("request shell")
            .unwrap();
        // channel 创建并初始化成功
        loop {
            tokio::select! {
                command = cmd_rx.recv() => {
                    match command{
                        Some(SshMessage::Input(data)) => {
                            // 将终端发送过来的命令，发送给ssh服务器
                            if let Err(e) =  channel.data_bytes(data).await{

                            };
                        },
                        Some(SshMessage::Resize(col,row )) => {
                           let _ = channel.window_change(col.into(), row.into(), 0, 0).await;
                        },
                        Some(SshMessage::Disconnect)|None => {
                            channel.eof();
                        },
                    }
                }
                message=channel.wait()=>{
                    match message{
                       Some(ChannelMsg::Data { data }) | Some(ChannelMsg::ExtendedData { data, ext: _ }) => {
                        let _ = events.send(SystemEvent::Output{
                            tab_id: tab_id.clone(),
                            bytes: data.to_vec(),
                        });
                    }
                    Some(ChannelMsg::ExitStatus { exit_status: _ }) | Some(ChannelMsg::Eof) => {
                    }
                    Some(ChannelMsg::Close) => {

                        break;
                    }
                    None => {

                        break;
                    }
                    _ => {}
                    }
                }
            }
        }
    });
}
struct ClientHandler;

impl russh::client::Handler for ClientHandler {
    type Error = anyhow::Error;

    async fn check_server_key(
        &mut self,
        server_public_key: &ssh_key::PublicKey,
    ) -> Result<bool, Self::Error> {
        println!("Server key: {:?}", server_public_key);

        Ok(true)
    }
}
