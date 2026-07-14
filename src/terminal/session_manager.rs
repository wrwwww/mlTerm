use gpui::SharedString;

// 光标位置
// 滚动缓冲区
// ANSI 状态
// 当前颜色
// 选区
// 搜索状态
// 鼠标状态
// SSH 只是 Session 的一个数据来源。
pub struct SessionView {
    // 视图名称
    pub label: SharedString,
}

pub struct SessionViewManager {
    pub(crate) sessions: Vec<Box<SessionView>>,
    selected_index: Option<usize>,
}
impl SessionViewManager {
    pub fn new() -> Self {
        Self {
            sessions: Vec::with_capacity(10),
            selected_index: None,
        }
    }
    // 新增一个会话窗口
    pub fn add(&mut self, label: SharedString) {
        self.sessions.push(Box::new(SessionView { label }));
        self.selected_index = Some(self.sessions.len() - 1);
    }
    pub fn close_all() {}
    pub fn close(&mut self, idx: usize) {
        let len = self.sessions.len();
        if idx < 0 && idx >= len {
            log::error!("删除失败session view 失败，对应的索引不存在！！！");
            return;
        }
        if idx == len - 1 {
            self.selected_index = Some(len - 2);
        }
        self.sessions.remove(idx);
    }
}
