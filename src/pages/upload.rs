use dioxus::prelude::*;
use crate::route::Route;

#[component]
pub fn Upload() -> Element {
    let nav = navigator();
    let mut title = use_signal(String::new);
    let mut abstrakt = use_signal(String::new);
    let mut image_path = use_signal(String::new);
    let mut zip_path = use_signal(String::new);
    rsx! {
        div { class: "container", id: "upload_mother_container",
            div { id: "upload_title",
                h1 { id: "upload_research_title", "TITLE" }
                textarea {
                    id: "upload_input_title",
                    placeholder: "Enter your research title here",
                    oninput: move |evt| title.set(evt.value()),
                }
            }

            div { id: "upload_abstract",
                h1 { id: "upload_header_abstract", "ABSTRACT" }
                textarea {
                    id: "upload_input_abstract",
                    placeholder: "Enter your research abstract here ",
                    oninput: move |evt| abstrakt.set(evt.value()),
                }
            }

            FileDropzone {
                wrapper_id: "upload_image",
                header_id: "upload_header_image",
                input_id: "upload_input_image",
                label: "UPLOAD IMAGE",
                accept: "image/*",
                directory: false,
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
                on_select: move |path: String| image_path.set(path),
            }

            FileDropzone {
                wrapper_id: "upload_zipfile",
                header_id: "upload_header_zipfile",
                input_id: "upload_input_zipfile",
                label: "UPLOAD ZIPFILE",
                accept: "folder/*",
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
                on_select: move |path: String| zip_path.set(path),
            }

            button {
                class: "button",
                id: "upload_button",
                //to do: send user back to home then builds the item
                onclick: move |_| {
                    //temporary: replace this with the function that handles the upload
                    println!(
                        "upload: title={} abstract={} image={} zip={}",
                        title.read().as_str(),
                        abstrakt.read().as_str(),
                        image_path.read().as_str(),
                        zip_path.read().as_str(),
                    );
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
    on_select: EventHandler<String>,
    directory: bool,
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
                input {
                    id: input_id,
                    r#type: "file",
                    accept,
                    directory,
                    onchange: move |evt| {
                        if let Some(file) = evt.files().first() {
                            on_select.call(file.path().to_string_lossy().to_string());
                        }
                    },
                }
            }
        }
    }
}