// // src/views/profile.rs
// use dioxus::prelude::*;

// #[component]
// pub fn Profile() -> Element {
//     rsx! {
//         div { class: "profile-page",
//                 div { class: "user-info",
//                     div { class: "container",
//                         div { class: "row",
//                             div { class: "col-xs-12 col-md-10 offset-md-1",
//                                 img {
//                                     class: "user-img",
//                                     src: "http://i.imgur.com/Qr71crq.jpg",
//                                 }
//                                 h4 { "Eric Simons" }
//                                 p {
//                                     "\n              Cofounder @GoThinkster, lived in Aol's HQ for a few months, kinda looks like Peeta from\n              the Hunger Games\n            "
//                                 }
//                                 button { class: "btn btn-sm btn-outline-secondary action-btn",
//                                     i { class: "ion-plus-round" }
//                                     "\n              \u{a0} Follow Eric Simons\n            "
//                                 }
//                                 button { class: "btn btn-sm btn-outline-secondary action-btn",
//                                     i { class: "ion-gear-a" }
//                                     "\n              \u{a0} Edit Profile Settings\n            "
//                                 }
//                             }
//                         }
//                     }
//                 }
//                 div { class: "container",
//                     div { class: "row",
//                         div { class: "col-xs-12 col-md-10 offset-md-1",
//                             div { class: "articles-toggle",
//                                 ul { class: "nav nav-pills outline-active",
//                                     li { class: "nav-item",
//                                         a {
//                                             class: "nav-link active",
//                                             href: "",
//                                             "My Articles"
//                                         }
//                                     }
//                                     li { class: "nav-item",
//                                         a { class: "nav-link", href: "", "Favorited Articles" }
//                                     }
//                                 }
//                             }
//                             div { class: "article-preview",
//                                 div { class: "article-meta",
//                                     a { href: "/profile/eric-simons",
//                                         img { src: "http://i.imgur.com/Qr71crq.jpg" }
//                                     }
//                                     div { class: "info",
//                                         a {
//                                             class: "author",
//                                             href: "/profile/eric-simons",
//                                             "Eric Simons"
//                                         }
//                                         span { class: "date", "January 20th" }
//                                     }
//                                     button { class: "btn btn-outline-primary btn-sm pull-xs-right",
//                                         i { class: "ion-heart" }
//                                         " 29\n              "
//                                     }
//                                 }
//                                 a {
//                                     class: "preview-link",
//                                     href: "/article/how-to-buil-webapps-that-scale",
//                                     h1 { "How to build webapps that scale" }
//                                     p { "This is the description for the post." }
//                                     span { "Read more..." }
//                                     ul { class: "tag-list",
//                                         li { class: "tag-default tag-pill tag-outline",
//                                             "realworld"
//                                         }
//                                         li { class: "tag-default tag-pill tag-outline",
//                                             "implementations"
//                                         }
//                                     }
//                                 }
//                             }
//                             div { class: "article-preview",
//                                 div { class: "article-meta",
//                                     a { href: "/profile/albert-pai",
//                                         img { src: "http://i.imgur.com/N4VcUeJ.jpg" }
//                                     }
//                                     div { class: "info",
//                                         a {
//                                             class: "author",
//                                             href: "/profile/albert-pai",
//                                             "Albert Pai"
//                                         }
//                                         span { class: "date", "January 20th" }
//                                     }
//                                     button { class: "btn btn-outline-primary btn-sm pull-xs-right",
//                                         i { class: "ion-heart" }
//                                         " 32\n              "
//                                     }
//                                 }
//                                 a {
//                                     class: "preview-link",
//                                     href: "/article/the-song-you",
//                                     h1 {
//                                         "The song you won't ever stop singing. No matter how hard you try."
//                                     }
//                                     p { "This is the description for the post." }
//                                     span { "Read more..." }
//                                     ul { class: "tag-list",
//                                         li { class: "tag-default tag-pill tag-outline",
//                                             "Music"
//                                         }
//                                         li { class: "tag-default tag-pill tag-outline",
//                                             "Song"
//                                         }
//                                     }
//                                 }
//                             }
//                             ul { class: "pagination",
//                                 li { class: "page-item active",
//                                     a { class: "page-link", href: "", "1" }
//                                 }
//                                 li { class: "page-item",
//                                     a { class: "page-link", href: "", "2" }
//                                 }
//                             }
//                         }
//                     }
//                 }
//         }
//     }
// }



// src/views/profile.rs
use dioxus::prelude::*;
use crate::Route;
use crate::api::profile::{fetch_profile, Profile as ProfileData};
use crate::stores::app_state::{AppState, AuthStatus};

#[component]
pub fn Profile(username: String) -> Element {
    let app_state = use_context::<Signal<AppState>>();
    let current_route = use_route();

    // 获取 username
    let username = match current_route {
        Route::Profile { username } => Some(username),
        _ => None,
    };

    // 请求用户资料
    let profile_resource = use_resource(move || {
        let username = username.clone();
        let user_token = app_state.read().user.read().as_ref().map(|user| user.token.clone());
        async move {
            if let Some(u) = username {
                fetch_profile(&u, user_token.as_deref()).await
            } else {
                None
            }
        }
    });

    let profile = match profile_resource.value()() {
        Some(Some(response)) => Some(response.profile),
        _ => None,
    };

    let logged_in_username = app_state.read().user.read().as_ref().map(|u| u.username.clone());

    if profile.is_none() {
        return rsx! {
            div { class: "profile-page",
                div { class: "container", "正在加载用户资料..." }
            }
        };
    }

    let profile = profile.unwrap();
    let is_own_profile = Some(profile.username.clone()) == logged_in_username;


    rsx! {
        div { class: "profile-page",
            div { class: "user-info",
                div { class: "container",
                    div { class: "row",
                        div { class: "col-xs-12 col-md-10 offset-md-1",
                            img {
                                class: "user-img",
                                src: profile.image.as_deref().unwrap_or(""),
                            }
                            h4 { "{profile.username}" }
                            if let Some(bio) = &profile.bio {
                                p { "{bio}" }
                            }

                            if is_own_profile {
                                // 如果是自己的资料页，显示编辑按钮
                                button { class: "btn btn-sm btn-outline-secondary action-btn",
                                    i { class: "ion-gear-a" }
                                    " Edit Profile Settings"
                                }
                            } else {
                                // 如果不是自己的资料页，显示关注/取消关注按钮
                                button {
                                    class: "btn btn-sm btn-outline-secondary action-btn",
                                    // onclick: on_follow_click,
                                    i {
                                        class: if profile.following { "ion-minus-round" } else { "ion-plus-round" }
                                    }
                                    if profile.following {
                                        " Unfollow {profile.username}"
                                    } else {
                                        " Follow {profile.username}"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            div { class: "container",
                div { class: "row",
                    div { class: "col-xs-12 col-md-10 offset-md-1",
                        div { class: "articles-toggle",
                            ul { class: "nav nav-pills outline-active",
                                li { class: "nav-item",
                                    a {
                                        class: "nav-link active",
                                        href: "#",
                                        "My Articles"
                                    }
                                }
                                li { class: "nav-item",
                                    a { class: "nav-link", href: "#", "Favorited Articles" }
                                }
                            }
                        }

                        // TODO: 这里应该加载真实的文章列表
                        // 文章列表示例，这里硬编码示例数据
                        div { class: "article-preview",
                            div { class: "article-meta",
                                a { href: "/profile/{profile.username}",
                                    img {class: "user-img",
                                         src: profile.image.as_deref().unwrap_or("") }
                                }
                                div { class: "info",
                                    a {
                                        class: "author",
                                        href: "/profile/{profile.username}",
                                        "{profile.username}"
                                    }
                                    span { class: "date", "January 20th" }
                                }
                                button { class: "btn btn-outline-primary btn-sm pull-xs-right",
                                    i { class: "ion-heart" }
                                    " 29 "
                                }
                            }
                            a {
                                class: "preview-link",
                                href: "/article/how-to-build-webapps-that-scale",
                                h1 { "How to build webapps that scale" }
                                p { "This is the description for the post." }
                                span { "Read more..." }
                                ul { class: "tag-list",
                                    li { class: "tag-default tag-pill tag-outline", "realworld" }
                                    li { class: "tag-default tag-pill tag-outline", "implementations" }
                                }
                            }
                        }

                        div { class: "article-preview",
                            div { class: "article-meta",
                                a { href: "/profile/albert-pai",
                                    img { src: "http://i.imgur.com/N4VcUeJ.jpg" }
                                }
                                div { class: "info",
                                    a {
                                        class: "author",
                                        href: "/profile/albert-pai",
                                        "Albert Pai"
                                    }
                                    span { class: "date", "January 20th" }
                                }
                                button { class: "btn btn-outline-primary btn-sm pull-xs-right",
                                    i { class: "ion-heart" }
                                    " 32 "
                                }
                            }
                            a {
                                class: "preview-link",
                                href: "/article/the-song-you",
                                h1 { "The song you won't ever stop singing. No matter how hard you try." }
                                p { "This is the description for the post." }
                                span { "Read more..." }
                                ul { class: "tag-list",
                                    li { class: "tag-default tag-pill tag-outline", "Music" }
                                    li { class: "tag-default tag-pill tag-outline", "Song" }
                                }
                            }
                        }

                        ul { class: "pagination",
                            li { class: "page-item active",
                                a { class: "page-link", href: "#", "1" }
                            }
                            li { class: "page-item",
                                a { class: "page-link", href: "#", "2" }
                            }
                        }
                    }
                }
            }
        }
    }
}