use gpui::*;

use gpui_component::Root;

use crate::{
    models::{SshModels, layout_model::LayoutModel},
    state::{
        app_state::AppState,
        config_manager::ConfigManager,
        session_manager::{JsonSessionStorage, SessionManager},
        terminal_manager::TerminalManager,
    },
    ui::{
        components::splitter::{Splitter, SplitterDragHandle},
        widgets::TabBar,
        window::{
            side_bar::Sidebar, status_bar::AppBar, terminal_area::TerminalArea,
            title_bar::AppMenuBar,
        },
    },
};

pub struct AppRoot {
    sidebar: Entity<Sidebar>,
    tabs: Entity<TabBar>,
    terminal: Entity<TerminalArea>,
    appbar: Entity<AppBar>,
    menus: Entity<AppMenuBar>,
    splitter: Entity<Splitter>,
    // config: Arc<AppConfig>,
    state: Entity<AppState>,
    ssh_model: Entity<SshModels>,
    layout_model: Entity<LayoutModel>,
}

impl AppRoot {
    pub fn new(window: &mut Window, cx: &mut App, config_manager: ConfigManager) -> Self {
        let initial_config = config_manager.get_config();
        let config = initial_config.clone();
        let state = cx.new(|cx| AppState::new(cx, config_manager));
        let layout_model = cx.new(|cx| LayoutModel::default());
        let storage = JsonSessionStorage::new("./config/sessions.json");
        let session_manager = cx.new(|cx| SessionManager::new(storage));
        session_manager.update(cx, |this, cx| {
            this.load().expect("session 加载失败");
        });
        let terminal_manager = cx.new(|cx| TerminalManager::new(session_manager, cx));
        Self {
            sidebar: cx.new(|cx| Sidebar::new(window, cx, terminal_manager.clone())),
            tabs: cx.new(|cx| TabBar::new(cx)),
            terminal: cx
                .new(|cx| TerminalArea::new(window, cx, state.clone(), terminal_manager.clone())),
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
            // .child(div().h_10().w_full().child(self.menus.clone()))
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
