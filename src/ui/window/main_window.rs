use gpui::*;

use crate::gui::main_window::MainWindow;
use crate::infrastructure::event_loop::EventLoop;
use crate::models::session_config::SessionConfig;
use crate::terminal::session_manager::SessionManager;
use crate::{
    appbar::AppBar, components::splitter, menus::AppMenuBar, sidebar::Sidebar, tabbar::TabBar,
    terminal::TerminalView,
};
use anyhow::Result;
use log::{debug, info};
use std::sync::Arc;
pub struct AppState {
    pub layout: LayoutState,
    pub sidebar: SidebarState,
    pub splitter: SplitterState,
    // pub tabs: TabsState,
    // pub terminal: TerminalState,
}
pub struct LayoutState {
    pub sidebar_width: f32,
    pub splitter_position: f32,
}
pub struct SidebarState {
    pub collapsed: bool,
    pub active_session: Option<String>,
}
pub struct SplitterState {
    pub dragging: bool,
    pub hover: bool,
}
// pub struct TabsState {
//     pub active: usize,
//     pub tabs: Vec<Tab>,
// }
// pub struct TerminalState {
//     pub buffers: Vec<Buffer>,
//     pub cursor: Cursor,
// }
pub struct AppRoot {
    sidebar: Entity<Sidebar>,
    tabs: Entity<TabBar>,
    terminal: Entity<TerminalView>,
    appbar: Entity<AppBar>,
    state: Entity<AppState>,
    menus: Entity<AppMenuBar>,
}
impl AppRoot {
    pub fn new(window: &Window, cx: &mut App) -> Self {
        Self {
            sidebar: cx.new(|cx| Sidebar::new(cx)),
            tabs: cx.new(|cx| TabBar::new(cx)),
            terminal: cx.new(|cx| TerminalView::new(cx)),
            appbar: cx.new(|cx| AppBar::new(cx)),
            state: cx.new(|cx| AppState {
                layout: LayoutState {
                    sidebar_width: 240.0,
                    splitter_position: 0.0,
                },
                sidebar: SidebarState {
                    collapsed: false,
                    active_session: None,
                },
                splitter: SplitterState {
                    dragging: false,
                    hover: false,
                },
                // tabs: TabsState {
                //     active: 0,
                //     tabs: Vec::new(),
                // },
                // terminal: TerminalState {
                //     buffers: Vec::new(),
                //     cursor: Cursor::default(),
                // },
            }),
            menus: cx.new(|cx| AppMenuBar::new(cx)),
        }
    }
}
impl Render for AppRoot {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .child(self.appbar.clone())
            .child(div().h_10().w_full().child(self.menus.clone()))
            .child(
                div()
                    .w_full()
                    .h_full()
                    .flex()
                    .flex_row()
                    .child(
                        // 左侧 session tree
                        div().w(px(240.0)).h_full().child(self.sidebar.clone()),
                    )
                    // .child()
                    .child(
                        // 右侧 main
                        div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .child(div().h(px(36.0)))
                            .child(div().child(self.tabs.clone()))
                            .child(div().flex_1().child(div().child(self.terminal.clone()))),
                    ),
            )
            .flex_auto()
    }
}
