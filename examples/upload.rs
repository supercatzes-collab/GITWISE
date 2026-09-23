use dioxus::{html::{h1, ul}, prelude::*};
//const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const UPLOAD_CSS: Asset = asset!("/assets/upload.css");
//const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        //document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: UPLOAD_CSS }
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
                textarea { id: "upload_input_title" }
            }

            div { id: "upload_abstract",
                h1 { id: "upload_header_abstract", "ABSTRACT" }
                textarea { id: "upload_input_abstract" }
            }

            FileDropzone {
                wrapper_id: "upload_image",
                header_id: "upload_header_image",
                input_id: "upload_input_image",
                label: "UPLOAD IMAGE",
                accept: "image/*",
                icon: rsx! {
                    rect {
                        x: "3",
                        y: "3",
                        width: "18",
                        height: "18",
                        rx: "2",
                        ry: "2",
                    }
                    circle { cx: "8.5", cy: "8.5", r: "1.5" }
                    polyline { points: "21 15 16 10 5 21" }
                },
            }

            FileDropzone {
                wrapper_id: "upload_zipfile",
                header_id: "upload_header_zipfile",
                input_id: "upload_input_zipfile",
                label: "UPLOAD ZIP",
                accept: ".zip",
                icon: rsx! {
                    path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                    polyline { points: "17 8 12 3 7 8" }
                    line {
                        x1: "12",
                        y1: "3",
                        x2: "12",
                        y2: "15",
                    }
                },
            }

            button { class: "button", id: "upload_button", "UPLOAD" }
        }
    }
}

#[component]
fn FileDropzone(
    wrapper_id: &'static str,
    header_id: &'static str,
    input_id: &'static str,
    label: &'static str,
    accept: &'static str,
    icon: Element,
) -> Element {
    rsx! {
        div { id: wrapper_id,
            h1 { id: header_id, "{label}" }
            div {
                svg {
                    width: "40",
                    height: "40",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    {icon}
                }
                input { id: input_id, r#type: "file", accept }
            }
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