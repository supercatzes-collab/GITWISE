use crate::database::{
    //add_item_contributor,
    get_pending_proposal_by_id,
    initialize_database,
    update_proposal_status,
};
use crate::Route;
use dioxus::prelude::*;

use crate::filesystem::{
    available_destination_path,
    copy_folder_recursive,
    ensure_storage_directories,
    environment_root,
    user_workspace_root,
    replace_item_folder,
};

use std::path::PathBuf;

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

    let original_folder_path_for_accept = proposal
        .read()
        .as_ref()
        .map(|request| request.original_folder_path.clone())
        .unwrap_or_default();
    
    let replacement_folder_path_for_accept = proposal
        .read()
        .as_ref()
        .map(|request| request.replacement_folder_path.clone())
        .unwrap_or_default();

    let review_download_proposal_id = proposal_id.clone();

    let review_download_source = proposal
        .read()
        .as_ref()
        .map(|request| request.replacement_folder_path.clone())
        .unwrap_or_default();
    
    let review_download_user_id = me.read().clone();

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
                                    let mut conn = match initialize_database() {
                                        Ok(conn) => conn,
                                        Err(error) => {
                                            eprintln!("Failed to open database: {error}");
                                            return;
                                        }
                                    };

                                    let owner_id = me.read().clone();

                                    let tx = match conn.transaction() {
                                        Ok(tx) => tx,
                                        Err(error) => {
                                            eprintln!("Failed to start transaction: {error}");
                                            return;
                                        }
                                    };

                                    // Stage the status update inside the transaction.
                                    // It will not become committed until folder replacement succeeds.
                                    let updated = match update_proposal_status(
                                        &tx,
                                        &accept_proposal_id,
                                        &owner_id,
                                        "Accepted",
                                    ) {
                                        Ok(updated) => updated,
                                        Err(error) => {
                                            eprintln!("Failed to update proposal: {error}");
                                            let _ = tx.rollback();
                                            return;
                                        }
                                    };

                                    if !updated {
                                        eprintln!(
                                            "Proposal was not accepted. Check its ID, \
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         owner ID, and current status.",
                                        );
                                        let _ = tx.rollback();
                                        return;
                                    }

                                    // Resolve both paths relative to Gitiwise_env when necessary.
                                    let stored_destination =
                                        PathBuf::from(&original_folder_path_for_accept);

                                    let destination = if stored_destination.is_absolute() {
                                        stored_destination
                                    } else {
                                        environment_root().join(stored_destination)
                                    };

                                    let stored_source =
                                        PathBuf::from(&replacement_folder_path_for_accept);

                                    let source = if stored_source.is_absolute() {
                                        stored_source
                                    } else {
                                        environment_root().join(stored_source)
                                    };
                                    let mut replacement = match replace_item_folder(&destination, &source) {
                                        Ok(replacement) => replacement,
                                        Err(error) => {
                                            eprintln!(
                                                "Folder replacement failed; proposal remains Pending: {error}",
                                            );

                                            if let Err(rollback_error) = tx.rollback() {
                                                eprintln!("Database rollback also failed: {rollback_error}");
                                            }

                                            return;
                                        }
                                    };

                                    // Commit Accepted only after the folder replacement succeeded.
                                    if let Err(error) = tx.commit() {
                                        eprintln!("Database commit failed after replacement: {error}");
                                        if let Err(restore_error) = replacement.rollback() {
                                            eprintln!(
                                                "CRITICAL: Could not restore the original folder: \
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             {restore_error}",
                                            );
                                        }

                                        return;
                                    }

                                    // The database commit succeeded, so the new library folder
                                    // is now authoritative. Remove the old backup.
                                    if let Err(error) = replacement.finalize() {
                                        eprintln!("Proposal accepted, but backup cleanup failed: {error}");
                                    }

                                    accept_nav
                                        .push(Route::Download {
                                            item_id: item_id_for_download.clone(),
                                        });
                                },

                                "ACCEPT REQUEST"
                            }

                            button {
                                class: "button",
                                id: "download_modified_folder_btn",

                                onclick: move |_| {
                                    let source_value = review_download_source.trim();

                                    if source_value.is_empty() {
                                        eprintln!("No replacement folder path was provided.");
                                        return;
                                    }

                                    // Resolve the contributor's submitted folder.
                                    let stored_source = PathBuf::from(source_value);

                                    let source = if stored_source.is_absolute() {
                                        stored_source
                                    } else {
                                        environment_root().join(stored_source)
                                    };

                                    if !source.is_dir() {
                                        eprintln!("Modified folder does not exist: {}", source.display());
                                        return;
                                    }

                                    // Ensure the reviewer's private workspace exists.
                                    if let Err(error) =
                                        ensure_storage_directories(&review_download_user_id)
                                    {
                                        eprintln!("Failed to prepare workspace: {error}");
                                        return;
                                    }
                                    let workspace = match user_workspace_root(&review_download_user_id) {
                                        Ok(path) => path,
                                        Err(error) => {
                                            eprintln!("Failed to locate workspace: {error}");
                                            return;
                                        }
                                    };

                                    // Store review copies separately from normal downloads.
                                    let review_root = workspace.join("reviewed_proposals");

                                    if let Err(error) = std::fs::create_dir_all(&review_root) {
                                        eprintln!("Failed to create review folder: {error}");
                                        return;
                                    }

                                    // Preferred name, based on the proposal ID.
                                    let desired_destination =
                                        review_root.join(&review_download_proposal_id);
                                    let destination = match available_destination_path(&desired_destination) {
                                        Ok(path) => path,
                                        Err(error) => {
                                            eprintln!("Failed to find an available review destination: {error}");
                                            return;
                                        }
                                    };

                                    // Copy the entire submitted folder.
                                    if let Err(error) =
                                        copy_folder_recursive(&source, &destination)
                                    {
                                        eprintln!("Failed to copy modified folder: {error}");
                                        return;
                                    }

                                    // Open this specific copy in Windows File Explorer.
                                    if let Err(error) =
                                        std::process::Command::new("explorer.exe")
                                        .arg(&destination)
                                        .spawn()
                                    {
                                        eprintln!(
                                            "Folder copied successfully, but File Explorer \
                                                                                                     could not open it: {error}",
                                        );
                                    }
                                },

                                "DOWNLOAD MODIFIED FOLDER"
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