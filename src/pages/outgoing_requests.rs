use dioxus::prelude::*;
use crate::components::*;
use crate::route::Route;

#[derive(Clone, PartialEq)]
struct OutgoingItem {
    id: String,
    title: String,
    author: String,
    thumbnail_src: Asset,
    status: State
}

#[derive(Clone, Copy, PartialEq)]
pub enum State {
    Accepted,
    Rejected,
    Pending
}

#[component]
pub fn Outgoing() -> Element {
    let mut outgoing = use_signal(|| {
        vec![
            OutgoingItem {
                id: "outgoing_1".into(),
                title: "lorem ipsum dolor sit amet consectetur adipiscing elit velit libero temporibus occaecat occaecat occaecat occaecat".into(),
                author: "Alan Turing".into(),
                thumbnail_src: asset!("/assets/img_assets/thumbnail.jpg"),
                status: State::Pending
            },

            OutgoingItem {
                id: "outgoing_2".into(),
                title: "lorem ipsum dolor sit amet consectetur adipiscing elit velit libero temporibus occaecat occaecat occaecat occaecat".into(),
                author: "Alan Turing".into(),
                thumbnail_src: asset!("/assets/img_assets/thumbnail.jpg"),
                status: State::Rejected
            },

            OutgoingItem {
                id: "outgoing_3".into(),
                title: "lorem ipsum dolor sit amet consectetur adipiscing elit velit libero temporibus occaecat occaecat occaecat occaecat".into(),
                author: "Alan Turing".into(),
                thumbnail_src: asset!("/assets/img_assets/thumbnail.jpg"),
                status: State::Accepted
            },
        ]
    });

    let nav = navigator();
    rsx!{
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
                div { class: "container", id: "header", "OUTGOING REQUESTS" }
                //These are the placeholder cards for the actual contents. They should be built dynamically.
                for item in outgoing.read().iter() {
                    OutgoingUpdates {
                        key: "{item.id}",
                        id: item.id.clone(),
                        img_src: item.thumbnail_src.clone(),
                        title: item.title.clone(),
                        author: item.author.clone(),
                        status: item.status.clone(),
                        on_dismiss: move |id: String| {
                            outgoing.write().retain(|o| o.id != id);
                        },
                    }
                }
            }
        }
    }
}
