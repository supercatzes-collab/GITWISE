use dioxus::{html::{h1, ul}, prelude::*};
//const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const DOWNLOAD_CSS: Asset = asset!("/assets/download.css");
//const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        //document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: DOWNLOAD_CSS }
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
        div { class: "container", id: "mother_container",
            div { class: "container", id: "profile_details_container",
                div { class: "container", id: "profile_container",
                    img {
                        src: asset!("/assets/img_assets/profile.jpg"),
                        id: "profile_img",
                    }
                }

                div { class: "container", id: "profile_info_container",
                    h1 { id: "profile_name", "Alan Turing" }
                }

                div { class: "container", id: "profile_info_sub_container",
                    ul {
                        id: "profile_info_ul",
                        style: "list-style: none; margin: 0; padding: 0;",
                        li { "Software Engineer" }
                        li { "Mathematician" }
                        li { "Cryptanalyst" }
                    }
                }
            }

            div { class: "container", id: "center_container" }

            div { class: "container", id: "details_container" }
        }
    }
}

