use dioxus::prelude::*;

pub mod components;
pub mod navbar;
pub mod pages;
pub mod route;


use route::Route;

fn main() {
    dioxus::launch(App);
}

const MAIN_CSS: Asset = asset!("/assets/main.css");
const HOME_CSS: Asset = asset!("/assets/home.css");
const LIBRARY_CSS: Asset = asset!("/assets/library.css");
const DOWNLOAD_CSS: Asset = asset!("/assets/download.css");
const NOTIFICATION_CSS: Asset = asset!("/assets/notifications.css");
const CONTRIBUTE_CSS: Asset = asset!("/assets/contribute.css");
const UPLOAD_CSS: Asset = asset!("/assets/upload.css");
const SETTINGS_CSS: Asset = asset!("/assets/settings.css");
const SIGN_CSS: Asset = asset!("/assets/sign.css");


#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: HOME_CSS }
        document::Link { rel: "stylesheet", href: LIBRARY_CSS }
        document::Link { rel: "stylesheet", href: DOWNLOAD_CSS }
        document::Link { rel: "stylesheet", href: NOTIFICATION_CSS }
        document::Link { rel: "stylesheet", href: CONTRIBUTE_CSS }
        document::Link { rel: "stylesheet", href: UPLOAD_CSS }
        document::Link { rel: "stylesheet", href: SETTINGS_CSS }
        document::Link { rel: "stylesheet", href: SIGN_CSS }
        Router::<Route> {}
    }
}