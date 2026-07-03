use gpui::*;

pub struct Sidebar {
    sessions: Vec<i32>,
}
impl Sidebar {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self { sessions: vec![] }
    }
}
impl Render for Sidebar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().child("Sidebar")
    }
}
