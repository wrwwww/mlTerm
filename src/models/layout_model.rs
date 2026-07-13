pub struct LayoutModel {
    pub sidebar_width: f32,
    pub sidebar_collapsed: bool,
    pub dragging_splitter: bool,
    pub splitter_position: f32,
    pub bottom_panel_height: f32,
    pub dragging_bottom_splitter: bool,

    pub left_panel_visible: bool,
    pub right_panel_visible: bool,
}

impl Default for LayoutModel {
    fn default() -> Self {
        Self {
            sidebar_width: 240.,
            sidebar_collapsed: Default::default(),
            splitter_position: 0.0,
            dragging_splitter: Default::default(),
            bottom_panel_height: Default::default(),
            dragging_bottom_splitter: Default::default(),
            left_panel_visible: Default::default(),
            right_panel_visible: Default::default(),
        }
    }
}
