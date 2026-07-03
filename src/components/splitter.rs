use gpui::{prelude::FluentBuilder, *};
// use gpui_signals::{Memo, Signal, effect};

// // ============== 事件定义 ==============
// #[derive(Clone, Debug)]
// pub enum SplitterEvent {
//     DragStart,
//     Drag(f32),
//     DragEnd,
// }

// // ============== Splitter 实体 ==============
// #[derive(Clone)]
// pub struct Splitter {
//     // 响应式状态
//     dragging: Signal<bool>,
//     last_x: Signal<f32>,
//     // 事件回调
//     on_event: Option<Shared<dyn Fn(SplitterEvent, &mut Window, &mut App)>>,
// }

// impl Splitter {
//     pub fn new() -> Self {
//         Self {
//             dragging: Signal::new(false),
//             last_x: Signal::new(0.0),
//             on_event: None,
//         }
//     }

//     pub fn on_event(
//         mut self,
//         handler: impl Fn(SplitterEvent, &mut Window, &mut App) + 'static,
//     ) -> Self {
//         self.on_event = Some(Shared::new(handler));
//         self
//     }

//     fn emit(&self, event: SplitterEvent, window: &mut Window, app: &mut App) {
//         if let Some(cb) = &self.on_event {
//             cb(event, window, app);
//         }
//     }

//     // 获取当前拖拽状态
//     pub fn is_dragging(&self) -> bool {
//         *self.dragging.get()
//     }

//     // 获取当前 X 位置
//     pub fn last_x(&self) -> f32 {
//         *self.last_x.get()
//     }
// }

// // ============== Render 实现 ==============
// impl Render for Splitter {
//     fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
//         // 获取实体引用用于闭包
//         let this = cx.this();
//         let on_event = self.on_event.clone();

//         // 创建响应式依赖
//         let dragging_signal = self.dragging.clone();
//         let last_x_signal = self.last_x.clone();

//         div()
//             .id("splitter")
//             .w(px(4.0))
//             .h_full()
//             // 响应式样式：拖拽时高亮
//             .when(*dragging_signal.get(), |style| {
//                 style.bg(rgb(0x4a9eff)) // 拖拽时蓝色
//             })
//             .when(!*dragging_signal.get(), |style| {
//                 style.bg(rgb(0x2a2a2a)) // 空闲时深色
//             })
//             .hover(|style| {
//                 style.bg(rgb(0x3a3a3a)) // 悬停时灰色
//             })
//             .cursor(CursorStyle::ResizeColumn)
//             // ===== MouseDown =====
//             .on_mouse_down(
//                 MouseButton::Left,
//                 move |event: &MouseDownEvent, window: &mut Window, app: &mut App| {
//                     // 更新状态
//                     dragging_signal.set(true);
//                     last_x_signal.set(event.position.x.as_f32());

//                     // 发送事件
//                     if let Some(cb) = &on_event {
//                         cb(SplitterEvent::DragStart, window, app);
//                     }

//                     // 触发重新渲染
//                     this.notify();
//                 },
//             )
//             // ===== MouseMove =====
//             .on_mouse_move(
//                 move |event: &MouseMoveEvent, window: &mut Window, app: &mut App| {
//                     // 检查是否在拖拽
//                     if !*dragging_signal.get() {
//                         return;
//                     }

//                     let current_x = event.position.x.as_f32();
//                     let last_x = *last_x_signal.get();
//                     let dx = current_x - last_x;

//                     // 更新位置
//                     last_x_signal.set(current_x);

//                     // 发送拖拽事件
//                     if let Some(cb) = &on_event {
//                         cb(SplitterEvent::Drag(dx), window, app);
//                     }

//                     // 触发重新渲染
//                     this.notify();
//                 },
//             )
//             // ===== MouseUp =====
//             .on_mouse_up(
//                 MouseButton::Left,
//                 move |_: &MouseUpEvent, window: &mut Window, app: &mut App| {
//                     // 重置状态
//                     dragging_signal.set(false);

//                     // 发送事件
//                     if let Some(cb) = &on_event {
//                         cb(SplitterEvent::DragEnd, window, app);
//                     }

//                     // 触发重新渲染
//                     this.notify();
//                 },
//             )
//             // ===== MouseUpOut (防止拖出窗口卡住) =====
//             .on_mouse_up_out(
//                 MouseButton::Left,
//                 move |_: &MouseUpEvent, window: &mut Window, app: &mut App| {
//                     // 重置状态
//                     dragging_signal.set(false);

//                     // 发送事件
//                     if let Some(cb) = &on_event {
//                         cb(SplitterEvent::DragEnd, window, app);
//                     }

//                     // 触发重新渲染
//                     this.notify();
//                 },
//             )
//     }
// }
