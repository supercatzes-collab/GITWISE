use dioxus::{html::{h1, ul}, prelude::*};
//const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
//const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        //document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        //Router::<Route> {}
        Navbar {}
        Hero {}
    }
}

#[component]
fn Navbar() -> Element {
    rsx! {
        nav { id: "navbar", class: "container",
            div { class: "nav_item", id: "nav_home", "home" }
            div { class: "nav_item", id: "nav_library", "library" }
            div { class: "nav_item", id: "nav_watching", "watching" }
            div { class: "nav_item", id: "nav_settings", "settings" }
        }
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div { class: "container", id: "upload_mother_container",
            div { id: "upload_title",
                h1 { id: "upload_research_title", "RESEARCH TITLE" }
                input { id: "upload_input_title" }
            }

            div { id: "upload_abstract",
                h1 { id: "upload_header_abstract", "ABSTRACT" }
                input { id: "upload_input_abstract" }
            }

            div { id: "upload_image",
                h1 { id: "upload_header_image", "UPLOAD IMAGE" }
                input { id: "upload_input_image" }
            }

            div { id: "upload_zipfile",
                h1 { id: "upload_header_zipfile", "UPLOAD ZIP" }
                input { id: "upload_input_zipfile" }
            }

            button { class: "button", id: "upload_button" }
        }
    }
}

//function to build the contents object
#[component]
fn WorkItem(id: String, img_src: Asset, title: String) -> Element {
    rsx! {
        div { class: "works_item", id: "{id}",
            img { src: img_src, class: "content_img" }
            h1 { class: "content_title", "{title}" }
        }
    }
}
