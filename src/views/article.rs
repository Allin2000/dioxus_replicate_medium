// // src/views/article.rs
// use dioxus::prelude::*;

// #[component]
// pub fn Article() -> Element {
//     rsx! {
//         div { class: "article-page",
//         div { class: "banner",
//             div { class: "container",
//                 h1 { "How to build webapps that scale" }
//                 div { class: "article-meta",
//                     a { href: "/profile/eric-simons",
//                         img { src: "http://i.imgur.com/Qr71crq.jpg" }
//                     }
//                     div { class: "info",
//                         a {
//                             class: "author",
//                             href: "/profile/eric-simons",
//                             "Eric Simons"
//                         }
//                         span { class: "date", "January 20th" }
//                     }
//                     button { class: "btn btn-sm btn-outline-secondary",
//                         i { class: "ion-plus-round" }
//                         "\n            \u{a0} Follow Eric Simons "
//                         span { class: "counter", "(10)" }
//                     }
//                     "\n          \u{a0}\u{a0}\n          "
//                     button { class: "btn btn-sm btn-outline-primary",
//                         i { class: "ion-heart" }
//                         "\n            \u{a0} Favorite Post "
//                         span { class: "counter", "(29)" }
//                     }
//                     button { class: "btn btn-sm btn-outline-secondary",
//                         i { class: "ion-edit" }
//                         " Edit Article\n          "
//                     }
//                     button { class: "btn btn-sm btn-outline-danger",
//                         i { class: "ion-trash-a" }
//                         " Delete Article\n          "
//                     }
//                 }
//             }
//         }
//         div { class: "container page",
//             div { class: "row article-content",
//                 div { class: "col-md-12",
//                     p {
//                         "\n            Web development technologies have evolved at an incredible clip over the past few years.\n          "
//                     }
//                     h2 { id: "introducing-ionic", "Introducing RealWorld." }
//                     p { "It's a great solution for learning how other frameworks work." }
//                     ul { class: "tag-list",
//                         li { class: "tag-default tag-pill tag-outline", "realworld" }
//                         li { class: "tag-default tag-pill tag-outline", "implementations" }
//                     }
//                 }
//             }
//             hr {}
//             div { class: "article-actions",
//                 div { class: "article-meta",
//                     a { href: "profile.html",
//                         img { src: "http://i.imgur.com/Qr71crq.jpg" }
//                     }
//                     div { class: "info",
//                         a { class: "author", href: "", "Eric Simons" }
//                         span { class: "date", "January 20th" }
//                     }
//                     button { class: "btn btn-sm btn-outline-secondary",
//                         i { class: "ion-plus-round" }
//                         "\n            \u{a0} Follow Eric Simons\n          "
//                     }
//                     "\n          \u{a0}\n          "
//                     button { class: "btn btn-sm btn-outline-primary",
//                         i { class: "ion-heart" }
//                         "\n            \u{a0} Favorite Article "
//                         span { class: "counter", "(29)" }
//                     }
//                     button { class: "btn btn-sm btn-outline-secondary",
//                         i { class: "ion-edit" }
//                         " Edit Article\n          "
//                     }
//                     button { class: "btn btn-sm btn-outline-danger",
//                         i { class: "ion-trash-a" }
//                         " Delete Article\n          "
//                     }
//                 }
//             }
//             div { class: "row",
//                 div { class: "col-xs-12 col-md-8 offset-md-2",
//                     form { class: "card comment-form",
//                         div { class: "card-block",
//                             textarea {
//                                 class: "form-control",
//                                 placeholder: "Write a comment...",
//                                 rows: "3",
//                             }
//                         }
//                         div { class: "card-footer",
//                             img {
//                                 class: "comment-author-img",
//                                 src: "http://i.imgur.com/Qr71crq.jpg",
//                             }
//                             button { class: "btn btn-sm btn-primary", "Post Comment" }
//                         }
//                     }
//                     div { class: "card",
//                         div { class: "card-block",
//                             p { class: "card-text",
//                                 "\n                With supporting text below as a natural lead-in to additional content.\n              "
//                             }
//                         }
//                         div { class: "card-footer",
//                             a {
//                                 class: "comment-author",
//                                 href: "/profile/author",
//                                 img {
//                                     class: "comment-author-img",
//                                     src: "http://i.imgur.com/Qr71crq.jpg",
//                                 }
//                             }
//                             "\n              \u{a0}\n              "
//                             a {
//                                 class: "comment-author",
//                                 href: "/profile/jacob-schmidt",
//                                 "Jacob Schmidt"
//                             }
//                             span { class: "date-posted", "Dec 29th" }
//                         }
//                     }
//                     div { class: "card",
//                         div { class: "card-block",
//                             p { class: "card-text",
//                                 "\n                With supporting text below as a natural lead-in to additional content.\n              "
//                             }
//                         }
//                         div { class: "card-footer",
//                             a {
//                                 class: "comment-author",
//                                 href: "/profile/author",
//                                 img {
//                                     class: "comment-author-img",
//                                     src: "http://i.imgur.com/Qr71crq.jpg",
//                                 }
//                             }
//                             "\n              \u{a0}\n              "
//                             a {
//                                 class: "comment-author",
//                                 href: "/profile/jacob-schmidt",
//                                 "Jacob Schmidt"
//                             }
//                             span { class: "date-posted", "Dec 29th" }
//                             span { class: "mod-options",
//                                 i { class: "ion-trash-a" }
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }
// }


// use dioxus::prelude::*;
// use crate::Route;
// use crate::api::article::{
//     fetch_article_by_slug, favorite_article, unfavorite_article,
//     delete_article, update_article,
// Article as ArticleData};
// use crate::stores::app_state::{AppState, AuthStatus};
// use crate::api::profile::{follow_user, unfollow_user};




// #[component]
// pub fn Article(slug: String) -> Element {
//     let app_state = use_context::<Signal<AppState>>();
//     let current_route = use_route();

//     // 获取 slug
//     let slug = match current_route {
//         Route::Article { slug } => Some(slug),
//         _ => None,
//     };

//     // 请求文章详情
//     let article_resource = use_resource(move || {
//         let slug = slug.clone();
//         let user_token = app_state.read().user.read().as_ref().map(|user| user.token.clone());
//         async move {
//             if let Some(s) = slug {
//                 fetch_article_by_slug(&s, user_token).await
//             } else {
//                 None
//             }
//         }
//     });

//     let article = match article_resource.value()() {
//         Some(Some(response)) => Some(response.article),
//         _ => None,
//     };

//     let logged_in_username = app_state.read().user.read().as_ref().map(|u| u.username.clone());

//     if article.is_none() {
//         return rsx! {
//             div { class: "article-page",
//                 div { class: "container", "正在加载文章..." }
//             }
//         };
//     }

//     let article = article.unwrap();
//     let is_owner = Some(article.author.username.clone()) == logged_in_username;



//     rsx! {
//         div { class: "article-page",
//             div { class: "banner",
//                 div { class: "container",
//                     h1 { "{article.title}" }
//                     div { class: "article-meta",
//                         a { href: "/profile/{article.author.username}",
//                             img { src: "{article.author.image}" }
//                         }
//                         div { class: "info",
//                             a {
//                                 class: "author",
//                                 href: "/profile/{article.author.username}",
//                                 "{article.author.username}"
//                             }
//                             span { class: "date", "{article.created_at}" }
//                         }
//                         button { class: "btn btn-sm btn-outline-secondary",
//                             i { class: "ion-plus-round" }
//                             " Follow {article.author.username}"
//                             span { class: "counter", " (10)" }
//                         }
//                         button { class: "btn btn-sm btn-outline-primary",
//                             i { class: "ion-heart" }
//                             " Favorite Post"
//                             span { class: "counter", " ({article.favorites_count})" }
//                         }
//                         // {edit_and_delete_buttons}
//                     }
//                 }
//             }

//             div { class: "container page",
//                 div { class: "row article-content",
//                     div { class: "col-md-12",
//                         p { "{article.body}" }
//                         ul { class: "tag-list",
//                             for tag in article.tag_list.iter() {
//                                 li {
//                                     class: "tag-default tag-pill tag-outline",
//                                     key: "{tag}", // Add a key for better performance and state management
//                                     "{tag}"
//                                 }
//                             }
//                         }
//                     }
//                 }

//                 hr {}

//                 div { class: "article-actions",
//                     div { class: "article-meta",
//                         a { href: "/profile/{article.author.username}",
//                             img { src: "{article.author.image}" }
//                         }
//                         div { class: "info",
//                             a {
//                                 class: "author",
//                                 href: "/profile/{article.author.username}",
//                                 "{article.author.username}"
//                             }
//                             span { class: "date", "{article.created_at}" }
//                         }
//                         button { class: "btn btn-sm btn-outline-secondary",
//                             i { class: "ion-plus-round" }
//                             " Follow {article.author.username}"
//                         }
//                         button { class: "btn btn-sm btn-outline-primary",
//                             i { class: "ion-heart" }
//                             " Favorite Article"
//                             span { class: "counter", " ({article.favorites_count})" }
//                         }
//                         // {edit_and_delete_buttons}
//                     }
//                 }

//                 div { class: "row",
//                     div { class: "col-xs-12 col-md-8 offset-md-2",
//                         form { class: "card comment-form",
//                             div { class: "card-block",
//                                 textarea {
//                                     class: "form-control",
//                                     placeholder: "Write a comment...",
//                                     rows: "3",
//                                 }
//                             }
//                             div { class: "card-footer",
//                                 img {
//                                     class: "comment-author-img",
//                                     src: "http://i.imgur.com/Qr71crq.jpg"
//                                 }
//                                 button { class: "btn btn-sm btn-primary", "Post Comment" }
//                             }
//                         }

//                         // 示例评论
//                         div { class: "card",
//                             div { class: "card-block",
//                                 p { class: "card-text",
//                                     "With supporting text below as a natural lead-in to additional content."
//                                 }
//                             }
//                             div { class: "card-footer",
//                                 a {
//                                     class: "comment-author",
//                                     href: "/profile/jacob-schmidt",
//                                     img {
//                                         class: "comment-author-img",
//                                         src: "http://i.imgur.com/Qr71crq.jpg"
//                                     }
//                                 }
//                                 a {
//                                     class: "comment-author",
//                                     href: "/profile/jacob-schmidt",
//                                     "Jacob Schmidt"
//                                 }
//                                 span { class: "date-posted", "Dec 29th" }
//                                 span { class: "mod-options",
//                                     i { class: "ion-trash-a" }
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }






use dioxus::prelude::*;
use crate::Route;
use crate::api::article::{
    fetch_article_by_slug, favorite_article, unfavorite_article,
    delete_article, // update_article, // update_article is not used in Article component
    Article as ArticleData, Author as ArticleAuthor // Alias Author to avoid conflict
};
use crate::stores::app_state::{AppState, AuthStatus};
use crate::api::profile::{follow_user, unfollow_user, Profile}; // Import Profile

#[component]
pub fn Article(slug: String) -> Element {
    // Context, navigator, user state, etc.
    let app_state = use_context::<Signal<AppState>>();
    let navigator = use_navigator();
    let article_state = use_signal(|| None::<ArticleData>);

    // Handle favoriting/unfavoriting
    let toggle_favorite = {
        let mut article_state = article_state.clone();
        let token = app_state.read().user.read().as_ref().map(|u| u.token.clone());
        move |_| {
            let article = article_state.read();
            if let Some(article) = article.as_ref() {
                let slug = article.slug.clone();
                let is_favorited = article.favorited;
                let token = token.clone();

                spawn(async move {
                    if let Some(token) = token {
                        let updated = if is_favorited {
                            unfavorite_article(&token, &slug).await
                        } else {
                            favorite_article(&token, &slug).await
                        };
                        if let Some(new_article) = updated {
                            article_state.set(Some(new_article.article));
                        }
                    }
                });
            }
        }
    };

    // Handle follow/unfollow
    let toggle_follow = {
        let mut article_state = article_state.clone();
        let token = app_state.read().user.read().as_ref().map(|u| u.token.clone());
        move |_| {
            let article = article_state.read();
            if let Some(article) = article.as_ref() {
                let username = article.author.username.clone();
                let is_following = article.author.following;
                let token = token.clone();

let mut updated_article = article.clone();

    spawn(async move {
        if let Some(token) = token {
            let updated_profile_response = if is_following {
                unfollow_user(&username, &token).await
            } else {
                follow_user(&username, &token).await
            };

            if let Some(new_profile_response) = updated_profile_response {
                let new_profile: Profile = new_profile_response.profile;

                // 构建新的 ArticleAuthor
                let new_article_author = ArticleAuthor {
                    username: new_profile.username,
                    bio: new_profile.bio,
                    image: new_profile.image.unwrap_or_default(),
                    following: new_profile.following,
                };

                updated_article.author = new_article_author;

                // ✅ 注意这里必须再克隆一份 state
                article_state.set(Some(updated_article));
            }
                    }
                });
            }
        }
    };

    // Delete article
    let delete_this_article = {
        let navigator = navigator.clone();
        let token = app_state.read().user.read().as_ref().map(|u| u.token.clone());
        let slug = slug.clone();
        move |_| {
            let token = token.clone();
            let slug = slug.clone();
            spawn(async move {
                if let Some(token) = token {
                    if delete_article(&token, &slug).await {
                        navigator.replace(Route::GlobalFeed {});
                    }
                }
            });
        }
    };

    // Edit article
    let edit_article = {
        let navigator = navigator.clone();
        let slug = slug.clone();
        move |_| {
            navigator.push(Route::Edit { slug: slug.clone() });
        }
    };

    // Load article details
    use_effect(move || {
        let token = app_state.read().user.read().as_ref().map(|u| u.token.clone());
        let slug = slug.clone();
        let mut article_state = article_state.clone();

        spawn(async move {
            if let Some(data) = fetch_article_by_slug(&slug, token).await {
                article_state.set(Some(data.article));
            }
        });
    });

    // UI rendering part
    let article = match article_state() {
        Some(article) => article,
        None => return rsx!(div { "Loading..." }), // Ensure this is a complete and valid rsx! block
    };

    let is_owner = Some(article.author.username.clone()) == app_state.read().user.read().as_ref().map(|u| u.username.clone());

rsx! {
    div {
        class: "article-page",

        // Article banner
        div {
            class: "banner",
            div {
                class: "container",
                h1 { "{article.title}" }
                div {
                    class: "article-meta",
                    img { src: "{article.author.image}" }
                    div {
                        class: "info",
                        a {
                            href: "#",
                            class: "author",
                            "{article.author.username}"
                        }
                        span { class: "date", "{article.created_at}" }
                    }
                    if is_owner {
                        button {
                            class: "btn btn-outline-secondary btn-sm",
                            onclick: edit_article,
                            i { class: "ion-edit" }
                            " Edit Article"
                        }
                        button {
                            class: "btn btn-outline-danger btn-sm",
                            onclick: delete_this_article,
                            i { class: "ion-trash-a" }
                            " Delete Article"
                        }
                    } else {
                        button {
                            class: "btn btn-sm action-btn btn-outline-secondary",
                            onclick: toggle_follow,
                            i { class: "ion-plus-round" }
                            if article.author.following {
                                " Unfollow {article.author.username}"
                            } else {
                                " Follow {article.author.username}"
                            }
                        }
                        button {
                            class: "btn btn-sm btn-outline-primary",
                            onclick: toggle_favorite,
                            i { class: "ion-heart" }
                            if article.favorited {
                                " Unfavorite Article ({article.favorites_count})"
                            } else {
                                " Favorite Article ({article.favorites_count})"
                            }
                        }
                    }
                }
            }
        }

            div { class: "container page",
                div { class: "row article-content",
                    div { class: "col-md-12",
                        p { "{article.body}" }
                        ul { class: "tag-list",
                            for tag in article.tag_list.iter() {
                                li { 
                                    class: "tag-default tag-pill tag-outline",
                                    key: "{tag}",
                                    "{tag}" 
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
