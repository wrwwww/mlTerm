use gpui::*;
use gpui_component::{
    button::{Button, ButtonVariants},
    dock::ClosePanel,
    menu::DropdownMenu,
};

pub struct AppMenuBar;
impl AppMenuBar {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {}
    }
}
impl Render for AppMenuBar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap_2()
            .child(
                Button::new("menu-btn1")
                    .text()
                    .p(px(2.0))
                    .label("session")
                    .dropdown_menu(|menu, window, cx| {
                        menu.menu("新建会话", Box::new(ClosePanel))
                            .menu("Option 2", Box::new(ClosePanel))
                    }),
            )
            .child(
                Button::new("menu-btn2")
                    .text()
                    .p(px(2.0))
                    .label("edit")
                    .dropdown_menu(|menu, window, cx| {
                        menu.menu("Option 1", Box::new(ClosePanel))
                            .menu("Option 2", Box::new(ClosePanel))
                    }),
            )
    }
}
