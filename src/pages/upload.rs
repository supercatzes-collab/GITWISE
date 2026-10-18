//upload.rs
use crate::components::shared_data_components::Item;
use crate::database;
use crate::route::Route;
use dioxus::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

#[component]
pub fn Upload() -> Element {
    let nav = navigator();
    let mut title = use_signal(String::new);
    let mut abstrakt = use_signal(String::new);
    let mut image_path = use_signal(String::new);
    let mut zip_path = use_signal(String::new);
    let mut upload_message = use_signal(String::new);
    let current_user: Signal<String> = use_context();
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
                onclick: move |_| {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or_default();

                    let item = Item {
                        id: format!("item-{}", now.as_millis()),
                        title: title.read().to_string(),
                        abstract_text: abstrakt.read().to_string(),
                        author_id: current_user.read().clone(),
                        date: now.as_secs().to_string(),
                        version: "1".to_string(),
                        thumbnail_path: image_path.read().to_string(),
                        folder_path: zip_path.read().to_string(),
                        stars: 0,
                        watching: 0,
                        downloads: 0,
                    };

                    let result = database::initialize_database()
                        .and_then(|conn| {
                            database::ensure_default_account(&conn)?;
                            database::insert_item(&conn, &item)
                        });

                    match result {
                        Ok(()) => {
                            nav.push(Route::Home {});
                        }
                        Err(err) => {
                            upload_message.set(format!("Upload failed: {err}"));
                        }
                    }
                },
                "UPLOAD"
            }

            if !upload_message.read().is_empty() {
                p { "{upload_message}" }
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
