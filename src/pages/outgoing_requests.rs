use crate::components::*;
use dioxus::prelude::*;
use crate::database::{
    get_outgoing_proposals_for_user,
    initialize_database,
};

#[component]
pub fn Outgoing() -> Element {
    let me: Signal<String> = use_context();

    let mut outgoing = use_signal(|| Vec::<OutgoingItem>::new());

    use_effect(move || {
        let conn = initialize_database()
            .expect("Failed to initialize database");
    
        let user_id = me();
    
        let items = get_outgoing_proposals_for_user(&conn, &user_id)
            .expect("Failed to fetch outgoing proposals")
            .into_iter()
            .map(|proposal| OutgoingItem {
                id: proposal.id,
                item_id: proposal.item_id,
                title: proposal.title,
                author: proposal.author,
                thumbnail_src: get_thumbnail(&proposal.thumbnail_path),
                status: match proposal.status.as_str() {
                    "Pending" => State::Pending,
                    "Accepted" => State::Accepted,
                    "Rejected" => State::Rejected,
                    other => panic!("Unexpected proposal status: {other}"),
                },
            })
            .collect::<Vec<OutgoingItem>>();
        
        outgoing.set(items);
    });

    rsx! {
        div { class: "container", id: "mother_container",
            ProfileSidebar { user_id: me() }

            div { class: "container", id: "right_container",
                div { class: "container", id: "header", "OUTGOING REQUESTS" }
                //These are the placeholder cards for the actual contents. They should be built dynamically.
                for item in outgoing.read().iter() {
                    OutgoingUpdates {
                        key: "{item.id}",
                        id: item.id.clone(),
                        item_id: item.item_id.clone(),
                        img_src: item.thumbnail_src.clone(),
                        title: item.title.clone(),
                        author: item.author.clone(),
                        status: item.status.clone(),
                        on_dismiss: move |id: String| {
                            outgoing.write().retain(|o| o.id != id);
                        },
                    }
                }
            }
        }
    }
}
