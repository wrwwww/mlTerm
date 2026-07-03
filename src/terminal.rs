use gpui::*;
pub struct TerminalView;
impl TerminalView {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self
    }
}
impl Render for TerminalView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().child("Terminal will be here...")
    }
}
