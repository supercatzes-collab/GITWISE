use dioxus::prelude::*;

use crate::components::*;
use crate::route::Route;

#[derive(Clone, PartialEq)]
struct WatchingItem {
    id: String,
    title: String,
    author: String,
}

#[component]
pub fn Watching() -> Element {
    let nav = navigator();

    let mut watching = use_signal(|| {
        vec![
            WatchingItem {
                id: "watching_1".into(),
                title: "lorem ipsum dolor sit amet consectetur adipiscing elit velit libero temporibus occaecat occaecat occaecat occaecat".into(),
                author: "Alan Turing".into(),
            },
        ]
    });

    rsx! {
        div { class: "container", id: "mother_container",
            ProfileCard {
                button {
                    class: "button",
                    id: "request_button",
                    onclick: move |_| {
                        nav.push(Route::Pending {});
                    },
                    "SEE EDIT REQUESTS: 67"
                }
                button {
                    class: "button",
                    id: "watching_button",
                    onclick: move |_| {
                        nav.push(Route::Watching {});
                    },
                    "WATCHING UPDATES: 67"
                }
            }

            div { class: "container", id: "right_container",
                div { class: "container", id: "header", "WATCHING TITLES" }
                //These are the placeholder cards for the actual contents. They should be built dynamically.
                for item in watching.read().iter() {
                    WatchingUpdates {
                        key: "{item.id}",
                        id: item.id.clone(),
                        img_src: asset!("/assets/img_assets/thumbnail.jpg"),
                        title: item.title.clone(),
                        author: item.author.clone(),
                        on_dismiss: move |id: String| {
                            watching.write().retain(|w| w.id != id);
                        },
                    }
                }
            }
        }
    }
}
