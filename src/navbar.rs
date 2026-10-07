use dioxus::prelude::*;

use crate::route::Route;

#[component]
pub fn Navbar() -> Element {
    let nav = navigator();
    let route = use_route::<Route>();

    let home_active = route == Route::Home {};
    let library_active = route == Route::Library {};

    rsx! {
        nav { id: "navbar", class: "container",
            button {
                class: if home_active { "nav_item active" } else { "nav_item" },
                id: "nav_home",
                onclick: move |_| {
                    nav.push(Route::Home {});
                },
                "home"
            }
            button {
                class: if library_active { "nav_item active" } else { "nav_item" },
                id: "nav_library",
                onclick: move |_| {
                    nav.push(Route::Library {});
                },
                "library"
            }
            button { class: "nav_item", id: "nav_settings",
                //onclick: move |_| {
                //    nav.push(Route::Settings {});
                //},
                "settings"
            }
        
        }
        Outlet::<Route> {}
    }
}