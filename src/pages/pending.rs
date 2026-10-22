//pending.rs
use crate::components::*;
use dioxus::prelude::*;
use crate::database::{
    initialize_database,
    get_pending_proposals_for_owner,
};

#[component]
pub fn Pending() -> Element {
    let me: Signal<String> = use_context();
    
    let mut pending = use_signal(|| Vec::<PendingItem>::new());

    use_effect(move || {
        let conn = initialize_database()
            .expect("Failed to initialize database");
    
        let owner_id = me();
    
        let items = get_pending_proposals_for_owner(&conn, &owner_id)
            .expect("Failed to fetch pending proposals")
            .into_iter()
            .map(|proposal| PendingItem {
                // This is proposals.id, not items.id.
                id: proposal.id,
                title: proposal.item_title,
                author: proposal.proposer_name,
                author_img_src: get_thumbnail(&proposal.proposer_img_path),
                thumbnail_src: get_thumbnail(&proposal.item_thumbnail_path),
            })
            .collect::<Vec<PendingItem>>();
        
        pending.set(items);
    });

    //let nav = navigator();
    rsx! {
        div { class: "container", id: "mother_container",
            ProfileSidebar { user_id: me() }

            div { class: "container", id: "right_container",
                div { class: "container", id: "header", "PENDING CONTRIBUTIONS" }
                for item in pending.read().iter() {
                    PendingUpdates {
                        key: "{item.id}",
                        id: item.id.clone(),
                        img_src: item.thumbnail_src,
                        title: item.title.clone(),
                        author: item.author.clone(),
                        author_img_src: item.author_img_src,
                        on_dismiss: move |id: String| {
                            pending.write().retain(|p| p.id != id);
                        },
                    }
                }
            }
        }
    }
}
