use gpui::*;

use anyhow::Result;
use gpui_component::Root;
use log::{debug, info};
use std::sync::Arc;

use crate::{
    app::config::{AppConfig, ConfigManager},
    models::SshModels,
    ui::{
        components::splitter::{Splitter, SplitterDragHandle},
        terminal::terminal_view::TerminalView,
        widgets::TabBar,
        window::{side_bar::Sidebar, status_bar::AppBar, title_bar::AppMenuBar},
    },
};

pub struct LayoutState {
    pub sidebar_width: f32,
    pub splitter_position: f32,
}

pub struct SidebarState {
    pub collapsed: bool,
    pub active_session: Option<String>,
}

pub struct AppRoot {
    sidebar: Entity<Sidebar>,
    tabs: Entity<TabBar>,
    terminal: Entity<TerminalView>,
    appbar: Entity<AppBar>,
    menus: Entity<AppMenuBar>,
    splitter: Entity<Splitter>,
    // config: Arc<AppConfig>,
    state: Entity<AppState>,
    ssh_model: Entity<SshModels>,
}
pub struct AppState {
    pub layout: LayoutState,
    pub sidebar: SidebarState,
    pub config_manager: ConfigManager,
}

impl AppState {
    pub fn new(cx: &mut Context<'_, AppState>, config_manager: ConfigManager) -> Self {
        Self {
            config_manager,
            layout: LayoutState {
                sidebar_width: 240.0,
                splitter_position: 0.0,
            },
            sidebar: SidebarState {
                collapsed: false,
                active_session: None,
            },
        }
    }
}
impl AppRoot {
    pub fn new(window: &mut Window, cx: &mut App, config_manager: ConfigManager) -> Self {
        let initial_config = config_manager.get_config();
        let config = initial_config.clone();
        let state = cx.new(|cx| AppState::new(cx, config_manager));
        Self {
            sidebar: cx.new(|cx| Sidebar::new(window, cx)),
            tabs: cx.new(|cx| TabBar::new(cx)),
            terminal: cx.new(|cx| TerminalView::new(window, cx, state.clone())),
            appbar: cx.new(|cx| AppBar::new(cx)),
            ssh_model: cx.new(|cx| SshModels::new(cx)),
            state,
            menus: cx.new(|cx| AppMenuBar::new(cx)),
            splitter: cx.new(|cx| Splitter::new()),
            // config: config,
        }
    }
}

impl Render for AppRoot {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let dialog_layer = Root::render_dialog_layer(window, cx);
        let sidebar_width = self.state.read(cx).layout.sidebar_width;
        let drag_state = self.state.clone();
        let drop_state = self.state.clone();

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
                    .on_drag_move::<SplitterDragHandle>(move |event, _window, app| {
                        let new_width = (event.event.position.x.as_f32()
                            - event.bounds.left().as_f32())
                        .clamp(180.0, 560.0);

                        drag_state.update(app, |app_state, cx| {
                            app_state.layout.sidebar_width = new_width;
                            cx.notify();
                        });
                    })
                    .on_drop::<SplitterDragHandle>(move |_, _window, app| {
                        drop_state.update(app, |app_state, cx| {
                            app_state.layout.splitter_position = app_state.layout.sidebar_width;
                            cx.notify();
                        });
                    })
                    .child(
                        div()
                            .w(px(sidebar_width))
                            .h_full()
                            .flex_shrink_0()
                            .child(self.sidebar.clone()),
                    )
                    .child(self.splitter.clone())
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .min_w_0()
                            .child(div().h(px(36.0)))
                            .child(div().child(self.tabs.clone()))
                            .child(div().flex_1().child(div().child(self.terminal.clone()))),
                    ),
            )
            .children(dialog_layer)
            .flex_auto()
    }
}
