use gpui::{prelude::FluentBuilder, *};

use crate::models::layout_model::LayoutModel;

#[derive(Clone, Debug)]
pub struct Splitter {
    pub layout_model: Entity<LayoutModel>,
    pub hovered: Entity<bool>,
}

#[derive(Clone, Debug)]
pub struct SplitterDragHandle;

impl Splitter {
    pub fn new(cx: &mut Context<Self>, layout_model: Entity<LayoutModel>) -> Self {
        Self {
            layout_model,
            hovered: cx.new(|_| false),
        }
    }
}

impl Render for Splitter {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // if self.layout_model.read(cx).dragging_splitter && !window.on_mouse_event(MouseButton::Left)
        // {
        //     dragging = false;
        // }
        let active = *self.hovered.read(cx) || self.layout_model.read(cx).dragging_splitter;

        div()
            .on
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
                    .bg(rgba(0x00000000))
                    .cursor_col_resize()
                    .block_mouse_except_scroll()
                    .when(active, |style| style.bg(rgb(0x60a5fa)))
                    // .when(!active, |style| style.bg(rgba(0x00000000)))
                    .on_hover({
                        // let hovered = hovered.clone();
                        let state = self.hovered.clone();
                        move |&is_hovered, _, cx| {
                            state.update(cx, |t, cx| {
                                *t = is_hovered;
                                cx.notify();
                            })
                            // hovered.write(cx, is_hovered);
                        }
                    })
                    .on_drag(SplitterDragHandle, {
                        let state = self.layout_model.clone();
                        move |_, _, _, cx| {
                            state.update(cx, |state, cx| {
                                state.dragging_splitter = true;
                                cx.notify();
                            });
                            cx.new(|_| Empty)
                        }
                    }),
            )
    }
}
