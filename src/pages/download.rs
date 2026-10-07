use dioxus::prelude::*;
use crate::components::ProfileCard;
use crate::route::Route;

#[component]
pub fn Download() -> Element {
    let nav = navigator();
    rsx! {
        div { class: "container", id: "mother_container",
            ProfileCard {
                button {
                    class: "button",
                    id: "view_profile_button",
                    onclick: move |_| {
                        nav.push(Route::Profile {});
                    },
                    "VIEW PROFILE"
                }

                //convert this into component
                div { class: "container", id: "info_container",
                    p { id: "upload_date",
                        span { class: "info_label", "Uploaded: " }
                        span { class: "info_value", "1/18/1871" }
                    }

                    p { id: "update_date",
                        span { class: "info_label", "Updated: " }
                        span { class: "info_value", "5/05/1871" }
                    }

                    p { id: "contributions_text",
                        span { class: "info_label", "Contributions: " }
                        span { class: "info_value", "420" }
                    }
                }
            }

            //convert this into component
            div { class: "container", id: "center_container",
                h1 { id: "research_title",
                    "lorem ipsum dolor sit amet consectetur adipiscing elit velit libero temporibus occaecat commodo fugiat quis ducimus ipsum laboris excepteur qui fugiat et distinctio quibusdam quidem expedita"
                }

                div { class: "tag", id: "version_tag", "v1.2.3" }

                img {
                    src: asset!("/assets/img_assets/thumbnail.jpg"),
                    id: "thumbnail_img",
                }

                h2 { id: "abstract_label", "ABSTRACT" }

                p { id: "abstract_body",
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo. Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt. Neque porro quisquam est, qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit, sed quia non numquam eius modi tempora incidunt ut labore et dolore magnam aliquam quaerat voluptatem. Ut enim ad minima veniam, quis nostrum exercitationem ullam corporis suscipit laboriosam, nisi ut aliquid ex ea commodi consequatur. Quis autem vel eum iure reprehenderit qui in ea voluptate velit esse quam nihil molestiae consequatur, vel illum qui dolorem eum fugiat quo voluptas nulla pariatur. At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate non provident."
                }

                button { class: "button", id: "download_button", "download file" }
            }

            //convert into component
            div { class: "container", id: "details_container",
                button { class: "stat_box", id: "stars", "STARS: 14k" }
                button { class: "stat_box", id: "watching_label", "WATCHING: 14k" }
                button { class: "stat_box", id: "downloads_label", "DOWNLOADS: 14k" }

                button {
                    class: "button",
                    id: "contribute_button",
                    onclick: move |_| {
                        nav.push(Route::Contribute {});
                    },
                    "CONTRIBUTE"
                }

                div { class: "container", id: "contributors_list",
                    h1 { id: "contributors_label", "CONTRIBUTORS: 4" }

                    //These are the placeholder cards for the actual contents. They should be built dynamically.
                    div { class: "contributors_card", id: "contributor_id",
                        img {
                            src: asset!("/assets/img_assets/profile.jpg"),
                            class: "contributor_profile_img",
                        }
                        p {
                            class: "contributor_name",
                            id: "contributor_name_id",
                            "Ada lovelace"
                        }
                    }
                }
            }
        }
    }
}