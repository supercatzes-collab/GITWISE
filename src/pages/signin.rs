use crate::components::*;
use crate::route::Route;
use dioxus::prelude::*;

#[component]
pub fn Signin() -> Element {
    let nav = navigator();
    let mut profile_path = use_signal(String::new);
    let mut username = use_signal(String::new);
    let mut password = use_signal(String::new);
    rsx! {
        div { class: "container", id: "sign_main_container",
            h1 { class: "login_text", id: "sign_title", "SIGN IN" }
            p { class: "login_text", id: "profile_text0",
                "Share and contribute to open-source research"
            }
            div { class: "container", id: "profile_container",
                ImageInput {
                    img_id: "sign_profile",
                    input_id: "sign_profile_input",
                    img_src: asset!("/assets/img_assets/profile.jpg"),
                    on_select: move |path: String| profile_path.set(path),
                }
                span { id: "profile_details",
                    h2 { id: "profile_title", "Profile picture" }
                    p { class: "login_text", id: "profile_text1", "Choose an image" }
                }
            }

            FormField {
                label_id: "profile_text2",
                label: "Username",
                input_id: "sign_username_field",
                placeholder: "Insert username here.",
                on_input: move |text: String| username.set(text),
            }
            FormField {
                label_id: "profile_text3",
                label: "Password",
                input_id: "sign_password_field",
                input_type: "password",
                placeholder: "Insert password here.",
                on_input: move |text: String| password.set(text),
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
