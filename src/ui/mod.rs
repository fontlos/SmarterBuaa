mod login;

use dioxus::prelude::*;

#[derive(Routable, PartialEq, Clone)]
enum Route {
    #[route("/", login::LoginPage)]
    Login {},
}

#[component]
pub fn App() -> Element {
    #[cfg(feature = "desktop")]
    crate::config::desktop::tray();

    const FONT: Asset = asset!(
        "/assets/font/Poppins-Regular.woff2",
        AssetOptions::builder().with_hash_suffix(false)
    );
    rsx! {
        document::Link { rel: "preload", href: FONT, as: "font", type: "font/woff2"}
        document::Link { rel: "stylesheet", href: asset!("/assets/css/base.css") }
        Router::<Route> {}
    }
}
