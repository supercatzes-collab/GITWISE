use crate::route::Route;
use dioxus::prelude::*;

#[component]
pub fn Settings() -> Element {
    let nav = navigator();
    rsx! {
        div { class: "container", id: "settings_main_container",
            div { class: "container", id: "settings_secondary_container",
                img {
                    id: "settings_profile_img",
                    src: asset!("/assets/img_assets/profile.jpg"),
                }
                div { class: "container", id: "settings_details_container",
                    h1 { id: "settings_username", "Alice Von Neumann" }
                    p { id: "settings_account_creation",
                        span { id: "settings_text", "Account created: " }
                        span { id: "settings_date", "09/10/26" }
                    }
                }
            }
            // ...existing code...
            button {
                class: "button",
                id: "logout_btn",
                onclick: move |_| {
                    nav.push(Route::Login {});
                },
                "LOG OUT"
            }
        }
    }
}