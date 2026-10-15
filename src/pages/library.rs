use dioxus::prelude::*;
use crate::components::general_components::*;
use crate::components::shared_data_components::*;
#[component]
pub fn Library() -> Element {
    let mut search = use_signal(String::new);
    let query = search.read().to_lowercase();
    let works = use_signal(|| {
        vec![
            //this is supposed to take in data from the database.
            WorkEntry {
                id: "work_1".into(),
                title: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore".into(),
                author: "ludwig von mises".into(),
                date: "09/25/26".into(),
                thumbnail: asset!("/assets/img_assets/thumbnail.jpg"),
                author_img: asset!("/assets/img_assets/profile.jpg"),
            },
        ]
    });
    rsx! {
        div { class: "container", id: "mother_container",

            ProfileCard { profile: placeholder_profile() }

            //Needs to become a search funtion
            div { class: "container", id: "center_works_container",
                div { class: "container", id: "library_header",
                    h1 { id: "works_title", "PUBLIC LIBRARY" }
                    input {
                        id: "search_bar",
                        placeholder: "Search research title",
                        r#type: "text",
                        value: "{search}",
                        oninput: move |evt| search.set(evt.value()),
                    }
                }

                div { class: "container", id: "contents_container",
                    //this is supposed to take in data from the database.
                    for work in works.read().iter().filter(|w| w.title.to_lowercase().contains(&query)) {
                        WorkItem {
                            key: "{work.id}",
                            id: work.id.clone(),
                            img_src: work.author_img,
                            thumbnail: work.thumbnail,
                            title: work.title.clone(),
                            author: work.author.clone(),
                            date: work.date.clone(),
                        }
                    }
                }
            }
        }
    }
}