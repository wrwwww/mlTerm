use gpui::*;

use anyhow::Result;
use gpui_component::Root;
use log::{debug, info};
use std::sync::Arc;

use crate::{
    app::config::ConfigManager,
    models::{SshModels, layout_model::LayoutModel},
    terminal::session_manager::SessionViewManager,
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
    layout_model: Entity<LayoutModel>,
}
pub struct AppState {
    pub config_manager: ConfigManager,
}

impl AppState {
    pub fn new(cx: &mut Context<'_, AppState>, config_manager: ConfigManager) -> Self {
        Self { config_manager }
    }
}
impl AppRoot {
    pub fn new(window: &mut Window, cx: &mut App, config_manager: ConfigManager) -> Self {
        let initial_config = config_manager.get_config();
        let config = initial_config.clone();
        let state = cx.new(|cx| AppState::new(cx, config_manager));
        let layout_model = cx.new(|cx| LayoutModel::default());
        let session_view_manager = cx.new(|cx| SessionViewManager::new());
        Self {
            sidebar: cx.new(|cx| Sidebar::new(window, cx, session_view_manager.clone())),
            tabs: cx.new(|cx| TabBar::new(cx)),
            terminal: cx.new(|cx| {
                TerminalView::new(window, cx, state.clone(), session_view_manager.clone())
            }),
            appbar: cx.new(|cx| AppBar::new(cx)),
            ssh_model: cx.new(|cx| SshModels::new(cx)),
            state,
            menus: cx.new(|cx| AppMenuBar::new(cx)),
            splitter: cx.new(|cx| Splitter::new(cx, layout_model.clone())),
            layout_model,
            // config: config,
        }
    }
}

impl Render for AppRoot {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let layout = self.layout_model.read(cx);

        let sidebar_width = if layout.sidebar_collapsed {
            0.
        } else {
            layout.sidebar_width
        };
        let dialog_layer = Root::render_dialog_layer(window, cx);

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
                    .on_drag_move::<SplitterDragHandle>({
                        let state = self.layout_model.clone();
                        move |event, _, cx| {
                            state.update(cx, |state, cx| {
                                state.sidebar_width = (event.event.position.x.as_f32()
                                    - event.bounds.left().as_f32())
                                .clamp(180., 560.);

                                cx.notify();
                            });
                        }
                    })
                    .on_drop::<SplitterDragHandle>({
                        let state = self.layout_model.clone();

                        move |_, _, cx| {
                            state.update(cx, |state, cx| {
                                state.dragging_splitter = false;
                                cx.notify();
                            });

                            cx.new(|_| Empty);
                        }
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
