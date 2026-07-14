use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SessionId(pub Uuid);

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub enum SessionKind {
    #[default] // 指定默认变体
    Ssh,
    Telnet,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionConfig {
    pub hostname: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub enum SessionStatus {
    #[default] // 指定默认变体
    Connected,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Session {
    pub id: SessionId,

    // 显示名称
    pub name: String,

    // 连接类型
    pub kind: SessionKind,

    // 连接配置
    pub config: SessionConfig,

    // 当前状态
    #[serde(skip)]
    pub status: SessionStatus,
}
impl Session {
    pub fn get_hostname(&self) -> String {
        self.config.hostname.clone()
    }
}
