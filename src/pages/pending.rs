use dioxus::prelude::*;
use crate::components::*;
use crate::route::Route;

#[derive(Clone, PartialEq)]
struct PendingItem {
    id: String,
    title: String,
    author: String,
    author_img_src: Asset,
}

#[component]
pub fn Pending() -> Element {
    let mut pending = use_signal(|| {
        vec![
            PendingItem {
                id: "pending_1".into(),
                title: "lorem ipsum dolor sit amet consectetur adipiscing elit velit libero temporibus occaecat occaecat occaecat occaecat".into(),
                author: "Ada Lovelace".into(),
                author_img_src: asset!("/assets/img_assets/profile.jpg"),
            },
        ]
    });

    let nav = navigator();
    rsx! {
        div { class: "container", id: "mother_container",
            ProfileCard {
                button {
                    class: "button",
                    id: "request_button",
                    onclick: move |_| {
                        nav.push(Route::Pending {});
                    },
                    //needs to be swapped for a dynmic value
                    "SEE EDIT REQUESTS: 67"
                }
                button {
                    class: "button",
                    id: "watching_button",
                    onclick: move |_| {
                        nav.push(Route::Watching {});
                    },
                    //needs to be swapped for a dynmic value
                    "WATCHING UPDATES: 67"
                }

                button {
                    class: "button",
                    id: "outgoing_button",
                    onclick: move |_| {
                        nav.push(Route::Outgoing {});
                    },
                    //needs to be swapped for a dynmic value
                    "OUTGOING REQUESTS: 67"
                }
            }

            div { class: "container", id: "right_container",
                div { class: "container", id: "header", "PENDING CONTRIBUTIONS" }
                for item in pending.read().iter() {
                    PendingUpdates {
                        key: "{item.id}",
                        id: item.id.clone(),
                        img_src: asset!("/assets/img_assets/thumbnail.jpg"),
                        title: item.title.clone(),
                        author: item.author.clone(),
                        author_img_src: item.author_img_src,
                        on_dismiss: move |id: String| {
                            pending.write().retain(|p| p.id != id);
                        },
                    }
                }
            }
        }
    }
}