use dioxus::prelude::*;
use crate::route::Route;
use crate::pages::outgoing_requests::State;

#[component]
pub fn ProfileCard(children: Element) -> Element {
    rsx! {
        div { class: "container", id: "profile_details_container",
            div { class: "container", id: "profile_container",
                //make this dynamic
                img {
                    src: asset!("/assets/img_assets/profile.jpg"),
                    id: "profile_img",
                }
            }

            //make this dynamic
            div { class: "container", id: "profile_info_container",
                h1 { id: "profile_name", "Alan Turing" }
            }

            //make this dynamic
            div { class: "container", id: "profile_info_sub_container",
                ul {
                    id: "profile_info_ul",
                    style: "list-style: none; margin: 0; padding: 0;",
                    li { "Software Engineer" }
                    li { "Mathematician" }
                    li { "Cryptanalyst" }
                }
            }

            {children}
        }
    }
}

#[component]
pub fn WatchingUpdates( 
    id: String,
    img_src: Asset,
    title: String,
    author: String,
    on_dismiss: EventHandler<String>,
) -> Element {
    let nav = navigator();
    rsx! {
        div { class: "notification_content", id: "{id}",
            img { src: img_src, id: "thumbnail_img_notif" }

            h1 { class: "content_title", "{title}" }

            button {
                class: "button",
                id: "view_button",
                onclick: move |_| {
                    nav.push(Route::Download {});
                },
                "view"
            }
            button {
                class: "button",
                id: "dismiss_button",
                onclick: move |_| on_dismiss.call(id.clone()),
                "dismiss"
            }
        }
    }
}

#[component]
pub fn PendingUpdates(
    id: String,
    img_src: Asset,
    title: String,
    author_img_src: Asset,
    author: String,
    on_dismiss: EventHandler<String>,
) -> Element {
    let nav = navigator();
    
    rsx! {
        div { class: "notification_content_request", id: "{id}",
            //author header div
            div { class: "notification_author_row",
                span { class: "notification_author_label", "Push request by:" }
                img { src: author_img_src, class: "notification_author_img" }
                span { class: "notification_author_name", "{author}" }
                button {
                    class: "button",
                    id: "check_profile_button",
                    onclick: move |_| {
                        nav.push(Route::Profile {});
                    },
                    "check profile"
                }
            }

            div { class: "notification_body_row",
                img { src: img_src, id: "thumbnail_img_notif" }

                h1 { class: "content_title", "{title}" }

                button {
                    class: "button",
                    id: "view_button",
                    onclick: move |_| {
                        nav.push(Route::Download {});
                    },
                    "view"
                }
                button {
                    class: "button",
                    id: "dismiss_button",
                    onclick: move |_| on_dismiss.call(id.clone()),
                    "dismiss"
                }
            }
        }
    }
}

#[component]
pub fn OutgoingUpdates( 
    id: String,
    img_src: Asset,
    title: String,
    author: String,
    status: State,
    on_dismiss: EventHandler<String>,
) -> Element {
    let nav = navigator();

    let status_class = match status {
        State::Pending => "status_pending",
        State::Accepted => "status_accepted",
        State::Rejected => "status_rejected",
    };

    rsx! {
        div { class: "notification_content {status_class}", id: "{id}",
            img { src: img_src, id: "thumbnail_img_notif" }

            h1 { class: "content_title", "{title}" }

            button {
                class: "button",
                id: "view_button",
                onclick: move |_| {
                    nav.push(Route::Download {});
                },
                "view"
            }
            button {
                class: "button",
                id: "dismiss_button",
                onclick: move |_| on_dismiss.call(id.clone()),
                "dismiss"
            }
        }
    }
}