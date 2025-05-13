mod login;

use dioxus::prelude::*;

#[derive(Routable, PartialEq, Clone)]
enum Route{
    #[route("/", login::LoginPage)]
    Login {}
}

#[component]
pub fn App() -> Element {
    #[cfg(feature = "desktop")]
    crate::config::tray_config();
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./assets/css/base.css") }
        Router::<Route> {}
    }
}
