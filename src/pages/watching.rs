//watching.rs
use crate::components::*;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct WatchingItem {
    id: String,
    title: String,
    author: String,
    thumbnail_src: Asset,
}

use crate::database::{
    initialize_database,
    get_watched_items_for_user,
};

#[component]
pub fn Watching() -> Element {
   let me: Signal<String> = use_context();

    let mut watching = use_signal(|| Vec::<WatchingItem>::new());

    use_effect(move || {
        let conn = initialize_database()
            .expect("Failed to initialize database");

        let user_id = me();

        let items = get_watched_items_for_user(&conn, &user_id)
            .expect("Failed to fetch watched items")
            .into_iter()
            .map(|item| WatchingItem {
                id: item.id,
                title: item.title,
                author: item.author,
                thumbnail_src: get_thumbnail(&item.thumbnail_path),
            })
            .collect::<Vec<WatchingItem>>();

        watching.set(items);
    });
    rsx! {
        div { class: "container", id: "mother_container",
            ProfileSidebar { user_id: me() }

            div { class: "container", id: "right_container",
                div { class: "container", id: "header", "WATCHING TITLES" }
                //These are the placeholder cards for the actual contents. They should be built dynamically.
                for item in watching.read().iter() {
                    WatchingUpdates {
                        key: "{item.id}",
                        id: item.id.clone(),
                        img_src: item.thumbnail_src.clone(),
                        title: item.title.clone(),
                        author: item.author.clone(),
                        on_dismiss: move |id: String| {
                            watching.write().retain(|w| w.id != id);
                        },
                    }
                }
            }
        }
    }
}
