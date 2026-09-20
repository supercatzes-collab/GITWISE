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
                    }
                    input { id: "upload_input_image" }
                }
            }

            div { id: "upload_zipfile",
                h1 { id: "upload_header_zipfile", "UPLOAD ZIP" }
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
                        path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                        polyline { points: "17 8 12 3 7 8" }
                        line {
                            x1: "12",
                            y1: "3",
                            x2: "12",
                            y2: "15",
                        }
                    }
                    input { id: "upload_input_zipfile" }
                }
            }

            button { class: "button", id: "upload_button", "UPLOAD" }
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
