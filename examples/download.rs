use dioxus::{html::{h1, ul}, prelude::*};
//const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const DOWNLOAD_CSS: Asset = asset!("/assets/download.css");
//const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        //document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: DOWNLOAD_CSS }
        //Router::<Route> {}
        Navbar {}
        Hero {}
    }
}

#[component]
fn Navbar() -> Element {
    rsx! {
        nav { id: "navbar", class: "container",
            div { class: "nav_item", id: "nav_home", "home" }
            div { class: "nav_item", id: "nav_library", "library" }
            div { class: "nav_item", id: "nav_watching", "watching" }
            div { class: "nav_item", id: "nav_settings", "settings" }
        }
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div { class: "container", id: "mother_container",
            div { class: "container", id: "profile_details_container",
                div { class: "container", id: "profile_container",
                    img {
                        src: asset!("/assets/img_assets/profile.jpg"),
                        id: "profile_img",
                    }
                }

                div { class: "container", id: "profile_info_container",
                    h1 { id: "profile_name", "Alan Turing" }
                }

                div { class: "container", id: "profile_info_sub_container",
                    ul {
                        id: "profile_info_ul",
                        style: "list-style: none; margin: 0; padding: 0;",
                        li { "Software Engineer" }
                        li { "Mathematician" }
                        li { "Cryptanalyst" }
                    }
                }

                button { class: "button", id: "view_profile_button", "VIEW PROFILE" }

                div { class: "container", id: "info_container",
                    p { id: "upload_date",
                        span { class: "info_label", "Uploaded: " }
                        span { class: "info_value", "1/18/1871 - 11/9/1918" }
                    }
                    p { id: "contributions_text",
                        span { class: "info_label", "Contributions: " }
                        span { class: "info_value", "420" }
                    }
                }
            }

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

            div { class: "container", id: "details_container",
                button { class: "stat_box", id: "stars", "STARS: 14k" }
                button { class: "stat_box", id: "watching_label", "WATCHING: 14k" }
                button { class: "stat_box", id: "downloads_label", "DOWNLOADS: 14k" }

                button { class: "button", id: "contribute_button", "CONTRIBUTE" }

                div { class: "container", id: "contributors_list",
                    h1 { id: "contributors_label", "CONTRIBUTORS: 4" }
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

