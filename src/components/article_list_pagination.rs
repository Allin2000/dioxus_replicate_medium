// src/components/article_list_with_pagination.rs
use dioxus::prelude::*;
use crate::api::article::{fetch_articles, ArticleQuery};
use crate::components::pagination::Pagination;

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
    let mut current_page = use_signal(|| 1usize);
    let mut total_count = use_signal(|| 0usize);

    let articles = use_resource(move || {
        let tag = props.tag.clone();
        let author = props.author.clone();
        let favorited = props.favorited.clone();
        let feed_type = props.feed_type.clone();
        let page_size = props.page_size;
        let current_page_val = current_page();
        
        async move {
            // 构建查询参数
            let query = ArticleQuery {
                tag,
                author,
                favorited,
                limit: Some(page_size as u32),
                offset: Some(((current_page_val - 1) * page_size) as u32),
            };

            match feed_type.as_str() {
                "your" => {
                    // TODO: 实现获取用户关注的文章
                    // fetch_your_feed(query).await
                    None
                }
                _ => {
                    // 全局文章或特定查询
                    match fetch_articles(query).await {
                        Some(resp) => {
                            total_count.set(resp.articles_count);
                            Some(resp.articles)
                        }
                        None => None,
                    }
                }
            }
        }
    });

    let handle_page_change = move |page: usize| {
        current_page.set(page);
    };

    rsx! {
        div {
            // 文章列表
            match articles.value()() {
                Some(Some(articles)) if articles.is_empty() => {
                    rsx! { 
                        div { class: "article-preview",
                            "No articles are here... yet."
                        }
                    }
                },
                Some(Some(articles)) => rsx! {
                    for article in articles.iter() {
                        ArticlePreview { 
                            key: "{article.slug}",
                            article: article.clone()
                        }
                    }
                },
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
    pub article: crate::api::article::Article, // 根据您的实际类型调整
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
                    class: format!("btn btn-sm pull-xs-right {}", 
                        if article.favorited { "btn-primary" } else { "btn-outline-primary" }
                    ),
                    onclick: move |_| {
                        // TODO: 处理点赞/取消点赞
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