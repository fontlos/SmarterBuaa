use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    #[cfg(feature = "desktop")]
    crate::config::tray_config();
    rsx! {
        "Hello World"
    }
}
