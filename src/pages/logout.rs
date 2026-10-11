use crate::route::Route;
use dioxus::prelude::*;
use crate::components::*;

#[component]
pub fn Settings() -> Element {
    let nav = navigator();
    //needs to be done to every profile list
    let mut credentials = use_signal(|| {
        vec![]
    });
    let mut draft = use_signal(String::new);
    rsx! {
        div { class: "container", id: "settings_main_container",
            div { class: "container", id: "settings_secondary_container",
                //needs to be swapped for an SQL fetch
                LogoutProfile {
                    profile_img: asset!("/assets/img_assets/profile.jpg"),
                    username: "Alice von neumann",
                    date_created: "09/10/26",
                }
            }

            div {
                input {
                    id: "cred_input",
                    placeholder: "Input credentials here",
                    value: "{draft}",
                    oninput: move |evt| draft.set(evt.value()),
                    onkeydown: move |evt| {
                        if evt.key() == Key::Enter {
                            let text = draft.read().trim().to_string();
                            if !text.is_empty() {
                                credentials.write().push(text);
                                draft.set(String::new());
                            }
                        }
                    },
                }
                CredentialList { items: credentials.read().clone() }
            }

            //Needs to wipe ALL structs unique to the user.
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