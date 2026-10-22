use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct PendingItem {
    pub id: String,
    pub title: String,
    pub author: String,
    pub thumbnail_src: Asset,
    pub author_img_src: Asset,
}

#[derive(Clone, PartialEq)]
pub struct OutgoingItem {
    pub id: String, // Proposal ID
    pub item_id: String, // Existing item ID
    pub title: String,
    pub author: String,
    pub thumbnail_src: Asset,
    pub status: State,
}

#[derive(Clone, Copy, PartialEq)]
pub enum State {
    Accepted,
    Rejected,
    Pending,
}

#[component]
pub fn FileDropzone(
    wrapper_id: &'static str,
    header_id: &'static str,
    input_id: &'static str,
    placeholder: &'static str,
    accept: &'static str,
    icon: Element,
    #[props(default = false)]
    directory: bool,
    on_select: EventHandler<String>,
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

                if directory {
                    button {
                        id: input_id,
                        r#type: "button",
                        onclick: move |_| {
                            if let Some(path) =
                                rfd::FileDialog::new().pick_folder()
                            {
                                on_select.call(path.to_string_lossy().into_owned());
                            }
                        },
                        "{placeholder}"
                    }
                } else {
                    input {
                        id: input_id,
                        r#type: "file",
                        accept,
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
}

//Account struct
#[derive(Clone, PartialEq)]
pub struct UserProfile {
    pub name: String,
    pub profile_img: Asset,
    pub credentials: Vec<String>,
}

//File items
#[derive(Clone, PartialEq)]
pub struct FileEntry {
    pub folder: String,
    pub name: String,
    pub size: usize,
    pub version: String,
    pub updated: String,
}

#[derive(Clone, PartialEq)]
pub struct WorkEntry {
    pub id: String,
    pub title: String,
    pub author: String,
    pub date: String,
    pub thumbnail: Asset,
    pub author_img: Asset,
}

#[derive(Clone, PartialEq)]
pub struct ContributorEntry {
    pub name: String,
    pub profile_img: Asset,
}

/////////////////////////////////

#[derive(Clone, Debug, PartialEq)]
pub struct Account {
    pub id: String,
    pub username: String,
    pub password: String,
    pub display_name: String,
    pub profile_img_path: String,
    pub credentials: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    pub id: String,
    pub title: String,
    pub abstract_text: String,
    pub author_id: String,
    pub date: String,
    pub version: String,
    pub thumbnail_path: String,
    pub folder_path: String,
    pub stars: i64,
    pub watching: i64,
    pub downloads: i64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Proposal {
    pub id: String,
    pub item_id: String,
    pub target_file_path: String,
    pub proposer_id: String,
    pub replacement_path: String,
    pub note: String,
    pub status: ProposalStatus,
    pub date: String,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProposalStatus {
    Pending,
    Accepted,
    Rejected,
}
