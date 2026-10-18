use crate::components::*;
use crate::Route;
use dioxus::prelude::*;
//Note each individual right hand component should be unique to the file slected.
//So it must be a component too.
#[component]
pub fn IncomingRequest() -> Element {
    let nav = navigator();
    //let item = Item {
    //    id: "1".into(),
    //    title: "Lorem ipsum dolor sit amet".into(),
    //    description: "Long ass text".into(),
    //    other fields
    //};
    // later: let item = get_item(&item_id);
    //temporary placeholders
    let title = "Lorem ipsum dolor sit amet consectetur adipiscing elit";
    let description = "At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis 
                            praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias 
                            excepturi sint occaecati cupiditate non provident, similique sunt in culpa 
                            qui officia deserunt mollitia animi, id est laborum et dolorum fuga. Et harum 
                            quidem rerum facilis est et expedita distinctio. Nam libero tempore, cum soluta 
                            nobis est eligendi optio cumque nihil impedit quo minus id quod maxime placeat 
                            facere possimus, omnis voluptas assumenda est, omnis dolor repellendus. Temporibus 
                            autem quibusdam et aut officiis debitis aut rerum necessitatibus saepe eveniet ut et 
                            voluptates repudiandae sint et molestiae non recusandae. Itaque earum rerum hic 
                            tenetur a sapiente delectus, ut aut reiciendis voluptatibus maiores alias consequatur 
                            aut perferendis doloribus asperiores repellat.";
    rsx! {
        div { class: "container", id: "mother_container",
            div { class: "container", id: "main_container",
                div { class: "container", id: "header",
                    p { id: "header_text", "Reviewing request for:" }
                    h1 { id: "header_title", "{title}" }
                }

                div { class: "container", id: "secondary_main_container",
                    div { class: "container", id: "left_inner_container",
                        p { id: "left_inner_header", "Files changed:" }
                                        //Needs to replaced by an SQL fetch
                    //FileItem {
                    //    title: "GITWISE",
                    //    selected_file: "chapter1.docx",
                    //    file_size: 10,
                    //}
                    }

                    div { class: "container", id: "right_inner_container",
                        div { class: "container", id: "right_inner_header",
                            p {
                                id: "right_inner_header_text",
                                class: "page_sub_text",
                                "Contributors summary: "
                            }
                            SelectedFileItemDetails {
                                selected_file: "parent",
                                version: "v1.2.3",
                                file_size: 18,
                                update: "5/05/26",
                            }
                        }

                        p { class: "page_sub_text", id: "description", "{description}" }
                        //Updates original file version & redirects author to download page.
                        button {
                            class: "button",
                            id: "upload_contribution_btn",
                            onclick: move |_| {
                                nav.push(Route::Download {
                                    item_id: "1".to_string(),
                                });
                            },
                            "ACCEPT REQUEST"
                        }
                        //Update the state of the outgoing request of the requester and update ingoing request of the author.
                        button {
                            class: "button",
                            id: "cancel_contribution_btn",
                            onclick: move |_| {
                                nav.push(Route::Pending {});
                            },
                            "CANCEL"
                        }
                    }
                }
            }
        }
    }
}
