use crate::components::*;
use crate::route::Route;
use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
    let nav = navigator();
    let mut username = use_signal(String::new);
    let mut password = use_signal(String::new);
    rsx! {
        div { class: "container", id: "login_main_container",
            h1 { class: "login_text", id: "login_header_text", "LOGIN" }
            p { class: "login_text", id: "login_text1", "Welcome back." }

            FormField {
                label_id: "login_text2",
                label: "Username",
                input_id: "login_username_field",
                placeholder: "Insert username here.",
                on_input: move |text: String| username.set(text),
            }
            FormField {
                label_id: "login_text3",
                label: "Password",
                input_id: "login_password_field",
                input_type: "password",
                placeholder: "Insert password here.",
                on_input: move |text: String| password.set(text),
            }

            //Needs validation first.
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
