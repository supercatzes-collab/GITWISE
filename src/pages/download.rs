//download.rs
use crate::components::*;
use crate::database::{
    get_account_profile,
    get_item_by_id,
    increment_item_downloads,
    initialize_database,
    toggle_item_star,
    toggle_item_watch,
    //get_item_contributors,
};
use crate::Route;
use chrono::{Local, TimeZone};
use dioxus::prelude::*;

use crate::filesystem::{
    available_destination_path,
    copy_folder_recursive,
    ensure_storage_directories,
    environment_root,
    user_item_folder_path,
};
use std::path::PathBuf;

//#[derive(Clone, PartialEq)]
//struct ContributorProfileCardData {
//    user_id: String,
//    name: String,
//    profile_img: Asset,
//}
#[component]
pub fn Download(item_id: String) -> Element {
    let nav = navigator();
    let current_user: Signal<String> = use_context();
    let mut download_count_refresh = use_signal(|| 0_u32);
    let item = use_resource({
        let item_id = item_id.clone();
        move || {
            let item_id = item_id.clone();
            let _refresh = *download_count_refresh.read();
            async move {
                let conn = initialize_database()
                    .expect("Failed to initialize database");
                get_item_by_id(&conn, &item_id)
                    .expect("Failed to retrieve item")
            }
        }
    });

    //let contributor_item_id = item_id.clone();
    //let contributors = use_signal(move || {
    //    let conn = initialize_database()
    //        .expect("Failed to initialize database");
//
    //    get_item_contributors(&conn, &contributor_item_id)
    //        .expect("Failed to fetch item contributors")
    //        .into_iter()
    //        .map(|contributor| ContributorProfileCardData {
    //            user_id: contributor.user_id,
    //            name: contributor.display_name,
    //            profile_img: get_thumbnail(
    //                &contributor.profile_img_path,
    //            ),
    //        })
    //        .collect::<Vec<ContributorProfileCardData>>()
    //});
    //let contributor_count = contributors.read().len();
    let item_read = item.read();
    match &*item_read {
        // The item was found in the database.
        Some(Some(item)) => {
            let formatted_date = Local
                .timestamp_opt(
                    item.date.parse::<i64>().unwrap_or(0),
                    0,
                )
                .single()
                .map(|date| date.format("%B %d, %Y").to_string())
                .unwrap_or_else(|| "Unknown date".to_string());
            let author_id = item.author_id.clone();
            // Retrieve the item's actual author from the database.
            let conn = initialize_database()
                .expect("Failed to initialize database");
            let profile = get_account_profile(&conn, &author_id)
                .expect("Failed to retrieve author profile")
                .expect("Author profile not found");
            let credentials: Vec<String> =
                serde_json::from_str(&profile.credentials)
                    .unwrap_or_default();
            let profile_img = match profile.profile_img_path.as_str() {
                "assets/img_assets/profile_1.jpg" => {
                    asset!("/assets/img_assets/profile_1.jpg")
                }
                _ => asset!("/assets/img_assets/profile.jpg"),
            };
            let author_profile = UserProfile {
                name: profile.display_name,
                profile_img,
                credentials,
            };
            rsx! {
                div { class: "container", id: "mother_container",
                    ProfileCard { profile: author_profile,
                        button {
                            class: "button",
                            id: "view_profile_button",
                            onclick: {
                                let author_id = author_id.clone();
                                move |_| {
                                    nav.push(Route::Profile {
                                        user_id: author_id.clone(),
                                    });
                                }
                            },
                            "VIEW PROFILE"
                        }
                        ItemDetails {
                            upload_date: formatted_date.clone(),
                            update_date: formatted_date.clone(),
                        }
                    }
                    div { class: "container", id: "center_container",
                        CenterDetails {
                            title: item.title.clone(),
                            version: item.version.clone(),
                            thumbnail: get_thumbnail(&item.thumbnail_path),
                            abstrakt: item.abstract_text.clone(),
                            on_download: {
                                let download_item_id = item_id.clone();
                                let download_source_path = item.folder_path.clone();
                                let download_user_id = current_user.read().clone();

                                move |_| {
                                    // Ensure the library and current user's workspace exist.
                                    if let Err(error) =
                                        ensure_storage_directories(&download_user_id)
                                    {
                                        eprintln!("Failed to prepare storage directories: {error}");
                                        return;
                                    }

                                    // Resolve the source folder from the database path.
                                    let stored_source =
                                        PathBuf::from(&download_source_path);

                                    let source = if stored_source.is_absolute() {
                                        stored_source
                                    } else {
                                        environment_root().join(stored_source)
                                    };

                                    if !source.is_dir() {
                                        eprintln!(
                                            "Download failed: source folder does not exist: {}",
                                            source.display(),
                                        );
                                        return;
                                    }

                                    let desired_destination = match user_item_folder_path(
                                        &download_user_id,
                                        &download_item_id,
                                    ) {
                                        Ok(path) => path,
                                        Err(error) => {
                                            eprintln!("Failed to resolve destination: {error}");
                                            return;
                                        }
                                    };

                                    let destination =
                                        match available_destination_path(&desired_destination) {
                                        Ok(path) => path,
                                        Err(error) => {
                                            eprintln!("Failed to find an available folder name: {error}");
                                            return;
                                        }
                                    };

                                    // Copy the entire folder without overwriting existing data.
                                    if let Err(error) =
                                        copy_folder_recursive(&source, &destination)
                                    {
                                        eprintln!("Download failed: {error}");
                                        return;
                                    }

                                    // Count the download only after copying succeeds.
                                    let conn = match initialize_database() {
                                        Ok(conn) => conn,
                                        Err(error) => {
                                            eprintln!(
                                                "Folder copied, but database initialization failed: {error}",
                                            );
                                            return;
                                        }
                                    };

                                    if let Err(error) =
                                        increment_item_downloads(&conn, &download_item_id)
                                    {
                                        eprintln!("Folder copied, but download count update failed: {error}");
                                        return;
                                    }

                                    download_count_refresh += 1;

                                    println!(
                                        "Downloaded item {} to {}",
                                        download_item_id,
                                        destination.display(),
                                    );

                                    // Open File Explorer at the downloaded item's folder.
                                    if let Err(error) = std::process::Command::new("explorer.exe")
                                        .arg(&destination)
                                        .spawn()
                                    {
                                        eprintln!(
                                            "Download succeeded, but File Explorer could not open: {error}",
                                        );
                                    }
                                }
                            },
                        }
                    }
                    div { class: "container", id: "details_container",
                        StatsBtns {
                            stars: item.stars as usize,
                            watching: item.watching as usize,
                            downloads: item.downloads as usize,
                            on_star: {
                                let star_item_id = item_id.clone();
                                let star_user_id = current_user.read().clone();
                                move |_| {
                                    let conn = initialize_database()
                                        .expect("Failed to initialize database");
                                    toggle_item_star(&conn, &star_item_id, &star_user_id)
                                        .expect("Failed to toggle item star");
                                    download_count_refresh += 1;
                                }
                            },
                            on_watch: {
                                let watch_item_id = item_id.clone();
                                let watch_user_id = current_user.read().clone();
                                move |_| {
                                    println!(
                                        "Watching button: item_id={}, user_id={}",
                                        watch_item_id,
                                        watch_user_id,
                                    );
                                    let conn = initialize_database().expect("Failed to initialize database");
                                    toggle_item_watch(&conn, &watch_item_id, &watch_user_id)
                                        .expect("Failed to toggle item watch");
                                    download_count_refresh += 1;
                                }
                            },
                        }
                        button {
                            class: "button",
                            id: "contribute_button",
                            onclick: move |_| {
                                nav.push(Route::Contribute {
                                    item_id: item_id.clone(),
                                });
                            },
                            "CONTRIBUTE"
                        }
                                        //div { class: "container", id: "contributors_list",
                    //    h1 { id: "contributors_label", "CONTRIBUTORS: {contributor_count}" }
                    //    for contributor in contributors.read().iter() {
                    //        ContributorsProfileList {
                    //            key: "{contributor.user_id}",
                    //            profile_img: contributor.profile_img.clone(),
                    //            name: contributor.name.clone(),
                    //        }
                    //    }
                    //}
                    }
                }
            }
        }
        // The query completed, but no matching item exists.
        Some(None) => rsx! {
            div { class: "container", "Item not found." }
        },
        // The query is still running.
        None => rsx! {
            div { class: "container", "Loading item..." }
        },
    }
}