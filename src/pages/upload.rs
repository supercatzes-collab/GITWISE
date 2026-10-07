use dioxus::prelude::*;
use crate::route::Route;

#[component]
pub fn Upload() -> Element {
    let nav = navigator();
    rsx! {
        div { class: "container", id: "upload_mother_container",
            div { id: "upload_title",
                h1 { id: "upload_research_title", "RESEARCH TITLE" }
                textarea {
                    id: "upload_input_title",
                    placeholder: "Enter your research title here",
                }
            }

            div { id: "upload_abstract",
                h1 { id: "upload_header_abstract", "ABSTRACT" }
                textarea {
                    id: "upload_input_abstract",
                    placeholder: "Enter your research abstract here ",
                }
            }

            FileDropzone {
                wrapper_id: "upload_image",
                header_id: "upload_header_image",
                input_id: "upload_input_image",
                label: "UPLOAD IMAGE",
                accept: "image/*",
                icon: rsx! {
                    rect {
                        x: "3",
                        y: "3",
                        width: "18",
                        height: "18",
                        rx: "2",
                        ry: "2",
                    }
                    circle { cx: "8.5", cy: "8.5", r: "1.5" }
                    polyline { points: "21 15 16 10 5 21" }
                },
            }

            FileDropzone {
                wrapper_id: "upload_zipfile",
                header_id: "upload_header_zipfile",
                input_id: "upload_input_zipfile",
                label: "UPLOAD ZIP",
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

            //Change this to route to home.rs && call the function that will handle the upload of the research title, abstract, image, and zip file to the database.
            button {
                class: "button",
                id: "upload_button",
                //to do: send user back to home then builds the item
                onclick: move |_| {
                    nav.push(Route::Home {});
                },
                "UPLOAD"
            }
            button {
                class: "button",
                id: "cancel_button",
                onclick: move |_| {
                    nav.push(Route::Home {});
                },
                "CANCEL"
            }
        }
    }
}

#[component]
fn FileDropzone(
    wrapper_id: &'static str,
    header_id: &'static str,
    input_id: &'static str,
    label: &'static str,
    accept: &'static str,
    icon: Element,
) -> Element {
    rsx! {
        div { id: wrapper_id,
            h1 { id: header_id, "{label}" }
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