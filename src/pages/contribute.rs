use dioxus::prelude::*;
use crate::components::*;
use crate::Route;
//Note each individual right hand component should be unique to the file slected. So it must be a component too.


#[component] 
pub fn Contribute(item_id: String) -> Element {
    //let item = Item {
    //    id: "1".into(),
    //    title: "Lorem ipsum dolor sit amet".into(),
    //    other fields
    //};
    // later: let item = get_item(&item_id);
    let nav = navigator();
    let mut chosen = use_signal(String::new);
    let title = "Lorem ipsum dolor sit amet consectetur adipiscing elit";
    let mut selected = use_signal(|| 0usize);
    let mut note = use_signal(String::new);
    let files = use_signal(|| {
        vec![
            //Needs to replaced by an SQL fetch
            FileEntry {
                folder: "GITWISE".into(),
                name: "chapter1.docx".into(),
                size: 8,
                version: "v1.2.3".into(),
                updated: "5/05/26".into(),
            },
            // more entries
        ]
    });
    let selected_file = files.read().get(*selected.read()).cloned();
    rsx! {
        div { class: "container", id: "mother_container",
            div { class: "container", id: "main_container",
                div { class: "container", id: "header",
                    p { id: "header_text", "Contributing to:" }
                    h1 { id: "header_title", "{title}" }
                }

                div { class: "container", id: "secondary_main_container",
                    div { class: "container", id: "left_inner_container",
                        p { id: "left_inner_header", "Choose a file to replace:" }
                        //Needs to replaced by an SQL fetch
                        for (i , file) in files.read().iter().enumerate() {
                            FileItem {
                                key: "{i}",
                                title: file.folder.clone(),
                                selected_file: file.name.clone(),
                                file_size: file.size,
                                selected: i == *selected.read(),
                                on_click: move |_| selected.set(i),
                            }
                        }
                    }

                    div { class: "container", id: "right_inner_container",
                        div { class: "container", id: "right_inner_header",
                            p {
                                id: "right_inner_header_text",
                                class: "page_sub_text",
                                "Replacing: "
                            }
                            //Needs to replaced by an SQL fetch
                            if let Some(file) = &selected_file {
                                SelectedFileItemDetails {
                                    selected_file: format!("{}/{}", file.folder, file.name),
                                    version: file.version.clone(),
                                    file_size: file.size,
                                    update: file.updated.clone(),
                                }
                            }
                        }

                        //Needs to made database compatible, not sure how to do this part yet.
                        FileDropzone {
                            wrapper_id: "upload_zipfile",
                            header_id: "upload_header_zipfile",
                            input_id: "upload_input_zipfile",
                            placeholder: "UPLOAD ZIP",
                            accept: "folder/*",
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
                                //temporary: replace this with the call that inserts the proposal
                                println!(
                                    "submit: file={} note={}",
                                    chosen.read().as_str(),
                                    note.read().as_str(),
                                );
                                nav.push(Route::Outgoing {});
                            },
                            "SUBMIT EDIT REQUEST"
                        }
                        button {
                            class: "button",
                            id: "cancel_contribution_btn",
                            onclick: move |_| {
                                nav.push(Route::Download {
                                    item_id: "1".to_string(),
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
