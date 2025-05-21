use serde::Deserialize;
use dioxus::prelude::*;
use reqwest;
// 导入之前定义的 Article 和 Author 结构，以及 ArticlesResponse
// 假设它们在你的 models.rs 文件中
// use crate::models::{Article, Author, ArticlesResponse}; // 如果你将它们放在 models.rs

// 如果你暂时就定义在当前文件，那就不用上面的 use 语句，直接使用下面的定义
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Article {
    pub slug: String,
    pub title: String,
    pub description: String,
    #[serde(rename = "tagList")] // 匹配 JSON 字段名
    pub tag_list: Vec<String>,
    #[serde(rename = "createdAt")] // 匹配 JSON 字段名
    pub created_at: String,
    #[serde(rename = "updatedAt")] // 匹配 JSON 字段名
    pub updated_at: String,
    #[serde(rename = "favorited")] // 匹配 JSON 字段名
    pub is_favorited: bool,
    #[serde(rename = "favoritesCount")] // 匹配 JSON 字段名
    pub favorites_count: u32,
    pub author: Author,
}

// RealWorld API 的 Author 结构，注意 bio 可能为 null，用 Option<String> 更安全
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Author {
    pub username: String,
    // pub bio: String, // 如果 API 返回 null 会panic，建议改为 Option<String>
    pub bio: Option<String>, // 建议改为 Option<String>
    pub image: String,
    #[serde(rename = "following")] // 匹配 JSON 字段名
    pub is_following: bool,
}

// RealWorld API 获取文章列表的响应格式
#[derive(Debug, Deserialize, Clone)] // Added Clone
pub struct ArticlesResponse {
    pub articles: Vec<Article>,
    #[serde(rename = "articlesCount")] // 匹配 JSON 字段名
    pub articles_count: usize, // 或 u32
}


#[component]
pub fn ArticleList() -> Element {
    // 使用 use_resource 异步加载文章数据
    // 闭包返回 Option<Vec<Article>>，所以 resource 最终的状态值是 ReadOnlySignal<Option<Option<Vec<Article>>>>
    let articles = use_resource(|| async {
        let resp = reqwest::get("http://localhost:8000/api/articles?limit=10&offset=0")
            .await
            .ok()? // 处理 get/await 错误 -> Some(None)
            .json::<ArticlesResponse>()
            .await
            .ok()?; // 处理 json/await 错误 -> Some(None)
        Some(resp.articles) // 成功时返回 Some(Vec<Article>) -> Some(Some(Vec<Article>))
    });

    // 根据加载状态渲染不同内容
    // !! 修复点 !!
    // articles.value() 返回 ReadOnlySignal，需要调用它来获取实际值
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