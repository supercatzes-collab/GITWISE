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

//TO DO:
#[component]
pub fn StatsBtns (
    stars: usize,
    watching: usize,
    downloads: usize,
) -> Element {
    rsx! {
        button { class: "stat_box", id: "stars", "STARS: {stars}" }
        button { class: "stat_box", id: "watching_label", "WATCHING: {watching}" }
        button { class: "stat_box", id: "downloads_label", "DOWNLOADS: {downloads}" }
    }
}

#[component]
pub fn CenterDetails(
    title: String,
    version: String,
    thumbnail: Asset,
    abstrakt: String,
) -> Element {
    rsx! {
        h1 { id: "research_title", "{title}" }
        div { class: "tag", id: "version_tag", "{version}" }
        img { src: thumbnail, id: "thumbnail_img" }
        h2 { id: "abstract_label", "ABSTRACT" }
        p { id: "abstract_body", "{abstrakt}" }
        button { class: "button", id: "download_button", "download file" }
    }
}

#[component] 
pub fn ItemDetails(
    upload_date: String,
    update_date: String,
    count: usize,
) -> Element {
    rsx! {
        div { class: "container", id: "info_container",
            p { id: "upload_date",
                span { class: "info_label", "Uploaded: " }
                span { class: "info_value", "{upload_date}" }
            }

            p { id: "update_date",
                span { class: "info_label", "Updated: " }
                span { class: "info_value", "{update_date}" }
            }

            p { id: "contributions_text",
                span { class: "info_label", "Contributions: " }
                span { class: "info_value", "{count}" }
            }
        }
    }
}

#[component]
pub fn ProfileActions(
    pending_count: usize,
    watching_count: usize,
    outgoing_count: usize,
) -> Element {
    let nav = navigator();
    rsx! {
        button {
            class: "button",
            id: "request_button",
            onclick: move |_| {
                nav.push(Route::Pending {});
            },
            //needs to be swapped for a dynmic value
            "SEE EDIT REQUESTS: {pending_count}"
        }
        button {
            class: "button",
            id: "watching_button",
            onclick: move |_| {
                nav.push(Route::Watching {});
            },
            //needs to be swapped for a dynmic value
            "WATCHING UPDATES: {watching_count}"
        }
        button {
            class: "button",
            id: "outgoing_button",
            onclick: move |_| {
                nav.push(Route::Outgoing {});
            },
            //needs to be swapped for a dynmic value
            "OUTGOING REQUESTS: {outgoing_count}"
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
pub fn WorkItem(id: String, 
    img_src: Asset, 
    title: String, 
    author: String, 
    date: String, 
    thumbnail: Asset,
) -> Element {
    let nav = navigator();

    rsx! {
        div {
            class: "works_item",
            id: "{id}",
            onclick: move |_| {
                nav.push(Route::Download {});
            },
            img { src: thumbnail, class: "content_img" }
            h1 { class: "content_title", "{title}" }
            div { class: "content_author", id: "content_author_id_{id}",
                img { src: img_src, class: "content_profile" }
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
                        nav.push(Route::IncomingRequest {});
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

#[component]
pub fn FileItem(
    title: String,
    selected_file: String,
    file_size: usize,
) -> Element {
    rsx! {
        p { class: "file_cards", id: "page_id",
            span { id: "details_text_1", class: "page_sub_text", "{title}" }
            span { id: "details_text_2", class: "page_sub_text", "/{selected_file}" }
            span { id: "details_text_3", class: "page_sub_text", "{file_size}KB" }
        }
    }
}

//needs to be modified to get entire file path.
#[component] 
pub fn SelectedFileItemDetails(
    selected_file: String,
    version: String,
    file_size: usize,
    update: String,
) -> Element {
    rsx! {
        h1 { id: "right_inner_header_title", "{selected_file}" }
        p { id: "details",
            span { id: "details_text_4", class: "page_sub_text", "{version} ^ " }
            span { id: "details_text_5", class: "page_sub_text", "{file_size}KB ^ " }
            span { id: "details_text_6", class: "page_sub_text", "updated: " }
            span { id: "details_text_7", class: "page_sub_text", "{update}" }
        }
    }
}

#[component]
pub fn ContributorsProfileList(
    count: usize,
    profile_img: Asset,
    name: String,
) -> Element {
    rsx! {
        h1 { id: "contributors_label", "CONTRIBUTORS: {count}" }
        div { class: "contributors_card", id: "contributor_id",
            img { src: profile_img, class: "contributor_profile_img" }
            p { class: "contributor_name", id: "contributor_name_id", "{name}" }
        }
    }
}
