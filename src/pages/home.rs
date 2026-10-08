use dioxus::prelude::*;

use crate::components::ProfileCard;
use crate::components::WorkItem;
use crate::route::Route;

#[component]
pub fn Home() -> Element {
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

            div { class: "container", id: "center_works_container",
                div { class: "container", id: "home_header",
                    h1 { id: "works_title", "PERSONAL WORKS" }
                    button {
                        class: "button",
                        id: "new_works_button",
                        onclick: move |_| {
                            nav.push(Route::Upload {});
                        },
                        "+"
                    }
                }

                div { class: "container", id: "contents_container",
                    WorkItem {
                        id: "work_1",
                        img_src: asset!("/assets/img_assets/thumbnail.jpg"),
                        title: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore",
                        author: "ludwig von mises",
                        date: "09/25/26",
                    }
                }
            }
        }
    }
}