use dioxus::prelude::*;

use crate::components::ProfileCard;
use crate::components::WorkItem;
#[component]
pub fn Profile() -> Element {
    rsx! {
        div { class: "container", id: "mother_container",
            ProfileCard {
            }

            div { class: "container", id: "center_works_container",
                div { class: "container", id: "home_header",
                    h1 { id: "works_title", "PERSONAL WORKS" }
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