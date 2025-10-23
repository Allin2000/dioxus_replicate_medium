// src/components/articlelist.rs
use dioxus::prelude::*;
use crate::api::article::{fetch_articles, fetch_user_feed_articles, favorite_article, unfavorite_article,ArticleQuery};
use crate::stores::app_state::{AppState, User};

#[derive(PartialEq, Props, Clone)]
pub struct ArticleListProps {
    feed_type: String,
}

#[component]
pub fn ArticleList(props: ArticleListProps) -> Element {
    let app_state = use_context::<Signal<AppState>>();
    let token = app_state.read().user.read().as_ref().map(|u| u.token.clone());

    // 使用本地状态存储文章列表，以便更新收藏状态
    let mut articles_state = use_signal(|| None::<Vec<crate::api::article::Article>>);

    // ✅ 关键修复：使用 use_reactive! 宏包装 feed_type 依赖
    let feed_type = props.feed_type.clone();
    let token_clone = token.clone();
    
    let articles = use_resource(use_reactive!(|(feed_type, token_clone)| async move {
        // 解析 feed_type 以获取标签信息
        if feed_type.starts_with("tag:") {
            let tag = feed_type.strip_prefix("tag:").unwrap_or("");
            // 构建请求参数，包含标签过滤
            let mut params = crate::api::article::ArticleQuery::default();
            params.tag = Some(tag.to_string());
            fetch_articles(params).await.map(|r| r.articles)
        } else {
            match feed_type.as_str() {
                "your" if token_clone.is_some() => {
                    fetch_user_feed_articles(&token_clone.unwrap()).await.map(|r| r.articles)
                },
                _ => {
                    fetch_articles(Default::default()).await.map(|r| r.articles)
                }
            }
        }
    }));

    // 当资源加载完成时，更新本地状态
    use_effect(move || {
        if let Some(Some(articles)) = articles.value()() {
            articles_state.set(Some(articles));
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

    // 使用本地状态或资源状态
    let articles_to_render = articles_state.read().clone().or_else(|| {
        articles.value()().flatten()
    });

    match articles_to_render {
        // 数据加载完成，成功获取文章列表
        Some(articles) if articles.is_empty() => {
            rsx!(p { "No articles are here... yet." })
        },
        Some(articles) => rsx! {
            // 遍历文章列表，渲染每个文章预览
            for article in articles.iter() {
                div { class: "article-preview",
                    key: "{article.slug}", // 使用文章 slug 作为 key
                    div { class: "article-meta",
                        a { href: format!("/profile/{}", article.author.username),
                            img { src: "{article.author.image}" }
                        }
                        div { class: "info",
                            a { class: "author", href: format!("/profile/{}", article.author.username),
                                "{article.author.username}"
                            }
                            // TODO: 格式化日期
                            span { class: "date", "{article.created_at}" }
                        }
                        // 添加收藏/取消收藏功能
                        button { 
                            class: if article.favorited {
                                "btn btn-primary btn-sm pull-xs-right"
                            } else {
                                "btn btn-outline-primary btn-sm pull-xs-right"
                            },
                            onclick: {
                                let slug = article.slug.clone();
                                let toggle_favorite = toggle_favorite.clone();
                                move |_| toggle_favorite(slug.clone())
                            },
                            i { class: "ion-heart" }
                            " {article.favorites_count} "
                        }
                    }
                    // TODO: 链接到文章详情页
                    a { class: "preview-link", href: format!("/article/{}", article.slug),
                        h1 { "{article.title}" }
                        p { "{article.description}" }
                        span { "Read more..." }
                        ul { class: "tag-list",
                            // 遍历标签列表，渲染每个标签
                            for tag in article.tag_list.iter() { // 使用 tag_list 匹配结构体字段名
                                li { class: "tag-default tag-pill tag-outline",
                                    key: "{tag}", // 使用标签字符串作为 key
                                    "{tag}"
                                }
                            }
                        }
                    }
                }
            }
        },
        // 数据加载失败或文章列表为空
        None => {
            match articles.value()() {
                Some(None) => rsx! { p { "Failed to load articles." } },
                None => rsx! { p { "Loading..." } },
                _ => rsx! { p { "No articles available." } }
            }
        }
    }
}