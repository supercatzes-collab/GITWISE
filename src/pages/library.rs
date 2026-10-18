
use crate::components::*;
use crate::database::{
    get_account_display_name,
    get_account_profile,
    get_all_items,
    initialize_database,
};
use dioxus::prelude::*;

#[component]
pub fn Library() -> Element {
    let mut search = use_signal(String::new);
    let query = search.read().to_lowercase();
    let me: Signal<String> = use_context();

    // Retrieve public works from SQLite.
    let works = use_signal(|| {
        let conn = initialize_database()
            .expect("Failed to initialize database");

        let items = get_all_items(&conn)
            .expect("Failed to retrieve library items");

        items
            .into_iter()
            .map(|item| {
                let author = get_account_display_name(&conn, &item.author_id)
                    .ok()
                    .flatten()
                    .unwrap_or_else(|| item.author_id.clone());
            
                let profile_img_path = get_account_profile(&conn, &item.author_id)
                    .ok()
                    .flatten()
                    .map(|profile| profile.profile_img_path)
                    .unwrap_or_default();
            
                let author_img = match profile_img_path.as_str() {
                    "assets/img_assets/profile_1.jpg" => {
                        asset!("/assets/img_assets/profile_1.jpg")
                    }
                    _ => asset!("/assets/img_assets/profile.jpg"),
                };
            
                WorkEntry {
                    id: item.id,
                    title: item.title,
                    author,
                    date: item.date,
                    thumbnail: get_thumbnail(&item.thumbnail_path),
                    author_img,
                }
            })

            .collect::<Vec<WorkEntry>>()
    });

    rsx! {
        div { class: "container", id: "mother_container",

            // Only renders the profile sidebar, not another mother_container.
            ProfileSidebar { user_id: me() }

            div { class: "container", id: "center_works_container",

                div { class: "container", id: "library_header",

                    h1 { id: "works_title", "PUBLIC LIBRARY" }

                    input {
                        id: "search_bar",
                        placeholder: "Search research title",
                        r#type: "text",
                        value: "{search}",
                        oninput: move |evt| search.set(evt.value()),
                    }
                }

                div { class: "container", id: "contents_container",

                    for work in works.read().iter().filter(|work| work.title.to_lowercase().contains(&query)) {
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
                }
            }
        }
    }
}
