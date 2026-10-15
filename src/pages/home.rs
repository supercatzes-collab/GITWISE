use dioxus::prelude::*;
use crate::components::general_components::*;

#[component]
pub fn Home() -> Element {
    let me: Signal<String> = use_context();
    rsx! {
        ProfilePage { user_id: me() }
    }
}