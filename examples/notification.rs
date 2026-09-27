use dioxus::{html::{h1, ul}, prelude::*};
//const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const NOTIFICATION_CSS: Asset = asset!("/assets/notifications.css");

//const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        //document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: NOTIFICATION_CSS }
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
            div { class: "nav_item", id: "nav_watching", "notification" }
            div { class: "nav_item", id: "nav_settings", "settings" }
        }
    }
}

//note notification_content & notification_content_request are seperate lists.

#[component]
pub fn Hero() -> Element {
    rsx! {
        div { class: "container", id: "mother_container",
            div { class: "container", id: "left_container",
                div { class: "container", id: "left_sub_container",
                    button { class: "button", id: "request_button", "SEE EDIT REQUESTS: 67" }
                    button { class: "button", id: "watching_button", "WATCHING UPDATES: 67" }
                }
            }
            div { class: "container", id: "right_container",
                div { class: "container", id: "header", "WATCHING TITLES" }
                //These are the placeholder cards for the actual contents. They should be built dynamically.
                div { class: "notification_content", id: "content_id",
                    img {
                        src: asset!("/assets/img_assets/thumbnail.jpg"),
                        id: "thumbnail_img",
                    }

                    h1 { class: "content_title",
                        "lorem ipsum dolor sit amet consectetur adipiscing elit velit libero temporibus occaecat occaecat occaecat occaecat"
                    }

                    button { class: "button", id: "view_button", "view" }
                    button { class: "button", id: "dismiss_button", "dismiss" }
                }
                //These are the placeholder cards for the actual contents. They should be built dynamically.
                //note for this card pressing "view" should open a page where it shows what file it is modifying/adding then a download button
                div { class: "notification_content_request", id: "content_id",
                    div { class: "notification_author_row",
                        span { class: "notification_author_label", "Push request by:" }
                        img {
                            src: asset!("/assets/img_assets/profile.jpg"),
                            class: "notification_author_img",
                        }
                        span { class: "notification_author_name", "Ada Lovelace" }
                        button { class: "button", id: "check_profile_button", "check profile" }
                    }

                    div { class: "notification_body_row",
                        img {
                            src: asset!("/assets/img_assets/thumbnail.jpg"),
                            id: "thumbnail_img",
                        }

                        h1 { class: "content_title",
                            "lorem ipsum dolor sit amet consectetur adipiscing elit velit libero temporibus occaecat occaecat occaecat occaecat"
                        }

                        button { class: "button", id: "view_button", "view" }
                        button { class: "button", id: "dismiss_button", "dismiss" }
                    }
                }
            }
        }
    }
}

