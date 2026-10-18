//profile.rs
use crate::components::*;
use dioxus::prelude::*;

#[component]
pub fn Profile(user_id: String) -> Element {
    println!("Profile route received user_id: {}", user_id);
    rsx! {
        ProfilePage { user_id: user_id.clone(), show_works: true }
    }
}
