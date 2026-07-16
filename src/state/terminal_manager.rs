use anyhow::{Ok, bail};
use gpui::{Context, Entity, SharedString};

use crate::{
    state::session_manager::{JsonSessionStorage, SessionManager},
    terminal::session::{Session, SessionId},
};

// 光标位置
// 滚动缓冲区
// ANSI 状态
// 当前颜色
// 选区
// 搜索状态
// 鼠标状态
// SSH 只是 Session 的一个数据来源。
pub struct TerminalTab {
    // 视图名称
    pub label: SharedString,
}

pub struct TerminalManager {
    pub session_manager: Entity<SessionManager<JsonSessionStorage>>,
    pub tabs: Vec<Box<TerminalTab>>,
    selected_index: Option<usize>,
}
impl TerminalManager {
    pub fn new(
        session_manager: Entity<SessionManager<JsonSessionStorage>>,
        cx: &mut Context<'_, TerminalManager>,
    ) -> Self {
        Self {
            session_manager,
            tabs: Vec::with_capacity(10),
            selected_index: None,
        }
    }
    pub fn new_session(&mut self, session: Session, cx: &mut gpui::App) {
        let session_id = self.session_manager.update(cx, |a, cx| a.insert(session));
        self.add(cx, session_id);
    }
    // 新增一个会话窗口
    pub fn add(&mut self, cx: &gpui::App, session_id: SessionId) -> anyhow::Result<Session> {
        let sessions = &self.session_manager.read(cx).sessions;
        if let Some(session) = sessions.get(&session_id) {
            self.tabs.push(Box::new(TerminalTab {
                label: SharedString::new(session.name.clone()),
            }));
            self.selected_index = Some(self.tabs.len() - 1);
            let value = (*session).clone();
            return Ok(value);
        }
        bail!("")
    }
    pub fn close_all() {}
    pub fn close(&mut self, idx: usize) {
        let len = self.tabs.len();
        if idx < 0 && idx >= len {
            log::error!("删除失败session view 失败，对应的索引不存在！！！");
            return;
        }
        if idx == len - 1 {
            self.selected_index = Some(len - 2);
        }
        self.tabs.remove(idx);
    }
}
