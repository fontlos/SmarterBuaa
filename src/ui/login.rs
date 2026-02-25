use dioxus::prelude::*;

#[component]
pub fn LoginPage() -> Element {
    // 挂载元素后再启用动画避免抖动, 此类在 base.css
    let mut load = use_signal(|| "no-transition");
    use_effect(move || {
        load.set("");
    });
    // 页面状态
    let mut state = use_signal(|| "");
    // 活跃账户
    let mut account = use_signal(|| String::new());
    rsx! {
        link {
            rel: "stylesheet",
            href: asset!("/assets/css/login-page.css"),
        }
        div {
            class: "container {state} {load}",
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
                            required: "required",
                            oninput: move |e| {
                                account.set(e.value());
                            },
                        }
                    }
                    button {
                        class: "btn",
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
                    p { "User {account}" }
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
