use std::collections::HashMap;

use crate::terminal::session::{Session, SessionId};

pub struct SessionManager {
    pub sessions: HashMap<SessionId, Session>,
    pub selected_index: Option<usize>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::with_capacity(16),
            selected_index: None,
        }
    }

    pub fn insert(&mut self, session: Session) {
        self.sessions.insert(SessionId::default(), session);
    }
}
