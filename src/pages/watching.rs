//watching.rs
use crate::components::*;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct WatchingItem {
    id: String,
    title: String,
    author: String,
    thumbnail_src: Asset,
}

#[component]
pub fn Watching() -> Element {
    let mut watching = use_signal(|| {
        vec![
            //should be done through an api call from sqlite database
            WatchingItem {
                id: "watching_1".into(),
                title: "lorem ipsum dolor sit amet consectetur adipiscing elit velit libero temporibus occaecat occaecat occaecat occaecat".into(),
                author: "Alan Turing".into(),
                thumbnail_src: asset!("/assets/img_assets/thumbnail.jpg"),
            },
        ]
    });

    rsx! {
        div { class: "container", id: "mother_container",
            ProfileCard { profile: placeholder_profile(),
                ProfileActions { pending_count: 2, watching_count: 1, outgoing_count: 0 }
            }

            div { class: "container", id: "right_container",
                div { class: "container", id: "header", "WATCHING TITLES" }
                //These are the placeholder cards for the actual contents. They should be built dynamically.
                for item in watching.read().iter() {
                    WatchingUpdates {
                        key: "{item.id}",
                        id: item.id.clone(),
                        img_src: item.thumbnail_src.clone(),
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
