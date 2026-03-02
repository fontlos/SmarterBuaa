mod login;
mod schedule;

use buaa_api::Context;
use dioxus::prelude::*;

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[route("/", login::LoginPage)]
    Login {},
    #[route("/schedule", schedule::SchedulePage)]
    Schedule {},
}

#[component]
pub fn App() -> Element {
    #[cfg(feature = "desktop")]
    crate::config::desktop::tray();

    const FONT: Asset = asset!(
        "/assets/font/Poppins-Regular.woff2",
        AssetOptions::builder().with_hash_suffix(false)
    );

    use_context_provider(|| std::rc::Rc::new(Context::new()));

    rsx! {
        document::Link { rel: "preload", href: FONT, as: "font", type: "font/woff2"}
        document::Link { rel: "stylesheet", href: asset!("/assets/css/base.css") }
        Router::<Route> {}
    }
}
