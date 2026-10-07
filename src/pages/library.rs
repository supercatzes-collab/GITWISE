use dioxus::prelude::*;

use crate::components::ProfileCard;
use crate::components::WorkItem;
use crate::route::Route;

#[component]
pub fn Library() -> Element {
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
                    //needs to be swapped for a dynamic value
                    "SEE EDIT REQUESTS: 67"
                }

                button {
                    class: "button",
                    id: "watching_button",
                    onclick: move |_| {
                        nav.push(Route::Watching {});
                    },
                    //needs to be swapped for a dynamic value
                    "WATCHING UPDATES: 67"
                }
            }

            div { class: "container", id: "center_works_container",
                div { class: "container", id: "library_header",
                    h1 { id: "works_title", "PUBLIC LIBRARY" }
                    input {
                        id: "search_bar",
                        placeholder: "Search research title",
                        r#type: "text",
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