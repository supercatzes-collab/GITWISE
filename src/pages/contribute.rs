use dioxus::prelude::*;
//use crate::route::Route;

#[component] 
pub fn Contribute() -> Element {
    //let nav = navigator();
    rsx! {
        div { class: "container", id: "mother_container",
            div { class: "container", id: "main_container",
                div { class: "container", id: "contribute_header",
                    h1 { id: "contribute_title", "Short summary of what you changed" }
                    input {
                        class: "input",
                        id: "contribute_input",
                        placeholder: "Enter a short summary of your changes here",
                    }
                }
            
            }
        }
    }
}