//home.rs
use crate::components::general_components::*;
use dioxus::prelude::*;
#[component]
pub fn Home() -> Element {
    let me: Signal<String> = use_context();
    rsx! {
        ProfilePage { user_id: me(), show_works: true }
    }
}
