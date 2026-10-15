use dioxus::prelude::*;
use crate::components::*;
use crate::Route;
#[component]
pub fn Download(item_id: String) -> Element {
    let nav = navigator();
    let contributors = use_signal(|| {
        vec![
            //replace this with sql fetches
            ContributorEntry {
                name: "Ada Lovelace".into(),
                profile_img: asset!("/assets/img_assets/profile.jpg"),
            },
        ]
    });
    let contributor_count = contributors.read().len();
    rsx! {
        div { class: "container", id: "mother_container",
            ProfileCard { profile: placeholder_profile(),

                button {
                    class: "button",
                    id: "view_profile_button",
                    onclick: move |_| {
                        nav.push(Route::Profile {
                            user_id: "2".to_string(),
                        });
                    },
                    "VIEW PROFILE"
                }

                ItemDetails {
                    //replace this with sql fetches
                    upload_date: "10/10/26",
                    update_date: "10/10/26",
                    count: 4,
                }
            }

            div { class: "container", id: "center_container",
                //replace this with sql fetches
                CenterDetails {
                    title: "lorem ipsum dolor sit amet consectetur adipiscing elit velit libero temporibus occaecat commodo fugiat quis ducimus ipsum laboris excepteur qui fugiat et distinctio quibusdam quidem expedita"
                        .to_string(),
                    version: "v1.2.3".to_string(),
                    thumbnail: asset!("/assets/img_assets/thumbnail.jpg"),
                    abstrakt: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo. Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt. Neque porro quisquam est, qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit, sed quia non numquam eius modi tempora incidunt ut labore et dolore magnam aliquam quaerat voluptatem. Ut enim ad minima veniam, quis nostrum exercitationem ullam corporis suscipit laboriosam, nisi ut aliquid ex ea commodi consequatur. Quis autem vel eum iure reprehenderit qui in ea voluptate velit esse quam nihil molestiae consequatur, vel illum qui dolorem eum fugiat quo voluptas nulla pariatur. At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate non provident."
                        .to_string(),
                }
            }

            div { class: "container", id: "details_container",
                //replace this with sql fetches
                StatsBtns { stars: 10, watching: 10, downloads: 10 }

                button {
                    class: "button",
                    id: "contribute_button",
                    onclick: move |_| {
                        nav.push(Route::Contribute {
                            item_id: "1".to_string(),
                        });
                    },
                    "CONTRIBUTE"
                }
                //replace this with sql fetches
                div { class: "container", id: "contributors_list",
                    h1 { id: "contributors_label", "CONTRIBUTORS: {contributor_count}" }
                    for contributor in contributors.read().iter() {
                        ContributorsProfileList {
                            key: "{contributor.name}",
                            profile_img: contributor.profile_img,
                            name: contributor.name.clone(),
                        }
                    }
                }
            }
        }
    }
}