use crate::components::shared_data_components::*;
use dioxus::prelude::*;
//use crate::components::general_components::*;

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
