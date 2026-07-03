use gpui::*;
use gpui_component::{IconName, TitleBar, button::Button, menu::AppMenuBar};
pub struct AppBar;
impl AppBar {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self
    }
}
impl Render for AppBar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        TitleBar::new()
            .child(div().flex().items_center().gap_3().child("mlTerm"))
            .child(div().flex().items_center().child(AppMenuBar::new(cx)))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_end()
                    .gap_2()
                    .child(Button::new("notifications").icon(IconName::Bell)),
            )
    }
}
