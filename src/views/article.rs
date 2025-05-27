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


use dioxus::prelude::*;
use crate::Route;
use crate::api::article::{fetch_article_by_slug, Article as ArticleData};
use crate::stores::app_state::{AppState, AuthStatus};
use chrono::{DateTime, Local};

#[component]
pub fn Article(slug: String) -> Element {
    let app_state = use_context::<Signal<AppState>>();
    let current_route = use_route();

    // 获取 slug
    let slug = match current_route {
        Route::Article { slug } => Some(slug),
        _ => None,
    };

    // 请求文章详情
    let article_resource = use_resource(move || {
        let slug = slug.clone();
        let user_token = app_state.read().user.read().as_ref().map(|user| user.token.clone());
        async move {
            if let Some(s) = slug {
                fetch_article_by_slug(&s, user_token).await
            } else {
                None
            }
        }
    });

    let article = match article_resource.value()() {
        Some(Some(response)) => Some(response.article),
        _ => None,
    };

    let logged_in_username = app_state.read().user.read().as_ref().map(|u| u.username.clone());

    if article.is_none() {
        return rsx! {
            div { class: "article-page",
                div { class: "container", "正在加载文章..." }
            }
        };
    }

    let article = article.unwrap();
    let is_owner = Some(article.author.username.clone()) == logged_in_username;

// let tag_list: Vec<&str> = article.tag_list.iter().map(|tag| tag.as_str()).collect();

    // let edit_and_delete_buttons = if is_owner {
    //     rsx! {
    //         Fragment {
    //             button { class: "btn btn-sm btn-outline-secondary",
    //                 i { class: "ion-edit" }
    //                 " Edit Article"
    //             }
    //             button { class: "btn btn-sm btn-outline-danger",
    //                 i { class: "ion-trash-a" }
    //                 " Delete Article"
    //             }
    //         }
    //     }
    // } else {
    //     rsx! {
    //         Fragment {}
    //     }
    // };

    rsx! {
        div { class: "article-page",
            div { class: "banner",
                div { class: "container",
                    h1 { "{article.title}" }
                    div { class: "article-meta",
                        a { href: "/profile/{article.author.username}",
                            img { src: "{article.author.image}" }
                        }
                        div { class: "info",
                            a {
                                class: "author",
                                href: "/profile/{article.author.username}",
                                "{article.author.username}"
                            }
                            span { class: "date", "{article.created_at}" }
                        }
                        button { class: "btn btn-sm btn-outline-secondary",
                            i { class: "ion-plus-round" }
                            " Follow {article.author.username}"
                            span { class: "counter", " (10)" }
                        }
                        button { class: "btn btn-sm btn-outline-primary",
                            i { class: "ion-heart" }
                            " Favorite Post"
                            span { class: "counter", " ({article.favorites_count})" }
                        }
                        // {edit_and_delete_buttons}
                    }
                }
            }

            div { class: "container page",
                div { class: "row article-content",
                    div { class: "col-md-12",
                        p { "{article.body}" }
                        // ul { class: "tag-list", tag_list}
                    }
                }

                hr {}

                div { class: "article-actions",
                    div { class: "article-meta",
                        a { href: "/profile/{article.author.username}",
                            img { src: "{article.author.image}" }
                        }
                        div { class: "info",
                            a {
                                class: "author",
                                href: "/profile/{article.author.username}",
                                "{article.author.username}"
                            }
                            span { class: "date", "{article.created_at}" }
                        }
                        button { class: "btn btn-sm btn-outline-secondary",
                            i { class: "ion-plus-round" }
                            " Follow {article.author.username}"
                        }
                        button { class: "btn btn-sm btn-outline-primary",
                            i { class: "ion-heart" }
                            " Favorite Article"
                            span { class: "counter", " ({article.favorites_count})" }
                        }
                        // {edit_and_delete_buttons}
                    }
                }

                div { class: "row",
                    div { class: "col-xs-12 col-md-8 offset-md-2",
                        form { class: "card comment-form",
                            div { class: "card-block",
                                textarea {
                                    class: "form-control",
                                    placeholder: "Write a comment...",
                                    rows: "3",
                                }
                            }
                            div { class: "card-footer",
                                img {
                                    class: "comment-author-img",
                                    src: "http://i.imgur.com/Qr71crq.jpg"
                                }
                                button { class: "btn btn-sm btn-primary", "Post Comment" }
                            }
                        }

                        // 示例评论
                        div { class: "card",
                            div { class: "card-block",
                                p { class: "card-text",
                                    "With supporting text below as a natural lead-in to additional content."
                                }
                            }
                            div { class: "card-footer",
                                a {
                                    class: "comment-author",
                                    href: "/profile/jacob-schmidt",
                                    img {
                                        class: "comment-author-img",
                                        src: "http://i.imgur.com/Qr71crq.jpg"
                                    }
                                }
                                a {
                                    class: "comment-author",
                                    href: "/profile/jacob-schmidt",
                                    "Jacob Schmidt"
                                }
                                span { class: "date-posted", "Dec 29th" }
                                span { class: "mod-options",
                                    i { class: "ion-trash-a" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}