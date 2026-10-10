use crate::route::Route;
use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
    let nav = navigator();
    rsx! {
        div { class: "container", id: "login_main_container",
            h1 { class: "login_text", id: "login_header_text", "LOGIN" }
            p { class: "login_text", id: "login_text1", "Welcome back." }

            p { class: "login_text", id: "login_text2", "Username." }
            input {
                class: "login_input",
                id: "login_username_field",
                placeholder: "Insert username here.",
            }

            p { class: "login_text", id: "login_text3", "Password." }
            input {
                class: "login_input",
                r#type: "password",
                id: "login_password_field",
                placeholder: "Insert password here.",
            }

            button {
                class: "button",
                id: "login_btn",
                onclick: move |_| {
                    nav.push(Route::Home {});
                },
                "LOGIN"
            }

            p { class: "login_text", id: "login_text4", "Don't have an account?" }
            button {
                class: "button",
                id: "signin_btn",
                onclick: move |_| {
                    nav.push(Route::Signin {});
                },
                "SIGN IN"
            }
        }
    }
}