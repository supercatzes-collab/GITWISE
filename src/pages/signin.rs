use crate::route::Route;
use dioxus::prelude::*;

#[component]
pub fn Signin() -> Element {
    let nav  = navigator();
    rsx! {
        div { class: "container", id: "sign_main_container",
            h1 { class: "login_text", id: "sign_title", "SIGN IN" }
            p { class: "login_text", id: "profile_text0",
                "Share and contribute to open-source research"
            }
            div { class: "container", id: "profile_container",
                img {
                    id: "sign_profile",
                    src: asset!("/assets/img_assets/profile.jpg"),
                }
                span { id: "profile_details",
                    h2 { id: "profile_title", "Profile picture" }
                    p { class: "login_text", id: "profile_text1", "Choose an image" }
                }
            }
            p { class: "login_text", id: "profile_text2", "Display name" }
            input {
                class: "login_input",
                id: "sign_username_field",
                placeholder: "Insert username here.",
            }
            p { class: "login_text", id: "profile_text3", "Password" }
            input {
                class: "login_input",
                r#type: "password",
                id: "sign_password_field",
                placeholder: "Insert password here.",
            }

            button {
                class: "button",
                id: "signin_btn",
                onclick: move |_| {
                    nav.push(Route::Home {});
                },
                "SIGN IN"
            }
        }
    }
}