use dioxus::prelude::*;
use crate::components::*;


#[component]
pub fn Pending() -> Element {
    let mut pending = use_signal(|| {
        vec![
            //replace this with sql fetches
            PendingItem {
                id: "pending_1".into(),
                title: "lorem ipsum dolor sit amet consectetur adipiscing elit velit libero temporibus occaecat occaecat occaecat occaecat".into(),
                author: "Ada Lovelace".into(),
                author_img_src: asset!("/assets/img_assets/profile.jpg"),
                thumbnail_src: asset!("/assets/img_assets/thumbnail.jpg"),
            },
        ]
    });

    //let nav = navigator();
    rsx! {
        div { class: "container", id: "mother_container",
            ProfileCard {
                //replace this with sql fetches
                ProfileActions { pending_count: 2, watching_count: 1, outgoing_count: 0 }
            }

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