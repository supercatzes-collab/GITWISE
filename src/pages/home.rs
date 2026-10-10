use dioxus::prelude::*;

use crate::components::general_components::*;
use crate::route::Route;

#[component]
pub fn Home() -> Element {
    let nav = navigator();
    rsx! {
        div { class: "container", id: "mother_container",
            ProfileCard {
                //replace this with sql fetches
                ProfileActions { pending_count: 2, watching_count: 1, outgoing_count: 0 }
            }

            div { class: "container", id: "center_works_container",
                div { class: "container", id: "home_header",
                    h1 { id: "works_title", "YOUR PERSONAL WORKS" }
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
                    //replace this with sql fetches
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