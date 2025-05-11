#[inline]
pub fn launch() {
    use dioxus::LaunchBuilder;
    #[cfg(feature = "desktop")]
    let launcher = LaunchBuilder::desktop().with_cfg(crate::config::desktop_config());
    #[cfg(feature = "mobile")]
    let launcher = LaunchBuilder::mobile();
    launcher.launch(crate::ui::App);
}
