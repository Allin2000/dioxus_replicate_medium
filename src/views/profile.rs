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



// // src/views/profile.rs
// use dioxus::prelude::*;
// use crate::Route;
// use crate::api::profile::{fetch_profile, Profile as ProfileData};
// use crate::stores::app_state::{AppState, AuthStatus};

// #[component]
// pub fn Profile(username: String) -> Element {
//     let app_state = use_context::<Signal<AppState>>();
//     let current_route = use_route();

//     // 获取 username
//     let username = match current_route {
//         Route::Profile { username } => Some(username),
//         _ => None,
//     };

//     // 请求用户资料
//     let profile_resource = use_resource(move || {
//         let username = username.clone();
//         let user_token = app_state.read().user.read().as_ref().map(|user| user.token.clone());
//         async move {
//             if let Some(u) = username {
//                 fetch_profile(&u, user_token.as_deref()).await
//             } else {
//                 None
//             }
//         }
//     });

//     let profile = match profile_resource.value()() {
//         Some(Some(response)) => Some(response.profile),
//         _ => None,
//     };

//     let logged_in_username = app_state.read().user.read().as_ref().map(|u| u.username.clone());

//     if profile.is_none() {
//         return rsx! {
//             div { class: "profile-page",
//                 div { class: "container", "正在加载用户资料..." }
//             }
//         };
//     }

//     let profile = profile.unwrap();
//     let is_own_profile = Some(profile.username.clone()) == logged_in_username;


//     rsx! {
//         div { class: "profile-page",
//             div { class: "user-info",
//                 div { class: "container",
//                     div { class: "row",
//                         div { class: "col-xs-12 col-md-10 offset-md-1",
//                             img {
//                                 class: "user-img",
//                                 src: profile.image.as_deref().unwrap_or(""),
//                             }
//                             h4 { "{profile.username}" }
//                             if let Some(bio) = &profile.bio {
//                                 p { "{bio}" }
//                             }

//                             if is_own_profile {
//                                 // 如果是自己的资料页，显示编辑按钮
//                                 button { class: "btn btn-sm btn-outline-secondary action-btn",
//                                     i { class: "ion-gear-a" }
//                                     " Edit Profile Settings"
//                                 }
//                             } else {
//                                 // 如果不是自己的资料页，显示关注/取消关注按钮
//                                 button {
//                                     class: "btn btn-sm btn-outline-secondary action-btn",
//                                     // onclick: on_follow_click,
//                                     i {
//                                         class: if profile.following { "ion-minus-round" } else { "ion-plus-round" }
//                                     }
//                                     if profile.following {
//                                         " Unfollow {profile.username}"
//                                     } else {
//                                         " Follow {profile.username}"
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }

//             div { class: "container",
//                 div { class: "row",
//                     div { class: "col-xs-12 col-md-10 offset-md-1",
//                         div { class: "articles-toggle",
//                             ul { class: "nav nav-pills outline-active",
//                                 li { class: "nav-item",
//                                     a {
//                                         class: "nav-link active",
//                                         href: "#",
//                                         "My Articles"
//                                     }
//                                 }
//                                 li { class: "nav-item",
//                                     a { class: "nav-link", href: "#", "Favorited Articles" }
//                                 }
//                             }
//                         }

//                         // TODO: 这里应该加载真实的文章列表
//                         // 文章列表示例，这里硬编码示例数据
//                         div { class: "article-preview",
//                             div { class: "article-meta",
//                                 a { href: "/profile/{profile.username}",
//                                     img {class: "user-img",
//                                          src: profile.image.as_deref().unwrap_or("") }
//                                 }
//                                 div { class: "info",
//                                     a {
//                                         class: "author",
//                                         href: "/profile/{profile.username}",
//                                         "{profile.username}"
//                                     }
//                                     span { class: "date", "January 20th" }
//                                 }
//                                 button { class: "btn btn-outline-primary btn-sm pull-xs-right",
//                                     i { class: "ion-heart" }
//                                     " 29 "
//                                 }
//                             }
//                             a {
//                                 class: "preview-link",
//                                 href: "/article/how-to-build-webapps-that-scale",
//                                 h1 { "How to build webapps that scale" }
//                                 p { "This is the description for the post." }
//                                 span { "Read more..." }
//                                 ul { class: "tag-list",
//                                     li { class: "tag-default tag-pill tag-outline", "realworld" }
//                                     li { class: "tag-default tag-pill tag-outline", "implementations" }
//                                 }
//                             }
//                         }

//                         div { class: "article-preview",
//                             div { class: "article-meta",
//                                 a { href: "/profile/albert-pai",
//                                     img { src: "http://i.imgur.com/N4VcUeJ.jpg" }
//                                 }
//                                 div { class: "info",
//                                     a {
//                                         class: "author",
//                                         href: "/profile/albert-pai",
//                                         "Albert Pai"
//                                     }
//                                     span { class: "date", "January 20th" }
//                                 }
//                                 button { class: "btn btn-outline-primary btn-sm pull-xs-right",
//                                     i { class: "ion-heart" }
//                                     " 32 "
//                                 }
//                             }
//                             a {
//                                 class: "preview-link",
//                                 href: "/article/the-song-you",
//                                 h1 { "The song you won't ever stop singing. No matter how hard you try." }
//                                 p { "This is the description for the post." }
//                                 span { "Read more..." }
//                                 ul { class: "tag-list",
//                                     li { class: "tag-default tag-pill tag-outline", "Music" }
//                                     li { class: "tag-default tag-pill tag-outline", "Song" }
//                                 }
//                             }
//                         }

//                         ul { class: "pagination",
//                             li { class: "page-item active",
//                                 a { class: "page-link", href: "#", "1" }
//                             }
//                             li { class: "page-item",
//                                 a { class: "page-link", href: "#", "2" }
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }


// src/views/profile.rs
use dioxus::prelude::*;
use crate::Route;
use crate::api::profile::{fetch_profile, follow_user, unfollow_user, Profile as ProfileData};
use crate::stores::app_state::AppState;
use crate::api::article::{fetch_articles, favorite_article, unfavorite_article, ArticleQuery};

#[component]
pub fn Profile(username: String) -> Element {
    let app_state = use_context::<Signal<AppState>>();
    let current_route = use_route();

    let username = match current_route {
        Route::Profile { username } => Some(username),
        _ => None,
    };

    let profile_state = use_signal(|| None::<ProfileData>);
    let mut tab_state = use_signal(|| "my_articles".to_string());

    // 加载 profile 信息
    use_effect({
        let username = username.clone();
        let mut profile_state = profile_state.clone();
        let app_state = app_state.clone();

        move || {
            let token = app_state.read().user.read().as_ref().map(|u| u.token.clone());
            let username = username.clone();
            spawn(async move {
                if let Some(u) = username {
                    if let Some(response) = fetch_profile(&u, token.as_deref()).await {
                        profile_state.set(Some(response.profile));
                    }
                }
            });
        }
    });

    // 使用 use_resource 加载文章（依赖 profile 和 tab）
    let token = app_state.read().user.read().as_ref().map(|u| u.token.clone());
    let tab = tab_state.read().clone();
    let profile = profile_state.read().clone();

let articles_query = use_signal(|| (tab_state(), profile_state()));

let articles = use_resource({
    let app_state = app_state.clone();
    let articles_query = articles_query.clone();

    move || {
        let (tab, profile_opt) = articles_query();
        let token = app_state.read().user.read().as_ref().map(|u| u.token.clone());

        async move {
            if let Some(profile) = profile_opt {
                let query = ArticleQuery {
                    author: if tab == "my_articles" {
                        Some(profile.username.clone())
                    } else {
                        None
                    },
                    favorited: if tab == "favorited_articles" {
                        Some(profile.username.clone())
                    } else {
                        None
                    },
                    ..Default::default()
                };

                fetch_articles(query).await.map(|res| res.articles)
            } else {
                None
            }
        }
    }
});



use_effect({
    let mut articles_query = articles_query.clone();
    let tab_state = tab_state.clone();
    let profile_state = profile_state.clone();

    move || {
        articles_query.set((tab_state(), profile_state()));
    }
});


    let toggle_follow = {
        let mut profile_state = profile_state.clone();
        let token = app_state.read().user.read().as_ref().map(|u| u.token.clone());
        move |_| {
            let profile = profile_state.read();
            if let Some(profile) = profile.as_ref() {
                let username = profile.username.clone();
                let is_following = profile.following;
                let token = token.clone();
                let mut updated_profile = profile.clone();

                spawn(async move {
                    if let Some(token) = token {
                        let updated_profile_response = if is_following {
                            unfollow_user(&username, &token).await
                        } else {
                            follow_user(&username, &token).await
                        };

                        if let Some(new_profile_response) = updated_profile_response {
                            updated_profile.following = new_profile_response.profile.following;
                            profile_state.set(Some(updated_profile));
                        }
                    }
                });
            }
        }
    };

    // 渲染 UI
    let Some(profile) = profile_state() else {
        return rsx!(div { "Loading..." });
    };

    let logged_in_username = app_state.read().user.read().as_ref().map(|u| u.username.clone());
    let is_own_profile = Some(profile.username.clone()) == logged_in_username;

    rsx! {
        div { class: "profile-page",
            div { class: "user-info",
                div { class: "container",
                    div { class: "row",
                        div { class: "col-xs-12 col-md-10 offset-md-1",
                            img { class: "user-img" }
                            h4 { "{profile.username}" }
                            if let Some(bio) = &profile.bio {
                                p { "{bio}" }
                            }

                            if is_own_profile {
                                button {
                                    class: "btn btn-sm btn-outline-secondary action-btn",
                                    onclick: move |_| {use_navigator().push(Route::Settings {});},
                                    i { class: "ion-gear-a" }
                                    " Edit Profile Settings"
                                }
                            } else {
                                button {
                                    class: "btn btn-sm btn-outline-secondary action-btn",
                                    onclick: toggle_follow,
                                    i { class: "ion-plus-round" }
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
                                        class: if tab_state() == "my_articles" { "nav-link active" } else { "nav-link" },
                                        onclick: move |_| tab_state.set("my_articles".to_string()),
                                        "My Articles"
                                    }
                                }
                                li { class: "nav-item",
                                    a {
                                        class: if tab_state() == "favorited_articles" { "nav-link active" } else { "nav-link" },
                                        onclick: move |_| tab_state.set("favorited_articles".to_string()),
                                        "Favorited Articles"
                                    }
                                }
                            }
                        }

                        match articles.value()() {
                            Some(Some(articles)) if articles.is_empty() => rsx!(p { "No articles are here... yet." }),
                            Some(Some(articles)) => rsx! {
                                for article in articles.iter() {
                                    div { class: "article-preview", key: "{article.slug}",
                                        div { class: "article-meta",
                                            a { href: format!("/profile/{}", article.author.username),
                                                img { src: "{article.author.image}" }
                                            }
                                            div { class: "info",
                                                a { class: "author", href: format!("/profile/{}", article.author.username),
                                                    "{article.author.username}"
                                                }
                                                span { class: "date", "{article.created_at}" }
                                            }
                                            button { class: "btn btn-outline-primary btn-sm pull-xs-right",
                                                i { class: "ion-heart" }
                                                " {article.favorites_count} "
                                            }
                                        }
                                        a { class: "preview-link", href: format!("/article/{}", article.slug),
                                            h1 { "{article.title}" }
                                            p { "{article.description}" }
                                            span { "Read more..." }
                                            ul { class: "tag-list",
                                                for tag in article.tag_list.iter() {
                                                    li { class: "tag-default tag-pill tag-outline", key: "{tag}", "{tag}" }
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                            Some(None) => rsx!(p { "Failed to load articles." }),
                            None => rsx!(p { "Loading..." }),
                        }
                    }
                }
            }
        }
    }
}