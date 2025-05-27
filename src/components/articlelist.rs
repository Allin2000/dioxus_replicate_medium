// src/components/articlelist.rs
use dioxus::prelude::*;
use crate::api::article::{fetch_articles};

#[derive(PartialEq, Props, Clone)]
pub struct ArticleListProps {
    feed_type: String,
}



#[component]
// 接收一个布尔值 prop，用于判断显示哪种 Feed
pub fn ArticleList(props: ArticleListProps) -> Element {
 
    let feed_type = &props.feed_type;
    // 你可以根据 feed_type 执行不同逻辑
    let _ = feed_type;
   
    let articles = use_resource(|| async {
            // 这里是你的原有 API 调用逻辑
        let resp = fetch_articles(Default::default()).await?; // Default::default() 是一个很好的默认查询
        Some(resp.articles)
    }); // FIX: 移除这里的分号，让 use_resource 表达式成为 else 块的返回值

    match articles.value()() { // <-- 将 .read() 修改为 .value()()
        // 数据加载完成，成功获取 Some(Some(articles))
        Some(Some(articles)) if articles.is_empty() => {
        rsx!(p { "No articles are here... yet." })
        },
        Some(Some(articles)) => rsx! {
            // 遍历文章列表，渲染每个文章预览
            // !! 修复点 !! 为外层循环添加 key
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
                        // TODO: 根据 is_favorited 属性添加 btn-primary 类并处理点击事件
                        button { class: "btn btn-outline-primary btn-sm pull-xs-right",
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
                            // !! 修复点 !! 为内层循环添加 key
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
        // 数据加载失败或 use_resource 闭包返回 None
        Some(None) => rsx! { p { "Failed to load articles." } },
        // resource 仍在加载中或未开始
        None => rsx! { p { "Loading..." } },
    }
}






