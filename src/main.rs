use gpui::*;
use gpui_component::*;
use my_gpui_app::app::AppRoot;
mod app;
mod ui;
fn main() {
    env_logger::init();
    let app = gpui_platform::application().with_assets(gpui_component_assets::Assets);

    app.run(move |cx| {
        gpui_component::init(cx);
        let bounds = Bounds::centered(None, size(px(1000.), px(600.0)), cx);

        cx.spawn(async move |cx| {
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(TitleBar::title_bar_options()),
                    ..Default::default()
                },
                |window, cx| {
                    let view = cx.new(|cx| AppRoot::new(window, cx));
                    cx.new(|cx| Root::new(view, window, cx))
                },
            )
            .expect("Failed to open window");
        })
        .detach();
    });
}

fn main() -> anyhow::Result<()> {
    // 1. 初始化基础设施（日志最先）
    initialize()?;

    info!("Starting MyTerminal...");

    // 2. 创建 Application（程序大脑）
    let mut app = Application::new()?;

    // 3. 初始化资源（配置、主题、字体）
    app.init_resources()?;

    // 4. 创建主窗口
    app.create_main_window()?;

    // 5. 运行事件循环
    app.run()
}
