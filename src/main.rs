#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod config;
mod ui;

use buaa_api::Context;
use dioxus::LaunchBuilder;

use std::sync::OnceLock;

// 全局单例, Context 在内部通过内部可变性, 原子操作, Arc 已经实现了线程安全
// 所以我们只需要一个全局的不可变引用即可, 不需要担心线程安全问题
static CTX: OnceLock<Context> = OnceLock::new();

pub fn ctx() -> &'static Context {
    CTX.get().unwrap()
}

fn main() {
    // 初始化全局 Context
    CTX.set(Context::new()).unwrap();

    #[cfg(feature = "desktop")]
    let launcher = LaunchBuilder::desktop().with_cfg(crate::config::desktop::config());
    #[cfg(feature = "mobile")]
    let launcher = LaunchBuilder::mobile();
    launcher.launch(crate::ui::App);
}
