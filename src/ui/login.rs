use dioxus::prelude::*;

#[component]
pub fn LoginPage() -> Element {
    let mut state = use_signal(|| "");
    rsx!{
        link {
            rel: "stylesheet",
            href: asset!("./assets/css/login-page.css"),
        }
        div {
            class: "container {state}",
            div {
                class: "form-box account",
                form {
                    h1{ "Account" }
                    div {
                        class: "input-box",
                        input {
                            r#type: "text",
                            placeholder: "Account",
                            required: "required",
                        }
                    }
                    button {
                        class: "btn",
                        "Add"
                    }
                }
            }

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
                    p { "Switch account" }
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
