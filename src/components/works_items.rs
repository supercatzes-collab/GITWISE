use dioxus::prelude::*;

use crate::route::Route;

//function to build the contents object
#[component]
pub fn WorkItem(id: String, img_src: Asset, title: String, author: String, date: String) -> Element {
    let nav = navigator();

    rsx! {
        div {
            class: "works_item",
            id: "{id}",
            onclick: move |_| {
                nav.push(Route::Download {});
            },
            img { src: img_src, class: "content_img" }
            h1 { class: "content_title", "{title}" }
            div { class: "content_author", id: "content_author_id_{id}",
                img {
                    src: asset!("/assets/img_assets/profile.jpg"),
                    class: "content_profile",
                }
                h1 {
                    class: "content_author_class_text",
                    id: "content_author_id_text_{id}",
                    "{author}"
                }
                h1 { class: "content_date_class", id: "content_date_id_{id}", " | {date}" }
            }
        }
    }
}