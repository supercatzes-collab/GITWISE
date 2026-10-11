use dioxus::prelude::*;
use crate::components::*;

//Note each individual right hand component should be unique to the file slected. So it must be a component too.


#[component] 
pub fn Contribute() -> Element {
    let mut chosen = use_signal(String::new);
    //let item = Item {
    //    id: "1".into(),
    //    title: "Lorem ipsum dolor sit amet".into(),
    //    other fields
    //};
    // later: let item = get_item(&item_id);
    let title = "Lorem ipsum dolor sit amet consectetur adipiscing elit";
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
                        FileItem {
                            title: "GITWISE",
                            selected_file: "chapter1.docx",
                            file_size: 10,
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
                            SelectedFileItemDetails {
                                selected_file: "parent",
                                version: "v1.2.3",
                                file_size: 18,
                                update: "5/05/26",
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
                        textarea { id: "text_area", placeholder: "Insert text here" }
                        button { class: "button", id: "upload_contribution_btn", "SUBMIT EDIT REQUEST" }
                        button { class: "button", id: "cancel_contribution_btn", "CANCEL" }
                    }
                }
            }
        }
    }
}
