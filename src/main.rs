#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod config;
mod ui;

fn main() {
    use dioxus::LaunchBuilder;
    #[cfg(feature = "desktop")]
    let launcher = LaunchBuilder::desktop().with_cfg(crate::config::desktop::config());
    #[cfg(feature = "mobile")]
    let launcher = LaunchBuilder::mobile();
    launcher.launch(crate::ui::App);
}
