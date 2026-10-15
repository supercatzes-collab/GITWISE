use dioxus::prelude::*;
use crate::components::*;

#[component]
pub fn Outgoing() -> Element {
    let mut outgoing = use_signal(|| {
        vec![
            //replace these items with sql fetches
            OutgoingItem {
                id: "outgoing_1".into(),
                title: "lorem ipsum dolor sit amet consectetur adipiscing elit velit libero temporibus occaecat occaecat occaecat occaecat".into(),
                author: "Alan Turing".into(),
                thumbnail_src: asset!("/assets/img_assets/thumbnail.jpg"),
                status: State::Pending
            },

            OutgoingItem {
                id: "outgoing_2".into(),
                title: "lorem ipsum dolor sit amet consectetur adipiscing elit velit libero temporibus occaecat occaecat occaecat occaecat".into(),
                author: "Alan Turing".into(),
                thumbnail_src: asset!("/assets/img_assets/thumbnail.jpg"),
                status: State::Rejected
            },

            OutgoingItem {
                id: "outgoing_3".into(),
                title: "lorem ipsum dolor sit amet consectetur adipiscing elit velit libero temporibus occaecat occaecat occaecat occaecat".into(),
                author: "Alan Turing".into(),
                thumbnail_src: asset!("/assets/img_assets/thumbnail.jpg"),
                status: State::Accepted
            },
        ]
    });

    rsx!{
        div { class: "container", id: "mother_container",
            ProfileCard { profile: placeholder_profile(),
                ProfileActions { pending_count: 2, watching_count: 1, outgoing_count: 0 }
            }

            div { class: "container", id: "right_container",
                div { class: "container", id: "header", "OUTGOING REQUESTS" }
                //These are the placeholder cards for the actual contents. They should be built dynamically.
                for item in outgoing.read().iter() {
                    OutgoingUpdates {
                        key: "{item.id}",
                        id: item.id.clone(),
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
