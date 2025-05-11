#[cfg(feature = "desktop")]
pub fn desktop_config() -> dioxus::desktop::Config {
    use dioxus::desktop::{LogicalSize, WindowBuilder, tao::window::Icon};
    use image::imageops::{FilterType, resize};

    // 窗口大小
    let window_width = 1260;
    let window_height = 720;

    // 生成窗口图标
    let icon = image::open("./assets/logo.png").unwrap().to_rgba8();
    let window_icon_width = 32;
    let window_icon_height = 32;
    let window_icon = resize(
        &icon,
        window_icon_width,
        window_icon_height,
        FilterType::Lanczos3,
    );
    let window_icon = Icon::from_rgba(
        window_icon.into_raw(),
        window_icon_width,
        window_icon_height,
    )
    .unwrap();

    // 设置窗口图标, 大小, 位置, 标题
    let window = WindowBuilder::new()
        .with_window_icon(Some(window_icon))
        .with_inner_size(LogicalSize::new(window_width, window_height))
        .with_title("Smarter Buaa");

    // 对于 Windows 额外设置任务栏图标
    #[cfg(target_os = "windows")]
    let window = {
        use dioxus::desktop::tao::platform::windows::WindowBuilderExtWindows;
        let (width, height) = icon.dimensions();
        let taskbar_icon = Icon::from_rgba(icon.into_raw(), width, height).unwrap();
        window.with_taskbar_icon(Some(taskbar_icon))
    };

    dioxus::desktop::Config::new().with_window(window)
}
