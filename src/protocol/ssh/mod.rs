use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::Result;
use anyhow::bail;
use ssh2::{Channel, Session};
use tokio::sync::{Mutex, RwLock};

use crate::protocol::connection::{SessionState, TerminalSession};

mod client;
pub struct SshSession {
    // session: Session,
    channel: Channel,
    inner: Arc<Mutex<Option<Session>>>,
    config: SshConfig,
    state: Arc<RwLock<SessionState>>,
    last_activity: Arc<Mutex<Instant>>,
}
// SSH 连接配置
#[derive(Debug, Clone)]
pub struct SshConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub auth: AuthMethod,
    pub timeout: Duration,
    pub keepalive_interval: Duration,
    pub max_retries: u32,
    pub retry_delay: Duration,
}

#[derive(Debug, Clone)]
pub enum AuthMethod {
    Password {
        password: String,
    },
    PublicKey {
        pub_key_path: String,
        priv_key_path: String,
        passphrase: Option<String>,
    },
    Agent, // 使用 ssh-agent
}
impl SshSession {
    // 建立连接（带重试）
    pub async fn connect(&self) -> anyhow::Result<()> {
        let mut retries = 0;
        let mut delay = self.config.retry_delay;

        while retries < self.config.max_retries {
            match self.try_connect().await {
                Ok(session) => {
                    *self.inner.lock().await = Some(session);
                    *self.state.write().await = SessionState::Connected;
                    // self.update_activity().await;
                    return Ok(());
                }
                Err(e) => {
                    retries += 1;
                    *self.state.write().await = SessionState::Error(e.to_string());

                    if retries >= self.config.max_retries {
                        bail!("连接失败")
                    }

                    // 指数退避
                    tokio::time::sleep(delay).await;
                    delay = delay * 2;
                }
            }
        }
        Ok(())
    }

    async fn try_connect(&self) -> anyhow::Result<Session> {
        let addr = format!("{}:{}", self.config.host, self.config.port);
        let stream =
            tokio::time::timeout(self.config.timeout, tokio::net::TcpStream::connect(addr))
                .await??;

        // let mut session = AsyncSession::new(stream, None)?;
        let mut session = Session::new()?;
        session.set_tcp_stream(stream);
        session.handshake()?;
        // 握手
        // tokios::time::timeout(self.config.timeout, session.handshake()).await??;

        // 认证
        self.authenticate(&mut session).await?;

        Ok(session)
    }

    async fn authenticate(&self, session: &mut Session) -> Result<()> {
        match &self.config.auth {
            AuthMethod::Password { password } => {
                session.userauth_password(&self.config.username, password)?;
            }
            AuthMethod::PublicKey {
                pub_key_path,
                priv_key_path,
                passphrase,
            } => {
                session.userauth_pubkey_file(
                    &self.config.username,
                    Some(std::path::Path::new(pub_key_path)),
                    std::path::Path::new(priv_key_path),
                    passphrase.as_deref(),
                )?;
            }
            AuthMethod::Agent => {
                // 使用 ssh-agent
                #[cfg(unix)]
                {
                    use async_ssh2::userauth::Agent;
                    let mut agent = Agent::new()?;
                    agent.connect()?;
                    session.userauth_agent(&self.config.username, agent).await?;
                }
                #[cfg(not(unix))]
                bail!("不支持的平台");
            }
        }
        Ok(())
    }
}
// impl SshSession {
//     pub fn new(addr: &str, port: u16) -> Self {
//         // Implementation for creating SSH session
//         let tcp = TcpStream::connect("192.168.1.100:22").unwrap();
//         let mut session = Session::new().unwrap();

//         session.set_tcp_stream(tcp);
//         Self {
//             session,
//             // channel: (),
//         }
//     }
// }
// impl TerminalSession for SshSession {
//     pub fn write(&mut self, data: &[u8]) -> std::io::Result<()> {
//         self.channel.write_all(data)?;
//         self.channel.flush()?;
//         Ok(())
//     }

//     pub fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
//         self.channel.read(buf)
//     }

//     pub fn resize(&mut self, cols: u32, rows: u32) -> Result<(), ssh2::Error> {
//         self.channel.request_pty_size(cols, rows, None, None)
//     }

//     pub fn close(&mut self) -> Result<(), ssh2::Error> {
//         self.channel.close()
//     }
// }
