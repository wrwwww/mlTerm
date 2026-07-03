use gpui::*;
use gpui_component::{tab::Tab, *};
pub struct TabBar {
    tabs: Vec<Tab>,
    active: usize,
}
impl TabBar {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            tabs: vec![],
            active: 0,
        }
    }
}
impl Render for TabBar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().child("TabBar")
    }
}
