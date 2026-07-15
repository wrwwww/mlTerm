use crate::terminal::session::{Session, SessionId};
use anyhow::Result;
use std::result::Result::Ok;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

pub struct SessionManager<R>
where
    R: SessionRepository,
{
    pub sessions: HashMap<SessionId, Session>,
    pub selected_index: Option<usize>,
    repository: R,
}

impl<R> SessionManager<R>
where
    R: SessionRepository,
{
    pub fn new(repository: R) -> Self {
        Self {
            repository,
            sessions: HashMap::with_capacity(16),
            selected_index: None,
        }
    }

    pub fn insert(&mut self, session: Session) -> SessionId {
        let session_id = session.id;

        self.sessions.insert(session_id.clone(), session);
        self.save().expect("insert session 失败");
        session_id
    }
    pub fn load(&mut self) -> Result<()> {
        let list = self.repository.load()?;

        self.sessions.clear();

        for session in list {
            self.sessions.insert(session.id, session);
        }

        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        let sessions: Vec<Session> = self.sessions.values().cloned().collect();

        self.repository.save(&sessions)
    }
}

pub trait SessionRepository {
    fn load(&self) -> Result<Vec<Session>>;

    fn save(&self, sessions: &[Session]) -> Result<()>;
}
pub struct JsonSessionStorage {
    path: PathBuf,
}

impl JsonSessionStorage {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }
}

impl SessionRepository for JsonSessionStorage {
    fn load(&self) -> Result<Vec<Session>> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }

        let text = fs::read_to_string(&self.path)?;

        if let Ok(sessions) = serde_json::from_str::<Vec<Session>>(&text) {
            return Ok(sessions);
        }
        Ok(vec![])
    }

    fn save(&self, sessions: &[Session]) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(sessions)?;

        fs::write(&self.path, json)?;

        Ok(())
    }
}
