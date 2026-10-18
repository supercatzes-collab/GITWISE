//general_components.rs
use crate::components::*;
use crate::components::shared_data_components::{UserProfile, WorkEntry};
use crate::database::{
    get_account_profile,
    initialize_database,
    get_items_by_author,
};
use crate::route::Route;
use dioxus::prelude::*;

// The profile card that displays all profile details.
#[component]
pub fn ProfileCard(profile: UserProfile, children: Element) -> Element {
    rsx! {
        div { class: "container", id: "profile_details_container",
            div { class: "container", id: "profile_container",
                img { src: profile.profile_img, id: "profile_img" }
            }

            div { class: "container", id: "profile_info_container",
                h1 { id: "profile_name", "{profile.name}" }
            }

            div { class: "container", id: "profile_info_sub_container",
                CredentialList { items: profile.credentials.clone() }
            }

            {children}
        }
    }
}

#[component]
pub fn ProfileSidebar(user_id: String) -> Element {
    let me: Signal<String> = use_context();
    let is_mine = user_id == me.read().clone();
    let profile_id = user_id.clone();

    let profile = use_resource(move || {
        let profile_id = profile_id.clone();

        async move {
            let conn = initialize_database()
                .map_err(|err| err.to_string())?;

            let db_profile = get_account_profile(&conn, &profile_id)
                .map_err(|err| err.to_string())?
                .ok_or_else(|| "Account profile not found".to_string())?;

            let credentials: Vec<String> =
                serde_json::from_str(&db_profile.credentials)
                    .unwrap_or_default();

            let profile_img = match db_profile.profile_img_path.as_str() {
                "assets/img_assets/profile_1.jpg" => {
                    asset!("/assets/img_assets/profile_1.jpg")
                }
                _ => asset!("/assets/img_assets/profile.jpg"),
            };

            Ok::<UserProfile, String>(UserProfile {
                name: db_profile.display_name,
                profile_img,
                credentials,
            })
        }
    });

    rsx! {
        match &*profile.read() {
            Some(Ok(profile_data)) => rsx! {
                ProfileCard { profile: profile_data.clone(),

                    if is_mine {
                        ProfileActions { pending_count: 2, watching_count: 1, outgoing_count: 0 }
                    }
                }
            },
            Some(Err(err)) => rsx! {
                p { "Could not load profile: {err}" }
            },
            None => rsx! {
                p { "Loading profile..." }
            },
        }
    }
}

#[component]
pub fn ProfilePage(user_id: String, show_works: bool) -> Element {
    let me: Signal<String> = use_context();
    let is_mine = user_id == me.read().clone();

    let author_id = user_id.clone();
    println!("Opening profile for user_id: {}", user_id);
    let works = use_resource(move || {
        let author_id = author_id.clone();

        async move {
            let conn = initialize_database()
                .expect("Failed to initialize database");

            let db_profile = get_account_profile(&conn, &author_id)
                .expect("Failed to retrieve account profile")
                .expect("Account profile not found");

            let author_name = db_profile.display_name;

            let author_img = match db_profile.profile_img_path.as_str() {
                "assets/img_assets/profile_1.jpg" => {
                    asset!("/assets/img_assets/profile_1.jpg")
                }
                _ => asset!("/assets/img_assets/profile.jpg"),
            };

            let items = get_items_by_author(&conn, &author_id)
                .expect("Failed to retrieve personal works");

            items
                .into_iter()
                .map(|item| WorkEntry {
                    id: item.id,
                    title: item.title,
                    author: author_name.clone(),
                    date: item.date,
                    thumbnail: get_thumbnail(&item.thumbnail_path),
                    author_img,
                })
                .collect::<Vec<WorkEntry>>()
        }
    });

    rsx! {
        div { class: "container", id: "mother_container",

            ProfileSidebar { user_id: user_id.clone() }

            if show_works {
                div { class: "container", id: "center_works_container",

                    div { class: "container", id: "home_header",

                        h1 { id: "works_title",
                            if is_mine {
                                "YOUR PERSONAL WORKS"
                            } else {
                                "THEIR PERSONAL WORKS"
                            }
                        }

                        if is_mine {
                            button {
                                id: "new_works_button",
                                onclick: move |_| {
                                    navigator().push(Route::Upload {});
                                },
                                "+"
                            }
                        }
                    }

                    div { class: "container", id: "contents_container",

                        match &*works.read() {
                            Some(items) => rsx! {
                                for work in items.iter() {
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
                            },
                            None => rsx! {
                                p { "Loading works..." }
                            },
                        }
                    }
                }
            }
        }
    }
}

pub(crate) fn get_thumbnail(filename: &str) -> Asset {
    // Extract the filename from a full Windows or Unix path.
    let filename = filename.rsplit(['/', '\\']).next().unwrap_or(filename);

    match filename {
        "profile.jpg" => asset!("/assets/img_assets/profile.jpg"),
        "thumbnail.jpg" => asset!("/assets/img_assets/thumbnail.jpg"),
        "profile_1.jpg" => asset!("/assets/img_assets/profile_1.jpg"),
        "profile_2.jpg" => asset!("/assets/img_assets/profile_2.jpg"),
        "thumbnail_1.jpg" => asset!("/assets/img_assets/thumbnail_1.jpg"),
        "thumbnail_2.gif" => asset!("/assets/img_assets/thumbnail_2.gif"),
        "thumbnail_3.jpg" => asset!("/assets/img_assets/thumbnail_3.jpg"),
        _ => asset!("/assets/img_assets/thumbnail.jpg"),
    }
}


#[component]
pub fn StatsBtns(
    stars: usize,
    watching: usize,
    downloads: usize,
    on_star: EventHandler<()>,
    on_watch: EventHandler<()>,
) -> Element {
    rsx! {
        button {
            class: "stat_box",
            id: "stars",
            onclick: move |_| on_star.call(()),
            "STARS: {stars}"
        }

        button {
            class: "stat_box",
            id: "watching_label",
            onclick: move |_| on_watch.call(()),
            "WATCHING: {watching}"
        }

        button { class: "stat_box", id: "downloads_label", "DOWNLOADS: {downloads}" }
    }
}

#[component]
pub fn CenterDetails(
    title: String,
    version: String,
    thumbnail: Asset,
    abstrakt: String,
    on_download: EventHandler<()>,
) -> Element {
    rsx! {
        h1 { id: "research_title", "{title}" }
        div { class: "tag", id: "version_tag", "{version}" }
        img { src: thumbnail, id: "thumbnail_img" }
        h2 { id: "abstract_label", "ABSTRACT" }
        p { id: "abstract_body", "{abstrakt}" }

        button {
            class: "button",
            id: "download_button",
            onclick: move |_| on_download.call(()),
            "download file"
        }
    }
}


// The details of the title in the download page.
#[component]
pub fn ItemDetails(upload_date: String, update_date: String) -> Element {
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
        }
    }
}

// Profile action buttons.
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
            "SEE EDIT REQUESTS: {pending_count}"
        }

        button {
            class: "button",
            id: "watching_button",
            onclick: move |_| {
                nav.push(Route::Watching {});
            },
            "WATCHING UPDATES: {watching_count}"
        }

        button {
            class: "button",
            id: "outgoing_button",
            onclick: move |_| {
                nav.push(Route::Outgoing {});
            },
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
                    nav.push(Route::Download {
                        item_id: "1".to_string(),
                    });
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

use chrono::{DateTime, Utc};

fn format_date(timestamp: &str) -> String {
    match timestamp.parse::<i64>() {
        Ok(seconds) => match DateTime::<Utc>::from_timestamp(seconds, 0) {
            Some(date) => date.format("%Y-%m-%d").to_string(),
            None => timestamp.to_string(),
        },
        Err(_) => timestamp.to_string(),
    }
}

#[component]
pub fn WorkItem(
    id: String,
    img_src: Asset,
    title: String,
    author: String,
    date: String,
    thumbnail: Asset,
) -> Element {
    let nav = navigator();
    let item_id = id.clone();
    let formatted_date = format_date(&date);

    rsx! {
        div {
            class: "works_item",
            id: "{id}",

            onclick: move |_| {
                nav.push(Route::Download {
                    item_id: item_id.clone(),
                });
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

                h1 { class: "content_date_class", id: "content_date_id_{id}", " | {formatted_date}" }
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

            div { class: "notification_author_row",
                span { class: "notification_author_label", "Push request by:" }
                img { src: author_img_src, class: "notification_author_img" }
                span { class: "notification_author_name", "{author}" }

                button {
                    class: "button",
                    id: "check_profile_button",
                    onclick: move |_| {
                        nav.push(Route::Profile {
                            user_id: "test-002".to_string(),
                        });
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
                    nav.push(Route::Download {
                        item_id: "1".to_string(),
                    });
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
    selected: bool,
    on_click: EventHandler<()>,
) -> Element {
    rsx! {
        p {
            class: "file_cards",
            id: "page_id",
            onclick: move |_| on_click.call(()),

            span { id: "details_text_1", class: "page_sub_text", "{title}" }
            span { id: "details_text_2", class: "page_sub_text", "/{selected_file}" }
            span { id: "details_text_3", class: "page_sub_text", "{file_size}KB" }
        }
    }
}

// Displays the details of the selected file.
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
pub fn ContributorsProfileList(profile_img: Asset, name: String) -> Element {
    rsx! {
        div { class: "contributors_card", id: "contributor_id",
            img { src: profile_img, class: "contributor_profile_img" }
            p { class: "contributor_name", id: "contributor_name_id", "{name}" }
        }
    }
}

#[component]
pub fn LogoutProfile(profile_img: Asset, username: String, date_created: String) -> Element {
    rsx! {
        img { id: "settings_profile_img", src: profile_img }

        div { class: "container", id: "settings_details_container",
            h1 { id: "settings_username", "{username}" }

            p { id: "settings_account_creation",
                span { id: "settings_text", "Account created: " }
                span { id: "settings_date", "{date_created}" }
            }
        }
    }
}

#[component]
pub fn ImageInput(
    img_id: &'static str,
    input_id: &'static str,
    img_src: Asset,
    on_select: EventHandler<String>,
) -> Element {
    rsx! {
        label { r#for: input_id,
            img { id: img_id, src: img_src }
        }

        input {
            id: input_id,
            r#type: "file",
            accept: "image/*",
            hidden: true,

            onchange: move |evt| {
                if let Some(file) = evt.files().first() {
                    on_select.call(file.path().to_string_lossy().to_string());
                }
            },
        }
    }
}

#[component]
pub fn FormField(
    label_id: &'static str,
    label: &'static str,
    input_id: &'static str,
    placeholder: &'static str,
    input_type: Option<&'static str>,
    on_input: EventHandler<String>,
) -> Element {
    rsx! {
        p { class: "login_text", id: label_id, "{label}" }

        input {
            class: "login_input",
            r#type: input_type,
            id: input_id,
            placeholder,

            oninput: move |evt| on_input.call(evt.value()),
        }
    }
}

#[component]
pub fn CredentialList(items: Vec<String>) -> Element {
    rsx! {
        ul {
            id: "profile_info_ul",
            style: "list-style: none; margin: 0; padding: 0;",

            for (i , item) in items.iter().enumerate() {
                li { key: "{i}", "{item}" }
            }
        }
    }
}
