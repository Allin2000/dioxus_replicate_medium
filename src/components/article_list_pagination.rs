// // src/components/article_list_with_pagination.rs
// use dioxus::prelude::*;
// use crate::api::article::{fetch_articles, ArticleQuery};
// use crate::components::pagination::Pagination;

// #[derive(PartialEq, Props, Clone)]
// pub struct ArticleListWithPaginationProps {
//     #[props(default = String::from("global"))]
//     pub feed_type: String,
//     #[props(default = None)]
//     pub author: Option<String>,
//     #[props(default = None)]
//     pub favorited: Option<String>,
//     #[props(default = None)]
//     pub tag: Option<String>,
//     #[props(default = 10)]
//     pub page_size: usize,
// }

// #[component]
// pub fn ArticleListWithPagination(props: ArticleListWithPaginationProps) -> Element {
//     let mut current_page = use_signal(|| 1usize);
//     let mut total_count = use_signal(|| 0usize);

//     let articles = use_resource(move || {
//         let tag = props.tag.clone();
//         let author = props.author.clone();
//         let favorited = props.favorited.clone();
//         let feed_type = props.feed_type.clone();
//         let page_size = props.page_size;
//         let current_page_val = current_page();
        
//         async move {
//             // 构建查询参数
//             let query = ArticleQuery {
//                 tag,
//                 author,
//                 favorited,
//                 limit: Some(page_size as u32),
//                 offset: Some(((current_page_val - 1) * page_size) as u32),
//             };

//             match feed_type.as_str() {
//                 "your" => {
//                     // TODO: 实现获取用户关注的文章
//                     // fetch_your_feed(query).await
//                     None
//                 }
//                 _ => {
//                     // 全局文章或特定查询
//                     match fetch_articles(query).await {
//                         Some(resp) => {
//                             total_count.set(resp.articles_count);
//                             Some(resp.articles)
//                         }
//                         None => None,
//                     }
//                 }
//             }
//         }
//     });

//     let handle_page_change = move |page: usize| {
//         current_page.set(page);
//     };

//     rsx! {
//         div {
//             // 文章列表
//             match articles.value()() {
//                 Some(Some(articles)) if articles.is_empty() => {
//                     rsx! { 
//                         div { class: "article-preview",
//                             "No articles are here... yet."
//                         }
//                     }
//                 },
//                 Some(Some(articles)) => rsx! {
//                     for article in articles.iter() {
//                         ArticlePreview { 
//                             key: "{article.slug}",
//                             article: article.clone()
//                         }
//                     }
//                 },
//                 Some(None) => rsx! { 
//                     div { class: "article-preview",
//                         "Failed to load articles."
//                     }
//                 },
//                 None => rsx! { 
//                     div { class: "article-preview",
//                         "Loading articles..."
//                     }
//                 },
//             }

//             // 分页组件
//             Pagination {
//                 current_page: current_page(),
//                 total_count: total_count(),
//                 page_size: props.page_size,
//                 on_page_change: handle_page_change,
//             }
//         }
//     }
// }

// #[derive(PartialEq, Props, Clone)]
// pub struct ArticlePreviewProps {
//     pub article: crate::api::article::Article, // 根据您的实际类型调整
// }

// #[component]
// pub fn ArticlePreview(props: ArticlePreviewProps) -> Element {
//     let article = &props.article;
    
//     rsx! {
//         div { class: "article-preview",
//             div { class: "article-meta",
//                 a { href: format!("/profile/{}", article.author.username),
//                     img { src: "{article.author.image}" }
//                 }
//                 div { class: "info",
//                     a { class: "author", href: format!("/profile/{}", article.author.username),
//                         "{article.author.username}"
//                     }
//                     span { class: "date", "{article.created_at}" }
//                 }
//                 button { 
//                     class: format!("btn btn-sm pull-xs-right {}", 
//                         if article.favorited { "btn-primary" } else { "btn-outline-primary" }
//                     ),
//                     onclick: move |_| {
//                         // TODO: 处理点赞/取消点赞
//                     },
//                     i { class: "ion-heart" }
//                     " {article.favorites_count} "
//                 }
//             }
//             a { class: "preview-link", href: format!("/article/{}", article.slug),
//                 h1 { "{article.title}" }
//                 p { "{article.description}" }
//                 span { "Read more..." }
//                 ul { class: "tag-list",
//                     for tag in article.tag_list.iter() {
//                         li { 
//                             class: "tag-default tag-pill tag-outline",
//                             key: "{tag}",
//                             "{tag}"
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }




// 完善切换feed和点击♥  切换feed还有bug
// src/components/article_list_with_pagination.rs
use dioxus::prelude::*;
use crate::api::article::{fetch_articles, fetch_user_feed_articles, favorite_article, unfavorite_article, ArticleQuery};
use crate::components::pagination::Pagination;
use crate::stores::app_state::{AppState, User};

#[derive(PartialEq, Props, Clone)]
pub struct ArticleListWithPaginationProps {
    #[props(default = String::from("global"))]
    pub feed_type: String,
    #[props(default = None)]
    pub author: Option<String>,
    #[props(default = None)]
    pub favorited: Option<String>,
    #[props(default = None)]
    pub tag: Option<String>,
    #[props(default = 10)]
    pub page_size: usize,
}

#[component]
pub fn ArticleListWithPagination(props: ArticleListWithPaginationProps) -> Element {
    let app_state = use_context::<Signal<AppState>>();
    let token = app_state.read().user.read().as_ref().map(|u| u.token.clone());
    
    let mut current_page = use_signal(|| 1usize);
    let mut total_count = use_signal(|| 0usize);
    
    // 使用本地状态存储文章列表，以便更新收藏状态
    let mut articles_state = use_signal(|| None::<Vec<crate::api::article::Article>>);

    let feed_type = props.feed_type.clone();
    let token_clone = token.clone();



    let articles = use_resource(move || {
        let tag = props.tag.clone();
        let author = props.author.clone();
        let favorited = props.favorited.clone();
        let feed_type = feed_type.clone();
        let token = token_clone.clone();
        let page_size = props.page_size;
        let current_page_val = current_page();
        
        async move {
            match feed_type.as_str() {
                "your" if token.is_some() => {
                    // 获取用户关注的文章 - 注意：这里可能需要支持分页参数
                    fetch_user_feed_articles(&token.unwrap()).await.map(|resp| {
                        // 手动处理分页（如果API不支持分页）
                        let start = ((current_page_val - 1) * page_size).min(resp.articles.len());
                        let end = (current_page_val * page_size).min(resp.articles.len());
                        let articles = resp.articles[start..end].to_vec();
                        (articles, resp.articles.len())
                    })
                }
                _ => {
                    // 构建查询参数
                    let query = ArticleQuery {
                        tag,
                        author,
                        favorited,
                        limit: Some(page_size as u32),
                        offset: Some(((current_page_val - 1) * page_size) as u32),
                    };

                    // 全局文章或特定查询
                    fetch_articles(query).await.map(|resp| {
                        (resp.articles, resp.articles_count)
                    })
                }
            }
        }
    });





    // 当资源加载完成时，更新本地状态和总数
    use_effect(move || {
        if let Some(Some((articles, count))) = articles.value()() {
            articles_state.set(Some(articles));
            total_count.set(count);
        }
    });

    // 处理收藏/取消收藏的逻辑
    let toggle_favorite = {
        let articles_state = articles_state.clone();
        let token = app_state.read().user.read().as_ref().map(|u| u.token.clone());
        
        move |slug: String| {
            let token = token.clone();
            let mut articles_state = articles_state.clone();
            
            spawn(async move {
                if let Some(token) = token {
                    // 先获取当前文章的收藏状态
                    let (is_favorited, slug_clone) = {
                        if let Some(articles) = articles_state.read().as_ref() {
                            if let Some(article) = articles.iter().find(|a| a.slug == slug) {
                                (article.favorited, article.slug.clone())
                            } else {
                                return;
                            }
                        } else {
                            return;
                        }
                    };
                    
                    // 调用 API
                    let updated = if is_favorited {
                        unfavorite_article(&token, &slug_clone).await
                    } else {
                        favorite_article(&token, &slug_clone).await
                    };
                    
                    // 如果 API 调用成功，更新本地状态
                    if let Some(new_article) = updated {
                        let updated_articles = {
                            if let Some(mut articles) = articles_state.read().clone() {
                                if let Some(article) = articles.iter_mut().find(|a| a.slug == slug_clone) {
                                    article.favorited = new_article.article.favorited;
                                    article.favorites_count = new_article.article.favorites_count;
                                }
                                Some(articles)
                            } else {
                                None
                            }
                        };
                        
                        if let Some(articles) = updated_articles {
                            articles_state.set(Some(articles));
                        }
                    }
                }
            });
        }
    };

    let handle_page_change = move |page: usize| {
        current_page.set(page);
    };

    // 使用本地状态或资源状态
    let articles_to_render = articles_state.read().clone().or_else(|| {
        articles.value()().flatten().map(|(articles, _)| articles)
    });

    rsx! {
        div {
            // 文章列表
            match articles_to_render {
                Some(articles) if articles.is_empty() => {
                    rsx! { 
                        div { class: "article-preview",
                            "No articles are here... yet."
                        }
                    }
                },
                Some(articles) => rsx! {
                    for article in articles.iter() {
                        ArticlePreview { 
                            key: "{article.slug}",
                            article: article.clone(),
                            toggle_favorite: toggle_favorite.clone()
                        }
                    }
                },
                None => {
                    match articles.value()() {
                        Some(None) => rsx! { 
                            div { class: "article-preview",
                                "Failed to load articles."
                            }
                        },
                        None => rsx! { 
                            div { class: "article-preview",
                                "Loading articles..."
                            }
                        },
                        _ => rsx! { 
                            div { class: "article-preview",
                                "No articles available."
                            }
                        }
                    }
                }
            }

            // 分页组件
            Pagination {
                current_page: current_page(),
                total_count: total_count(),
                page_size: props.page_size,
                on_page_change: handle_page_change,
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct ArticlePreviewProps {
    pub article: crate::api::article::Article,
    pub toggle_favorite: Callback<String>,
}

#[component]
pub fn ArticlePreview(props: ArticlePreviewProps) -> Element {
    let article = &props.article;
    
    rsx! {
        div { class: "article-preview",
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
                button { 
                    class: if article.favorited {
                        "btn btn-primary btn-sm pull-xs-right"
                    } else {
                        "btn btn-outline-primary btn-sm pull-xs-right"
                    },
                    onclick: {
                        let slug = article.slug.clone();
                        let toggle_favorite = props.toggle_favorite.clone();
                        move |_| toggle_favorite.call(slug.clone())
                    },
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



