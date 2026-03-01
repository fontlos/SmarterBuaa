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
    // 账号列表
    let mut accounts = use_signal(Vec::<String>::new);
    // 活跃账户
    let mut account = use_signal(String::new);

    let mut show_list = use_signal(|| false);
    let account_list = accounts.iter().map(|a| {
        rsx! {
            div {
                class: "account-item",
                onclick: {
                    let a = a.clone();
                    move |_| { account.set(a.clone()); show_list.set(false); }
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
                            value: "{account}",
                            oninput: move |e| {
                                account.set(e.value());
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
                            if !account.is_empty() && !accounts().contains(&account()) {
                                accounts.push(account());
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
