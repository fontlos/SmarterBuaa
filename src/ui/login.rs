use buaa_api::Context;
use dioxus::prelude::*;

use super::Route;

#[component]
pub fn LoginPage() -> Element {
    let nav = navigator();
    let ctx = use_context::<std::rc::Rc<Context>>();
    // 挂载元素后再启用动画避免抖动, 此类在 base.css
    let mut load = use_signal(|| "no-transition");
    use_effect(move || {
        load.set("");
    });
    // 页面状态
    let mut state = use_signal(|| "");
    // 账号列表
    let mut accounts = use_signal(Vec::<String>::new);
    // 活跃账户
    let mut username = use_signal(String::new);
    let mut password = use_signal(String::new);

    let mut show_list = use_signal(|| false);
    let account_list = accounts.iter().map(|a| {
        rsx! {
            div {
                class: "account-item",
                onclick: {
                    let a = a.clone();
                    move |_| { username.set(a.clone()); show_list.set(false); }
                },
                "{a}"
            }
        }
    });
    rsx! {
        link {
            rel: "stylesheet",
            href: asset!("/assets/css/login-page.css"),
        }
        div {
            class: "login-container {state} {load}",
            div {
                class: "form-box account",
                // 选择或添加账户
                form {
                    h1{ "Account" }
                    div {
                        class: "input-box",
                        input {
                            r#type: "text",
                            placeholder: "Account",
                            value: "{username}",
                            oninput: move |e| {
                                username.set(e.value());
                            },
                        }
                        button {
                            class: "arrow-btn",
                            onclick: move |_| show_list.set(!show_list()),
                            img {
                                src: asset!("/assets/icon/drop.svg"),
                            }
                        }
                        {if show_list() && !accounts.is_empty() {
                            rsx! {
                                div { class: "account-dropdown",
                                    {account_list}
                                }
                            }
                        } else { rsx!() }}
                    }
                    button {
                        class: "btn",
                        onclick: move |_| {
                            if !username.is_empty() && !accounts().contains(&username()) {
                                accounts.push(username());
                            }
                        },
                        "Add"
                    }
                }
            }

            // 登陆账户
            div {
                class: "form-box login",
                form {
                    h1{ "Login" }
                    div {
                        class: "input-box",
                        input {
                            r#type: "password",
                            placeholder: "Password",
                            required: "required",
                            oninput: move |e| {
                                password.set(e.value());
                            },
                        }
                    }
                    div {
                        class: "forgot-link",
                        a {
                            href: "#",
                            "Forgot password?"
                        }
                    }
                    button {
                        class: "btn",
                        onclick: move |_| {
                            let ctx = ctx.clone();
                            ctx.set_account(&username(), &password());
                            spawn(async move {
                                if ctx.login().await.is_ok() {
                                    nav.push(Route::Schedule {});
                                } else {
                                    // 登录失败提示
                                }
                            });
                        },
                        "Login"
                    }
                }
            }

            // 切换面板
            div {
                class: "toggle-box",
                div {
                    class: "toggle-panel toggle-right",
                    h1 { "Hello, Welcome!" }
                    p { "Choose or add an account" }
                    button {
                        class: "btn login-btn",
                        onclick: move |_| {
                            state.set("to-login");
                        },
                        "Continue"
                    }
                }
                div {
                    class: "toggle-panel toggle-left",
                    h1 { "Welcome back!" }
                    p { "User {username}" }
                    button {
                        class: "btn account-btn",
                        onclick: move |_| {
                            state.set("");
                        },
                        "Switch"
                    }
                }
            }
        }
    }
}
