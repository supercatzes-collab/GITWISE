use dioxus::prelude::*;
use crate::components::*;

#[component]
pub fn Profile(user_id: String) -> Element {
    rsx! {
        ProfilePage { user_id }
    }
}