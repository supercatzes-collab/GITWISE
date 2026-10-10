

use dioxus::prelude::*;
//use crate::route::Route;

//Note each individual right hand component should be unique to the file slected. So it must be a component too.
#[component] 
pub fn IncomingRequest() -> Element {
    //let nav = navigator();
    rsx! {
        div { class: "container", id: "mother_container",
            div { class: "container", id: "main_container",
                div { class: "container", id: "header",
                    p { id: "header_text", "Reviewing request for:" }
                    h1 { id: "header_title", "Lorem ipsum dolor sit amet consectetur adipiscing elit" }
                }

                div { class: "container", id: "secondary_main_container",
                    div { class: "container", id: "left_inner_container",
                        p { id: "left_inner_header", "Files changed:" }
                        //Needs to be replaced with a component function
                        p { class: "file_cards", id: "page_id",
                            span { id: "details_text_1", class: "page_sub_text", "GITWISE" }
                            span { id: "details_text_2", class: "page_sub_text", "/chapter1.docx" }
                            span { id: "details_text_3", class: "page_sub_text", "8KB" }
                        }
                    }

                    div { class: "container", id: "right_inner_container",
                        //Needs to be replaced with a component function
                        div { class: "container", id: "right_inner_header",
                            p {
                                id: "right_inner_header_text",
                                class: "page_sub_text",
                                "Contributors summary: "
                            }
                            h1 { id: "right_inner_header_title", "parent/file.docx" }
                            p { id: "details",
                                span {
                                    id: "details_text_4",
                                    class: "page_sub_text",
                                    "v1.2.3 ^ "
                                }
                                span {
                                    id: "details_text_5",
                                    class: "page_sub_text",
                                    "48KB ^ "
                                }
                                span {
                                    id: "details_text_6",
                                    class: "page_sub_text",
                                    "submitted: "
                                }
                                span {
                                    id: "details_text_7",
                                    class: "page_sub_text",
                                    "5/05/26"
                                }
                            }
                        }

                        p { class: "page_sub_text", id: "description",
                            "At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate non provident, similique sunt in culpa qui officia deserunt mollitia animi, id est laborum et dolorum fuga. Et harum quidem rerum facilis est et expedita distinctio. Nam libero tempore, cum soluta nobis est eligendi optio cumque nihil impedit quo minus id quod maxime placeat facere possimus, omnis voluptas assumenda est, omnis dolor repellendus. Temporibus autem quibusdam et aut officiis debitis aut rerum necessitatibus saepe eveniet ut et voluptates repudiandae sint et molestiae non recusandae. Itaque earum rerum hic tenetur a sapiente delectus, ut aut reiciendis voluptatibus maiores alias consequatur aut perferendis doloribus asperiores repellat."
                        }
                        //Updates original file version & redirects author to download page.
                        button { class: "button", id: "upload_contribution_btn", "ACCEPT REQUEST" }
                        //Update the state of the outgoing request of the requester and update ingoing request of the author.
                        button { class: "button", id: "cancel_contribution_btn", "CANCEL" }
                    }
                }
            }
        }
    }
}