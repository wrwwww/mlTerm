use gpui::{prelude::FluentBuilder, *};

#[derive(Clone, Debug)]
pub struct Splitter;

#[derive(Clone, Debug)]
pub struct SplitterDragHandle;

impl Splitter {
    pub fn new() -> Self {
        Self
    }
}

impl Render for Splitter {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let hovered = window.use_state(cx, |_window, _cx| false);
        let dragging = window.use_state(cx, |_window, _cx| false);
        let active = *hovered.read(cx) || *dragging.read(cx);

        div()
            .id("splitter")
            .relative()
            .w(px(1.0))
            .mx_1()
            .h_full()
            .bg(rgb(0x2a2a2a))
            .child(
                div()
                    .id("splitter-handle")
                    .absolute()
                    .left_neg_0p5()
                    .top_0()
                    .bottom_0()
                    .w_1()
                    .cursor_col_resize()
                    .block_mouse_except_scroll()
                    .when(active, |style| style.bg(rgb(0x60a5fa)))
                    .when(!active, |style| style.bg(rgba(0x00000000)))
                    .on_hover({
                        let hovered = hovered.clone();
                        move |&is_hovered, _, cx| {
                            hovered.write(cx, is_hovered);
                        }
                    })
                    .on_drag(SplitterDragHandle, {
                        let dragging = dragging.clone();
                        move |_, _, _, cx| {
                            dragging.write(cx, true);
                            cx.new(|_| Empty)
                        }
                    })
                    .on_drop::<SplitterDragHandle>({
                        let dragging = dragging.clone();
                        move |_, _, cx| {
                            dragging.write(cx, false);
                        }
                    }),
            )
    }
}
