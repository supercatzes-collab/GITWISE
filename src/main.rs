use dioxus::prelude::*;

//#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
//enum Route {
//    #[layout(Navbar)]
//    #[route("/")]
//    Home {},
//    #[route("/blog/:id")]
//    Blog { id: i32 },
//}

//const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
//const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        //document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        //Router::<Route> {}
    }
}

#[component]
fn Navbar() -> Element {
    rsx! {
        nav { id: "navbar", class: "container",
            a { href: "/", "Home" }
            a { href: "/blog/1", "Blog 1" }
            a { href: "/blog/2", "Blog 2" }
        }
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {}
}
