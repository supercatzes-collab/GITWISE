use crate::database::{
    //add_item_contributor,
    get_pending_proposal_by_id,
    initialize_database,
    update_proposal_status,
};
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn IncomingRequest(proposal_id: String) -> Element {
    let nav = navigator();
    let me: Signal<String> = use_context();

    // Fetch this specific proposal from SQLite.
    let proposal_id_for_query = proposal_id.clone();

    let proposal = use_signal(move || {
        let conn = initialize_database()
            .expect("Failed to initialize database");

        let owner_id = me();

        get_pending_proposal_by_id(
            &conn,
            &proposal_id_for_query,
            &owner_id,
        )
        .expect("Failed to fetch proposal")
    });

    // The Download page needs the item's ID, not the proposal ID.
    //let item_id_for_download = proposal
        //.read()
        //.as_ref()
        //.map(|request| request.item_id.clone())
        //.unwrap_or_default();

       let item_id_for_download = proposal
        .read()
        .as_ref()
        .map(|request| request.item_id.clone())
        .unwrap_or_default();

    //let accepted_proposer_id = proposal
    //    .read()
    //    .as_ref()
    //    .map(|request| request.proposer_id.clone())
    //    .unwrap_or_default();

    let accept_nav = nav.clone();
    let cancel_nav = nav.clone();
    let accept_proposal_id = proposal_id.clone();

    let reject_proposal_id = proposal_id.clone();
    let reject_nav = nav.clone();

    rsx! {
        div { class: "container", id: "mother_container",

            div { class: "container", id: "main_container",

                if let Some(request) = proposal.read().as_ref() {
                    div { class: "container", id: "header",

                        p { id: "header_text", "Reviewing contribution for:" }

                        h1 { id: "header_title", "{request.item_title}" }
                    }

                    div { class: "container", id: "secondary_main_container",

                        // Original folder belonging to the existing item.
                        div { class: "container", id: "left_inner_container",

                            p { id: "left_inner_header", "Original folder:" }

                            p { id: "original_folder_path", "{request.original_folder_path}" }
                        }

                        // Proposed replacement folder.
                        div { class: "container", id: "right_inner_container",

                            div { class: "container", id: "right_inner_header",

                                p {
                                    id: "right_inner_header_text",
                                    class: "page_sub_text",
                                    "Replacement folder submitted by:"
                                }

                                p { id: "contributor_name", "{request.proposer_name}" }

                                p { id: "replacement_folder_path",
                                    "{request.replacement_folder_path}"
                                }
                            }

                            p {
                                class: "page_sub_text",
                                id: "description_header",
                                "Contributor's note:"
                            }

                            p { id: "description", "{request.note}" }

                            button {
                                class: "button",
                                id: "upload_contribution_btn",
                                onclick: move |_| {
                                    let conn = initialize_database()
                                        .expect("Failed to initialize database");

                                    let owner_id = me.read().clone();

                                    let updated = update_proposal_status(
                                            &conn,
                                            &accept_proposal_id,
                                            &owner_id,
                                            "Accepted",
                                        )
                                        .expect("Failed to update proposal status");

                                    if updated {
                                        println!("Proposal accepted successfully.");

                                        accept_nav
                                            .push(Route::Download {
                                                item_id: item_id_for_download.clone(),
                                            });
                                    } else {
                                        println!(
                                            "Proposal was not updated. Check the proposal ID, \
                                                                                                                                                                                                                                                                                                                                                                                                                         owner ID, and current status.",
                                        );
                                    }
                                },
                                "ACCEPT REQUEST"
                            }

                            button {
                                class: "button",
                                id: "reject_contribution_btn",
                                onclick: move |_| {
                                    let conn = initialize_database()
                                        .expect("Failed to initialize database");

                                    let owner_id = me.read().clone();

                                    let updated = update_proposal_status(
                                            &conn,
                                            &reject_proposal_id,
                                            &owner_id,
                                            "Rejected",
                                        )
                                        .expect("Failed to reject proposal");

                                    if updated {
                                        reject_nav.push(Route::Pending {});
                                    } else {
                                        println!(
                                            "Proposal was not rejected. Check its ID, \
                                                                        owner ID, and current status.",
                                        );
                                    }
                                },
                                "REJECT REQUEST"
                            }

                            button {
                                class: "button",
                                id: "cancel_contribution_btn",

                                onclick: move |_| {
                                    cancel_nav.push(Route::Pending {});
                                },

                                "CANCEL"
                            }
                        }
                    }
                } else {
                    div { id: "proposal_not_found",

                        h1 { "Proposal not found" }

                        p {
                            "This request may no longer be pending, or it may not belong to your account."
                        }

                        button {
                            class: "button",
                            onclick: move |_| {
                                nav.push(Route::Pending {});
                            },
                            "BACK TO PENDING"
                        }
                    }
                }
            }
        }
    }
}