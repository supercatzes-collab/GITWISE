use dioxus::prelude::*;
use crate::components::general_components::*;

#[component]
pub fn Library() -> Element {
    rsx! {
        div { class: "container", id: "mother_container",

            ProfileCard {
                //this is supposed to take in data from the database.
                ProfileActions { pending_count: 2, watching_count: 1, outgoing_count: 0 }
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
                    //this is supposed to take in data from the database.
                    WorkItem {
                        id: "work_1",
                        img_src: asset!("/assets/img_assets/profile.jpg"),
                        thumbnail: asset!("/assets/img_assets/thumbnail.jpg"),
                        title: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore",
                        author: "ludwig von mises",
                        date: "09/25/26",
                    }
                }
            }
        }
    }
}