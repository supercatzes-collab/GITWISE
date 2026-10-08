use dioxus::prelude::*;
//use crate::route::Route;

#[component] 
pub fn Contribute() -> Element {
    //let nav = navigator();
    rsx! {
        div { class: "container", id: "mother_container",
            div { class: "container", id: "main_container",
                div { class: "container", id: "header",
                    p { id: "header_text", "Contributing to:" }
                    h1 { id: "header_title", "Lorem ipsum dolor sit amet consectetur adipiscing elit" }
                }

                div { class: "container", id: "secondary_main_container",
                    div { class: "container", id: "left_inner_container",
                        p { id: "left_inner_header", "Choose a file to replace:" }
                        p { class: "file_cards", id: "page_id",
                            span { class: "page_text", "GITWISE" }
                            span { class: "page_text2", "/chapter1.docx" }
                            span { class: "page_text3", "8KB" }
                        }
                        p { class: "file_cards", id: "page_id",
                            span { class: "page_text", "GITWISE" }
                            span { class: "page_text2", "/chapter2.docx" }
                            span { class: "page_text3", "8KB" }
                        }
                        p { class: "file_cards", id: "page_id",
                            span { class: "page_text", "GITWISE" }
                            span { class: "page_text2", "/chapter3.docx" }
                            span { class: "page_text3", "8KB" }
                        }
                        p { class: "file_cards", id: "page_id",
                            span { class: "page_text", "GITWISE" }
                            span { class: "page_text2", "/chapter4.docx" }
                            span { class: "page_text3", "8KB" }
                        }
                        p { class: "file_cards", id: "page_id",
                            span { class: "page_text", "GITWISE" }
                            span { class: "page_text2", "/chapter5.docx" }
                            span { class: "page_text3", "8KB" }
                        }
                    
                    }

                    div { class: "container", id: "right_inner_container",
                        div { class: "container", id: "right_inner_header",
                            p { id: "right_inner_header_text", "Replacing: " }
                            h1 { id: "right_inner_header_title", "parent/file.docx" }
                            p { id: "details",
                                span { class: "page_text4", "v1.2.3 ^ " }
                                span { class: "page_text5", "48KB ^ " }
                                span { class: "page_text6", "updated: " }
                                span { class: "page_text7", "5/05/26" }
                            }
                        }

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
                        }
                        p { id: "right_inner_header_text2", "Describe what changed" }
                        textarea { id: "text_area", placeholder: "Insert text here" }
                        button { class: "button", id: "upload_contribution_btn", "SUBMIT EDIT REQUEST" }
                        button { class: "button", id: "cancel_contribution_btn", "CANCEL" }
                    }
                }
            }
        }
    }
}

#[component]
fn FileDropzone(
    wrapper_id: &'static str,
    header_id: &'static str,
    input_id: &'static str,
    placeholder: &'static str,
    accept: &'static str,
    icon: Element,
) -> Element {
    rsx! {
        div { id: wrapper_id,
            div {
                svg {
                    width: "40",
                    height: "40",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    {icon}
                }
                input { id: input_id, r#type: "file", accept }
            }
        }
    }
}