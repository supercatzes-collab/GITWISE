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
    pub id: String,
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

#[derive(Clone, PartialEq)]
pub struct UserProfile {
    pub name: String,
    pub profile_img: Asset,
    pub credentials: Vec<String>,
}

//Temporary placeholder until the database function exists
pub fn placeholder_profile() -> UserProfile {
    UserProfile {
        name: "Alan Turing".into(),
        profile_img: asset!("/assets/img_assets/profile.jpg"),
        credentials: vec![
            "Software Engineer".into(),
            "Mathematician".into(),
            "Cryptanalyst".into(),
        ],
    }
}

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
