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
            div { class: "nav_item", "library" }
            div { class: "nav_item", "notif" }
            div { class: "nav_item", "settings" }
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

            div { class: "container", id: "center_container",
                div { class: "container", id: "header",
                    h1 { id: "works_title", "WORKS" }
                    button { class: "button", id: "new_works_button", "NEW" }
                }

                //example of calling the work_item object builder
                //WorkItem {
                //    id: "work_1",
                //    img_src: asset!("/assets/img_assets/thumbnail.jpg"),
                //    title: "Lorem ipsum dolor sit amet...",
                //}

                //what the object should look like
                div { class: "container", id: "contents_container",
                    div { class: "works_item", id: "work_id",
                        img {
                            src: asset!("/assets/img_assets/thumbnail.jpg"),
                            class: "content_img",
                        }
                        h1 { class: "content_title",
                            "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore 
                            "
                        }
                    }
                }
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
