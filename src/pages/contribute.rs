use crate::components::*;
use crate::Route;
use dioxus::prelude::*;
use crate::database::{
    get_item_by_id,
    initialize_database,
    insert_proposal,
};
//Note each individual right hand component should be unique to the file slected. So it must be a component too.

#[component]
pub fn Contribute(item_id: String) -> Element {
    let mut chosen = use_signal(String::new);
    let mut note = use_signal(String::new);

    let title_item_id = item_id.clone();

    let title = use_signal(move || {
        let conn = initialize_database()
            .expect("Failed to initialize database");

        get_item_by_id(&conn, &title_item_id)
            .expect("Failed to fetch item")
            .map(|item| item.title)
            .unwrap_or_else(|| "Unknown item".to_string())
    });

    let submit_item_id = item_id.clone();
    let cancel_item_id = item_id.clone();

    let nav = navigator();
    let submit_nav = nav.clone();
    let cancel_nav = nav.clone();
    let me: Signal<String> = use_context();
    rsx! {
        div { class: "container", id: "mother_container",
            div { class: "container", id: "main_container",
                div { class: "container", id: "header",
                    p { id: "header_text", "Contributing to:" }
                    h1 { id: "header_title", "{title}" }
                }

                div { class: "container", id: "secondary_main_container",
                    div { class: "container", id: "left_inner_container",

                        p { id: "left_inner_header", "Replacement folder path:" }

                        if chosen.read().is_empty() {
                            p { class: "page_sub_text", "No folder selected yet." }
                        } else {
                            p { id: "selected_folder_path", "{chosen}" }
                        }
                    }

                    div { class: "container", id: "right_inner_container",
                        div { class: "container", id: "right_inner_header",

                            p {
                                id: "right_inner_header_text",
                                class: "page_sub_text",
                                "Replacing with folder:"
                            }

                            if chosen.read().is_empty() {
                                p { class: "page_sub_text", "No replacement folder selected." }
                            } else {
                                p { id: "replacement_folder_details", "{chosen}" }
                            }
                        }

                        //Needs to made database compatible, not sure how to do this part yet.
                        FileDropzone {
                            wrapper_id: "upload_zipfile",
                            header_id: "upload_header_zipfile",
                            input_id: "upload_input_zipfile",
                            placeholder: "SELECT FOLDER",
                            accept: "",
                            directory: true,

                            icon: rsx! {
                                path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                                polyline { points: "17 8 12 3 7 8" }
                                line {
                                    x1: "12",
                                    y1: "3",
                                    x2: "12",
                                    y2: "15",
                                }
                            },

                            on_select: move |path: String| {
                                chosen.set(path);
                            },
                        }

                        p {
                            id: "right_inner_header_text2",
                            class: "page_sub_text",
                            "Describe what changed"
                        }
                        //actions need to point ta a struct.
                        textarea {
                            id: "text_area",
                            placeholder: "Insert text here",
                            oninput: move |evt| note.set(evt.value()),
                        }
                        button {
                            class: "button",
                            id: "upload_contribution_btn",
                            onclick: move |_| {
                                let replacement_folder_path =
                                    chosen.read().trim().to_string();

                                let note_text = note.read().trim().to_string();
                                let proposer_id = me.read().clone();

                                if replacement_folder_path.is_empty() {
                                    println!("Please select a replacement folder first.");
                                    return;
                                }

                                let conn = initialize_database()
                                    .expect("Failed to initialize database");

                                insert_proposal(
                                        &conn,
                                        &submit_item_id,
                                        &proposer_id,
                                        &replacement_folder_path,
                                        &note_text,
                                    )
                                    .expect("Failed to submit contribution proposal");

                                submit_nav.push(Route::Outgoing {});
                            },
                            "SUBMIT EDIT REQUEST"
                        }
                        button {
                            class: "button",
                            id: "cancel_contribution_btn",
                            onclick: move |_| {
                                cancel_nav
                                    .push(Route::Download {
                                        item_id: cancel_item_id.clone(),
                                    });
                            },
                            "CANCEL"
                        }
                    }
                }
            }
        }
    }
}
