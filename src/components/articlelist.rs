// use serde::Deserialize;
// use dioxus::prelude::*;
// use crate::api::article::{fetch_articles, ArticlePreview}; 

// // use app_state::{AppState, AuthStatus}; // <-- Import AuthStatus and AppState
// // use crate::api::article::FeedFilter; // <-- Import FeedFilter (ensure this path is correct)



// #[component]
// pub fn ArticleList() -> Element {

//         let articles = use_resource(|| async {
//         let resp = fetch_articles(Default::default()).await?;
//         Some(resp.articles)
//     });

//     // 根据加载状态渲染不同内容
//     // !! 修复点 !!
//     // articles.value() 返回 ReadOnlySignal，需要调用它来获取实际值
//     match articles.value()() { // <-- 将 .read() 修改为 .value()()
//         // 数据加载完成，成功获取 Some(Some(articles))
//         Some(Some(articles)) if articles.is_empty() => {
//         rsx!(p { "No articles are here... yet." })
//         },
//         Some(Some(articles)) => rsx! {
//             // 遍历文章列表，渲染每个文章预览
//             // !! 修复点 !! 为外层循环添加 key
//             for article in articles.iter() {
//                 div { class: "article-preview",
//                     key: "{article.slug}", // 使用文章 slug 作为 key
//                     div { class: "article-meta",
//                         a { href: format!("/profile/{}", article.author.username),
//                             img { src: "{article.author.image}" }
//                         }
//                         div { class: "info",
//                             a { class: "author", href: format!("/profile/{}", article.author.username),
//                                 "{article.author.username}"
//                             }
//                             // TODO: 格式化日期
//                             span { class: "date", "{article.created_at}" }
//                         }
//                         // TODO: 根据 is_favorited 属性添加 btn-primary 类并处理点击事件
//                         button { class: "btn btn-outline-primary btn-sm pull-xs-right",
//                             i { class: "ion-heart" }
//                             " {article.favorites_count} "
//                         }
//                     }
//                     // TODO: 链接到文章详情页
//                     a { class: "preview-link", href: format!("/article/{}", article.slug),
//                         h1 { "{article.title}" }
//                         p { "{article.description}" }
//                         span { "Read more..." }
//                         ul { class: "tag-list",
//                             // 遍历标签列表，渲染每个标签
//                             // !! 修复点 !! 为内层循环添加 key
//                             for tag in article.tag_list.iter() { // 使用 tag_list 匹配结构体字段名
//                                 li { class: "tag-default tag-pill tag-outline",
//                                     key: "{tag}", // 使用标签字符串作为 key
//                                     "{tag}"
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//         },
//         // 数据加载失败或 use_resource 闭包返回 None
//         Some(None) => rsx! { p { "Failed to load articles." } },
//         // resource 仍在加载中或未开始
//         None => rsx! { p { "Loading..." } },
//     }
// }



    // 使用 use_resource 异步加载文章数据
    // 闭包返回 Option<Vec<Article>>，所以 resource 最终的状态值是 ReadOnlySignal<Option<Option<Vec<Article>>>>
    // let articles = use_resource(|| async {
    //     let resp = reqwest::get("http://localhost:8000/api/articles?limit=10&offset=0")
    //         .await
    //         .ok()? // 处理 get/await 错误 -> Some(None)
    //         .json::<ArticlesResponse>()
    //         .await
    //         .ok()?; // 处理 json/await 错误 -> Some(None)
    //     Some(resp.articles) // 成功时返回 Some(Vec<Article>) -> Some(Some(Vec<Article>))
    // });



// src/components/articlelist.rs
use dioxus::prelude::*;
use reqwest;

// 从 api/models.rs 导入需要的结构体
// 确保你在 src/api/models.rs 中定义了这些结构体
use crate::api::models::{Article, Author, ArticlesResponse};

#[component]
// 接收一个布尔值 prop，用于判断显示哪种 Feed
pub fn ArticleList(show_your_feed: bool) -> Element {
    // 根据 show_your_feed 的值来决定渲染内容
    if show_your_feed {
        // 如果 show_your_feed 为 true，表示显示“你的 Feed”，此时只显示占位符
        rsx!(p { "Your feed is empty. Start following users or publish your first article!" })
    } else {
        // 如果 show_your_feed 为 false，表示显示“全局 Feed”，执行原有的 API 获取逻辑
        let articles = use_resource(|| async {
            // 这里是你的原有 API 调用逻辑
            let resp = reqwest::get("http://localhost:8000/api/articles?limit=10&offset=0")
                .await
                .ok()?
                .json::<ArticlesResponse>()
                .await
                .ok()?;
            Some(resp.articles)
        });

        // 根据 articles 资源的状态渲染
        match articles.value()() {
            // 数据加载完成，并且成功获取到文章列表 (Some(Some(articles)))
            Some(Some(articles)) => {
                if articles.is_empty() {
                    // 如果文章列表为空，显示“没有文章”的消息
                    rsx!(p { "No articles are here... yet." })
                } else {
                    // 如果有文章，遍历并渲染每个文章预览
                    rsx! {
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
                                        span { class: "date", "{article.created_at}" } // TODO: 格式化日期
                                    }
                                    // TODO: 根据 is_favorited 属性添加 btn-primary 类并处理点击事件
                                    button { class: "btn btn-outline-primary btn-sm pull-xs-right",
                                        i { class: "ion-heart" }
                                        " {article.favorites_count} "
                                    }
                                }
                                // TODO: 链接到文章详情页（虽然你现在不实现，但保持链接结构）
                                a { class: "preview-link", href: format!("/article/{}", article.slug),
                                    h1 { "{article.title}" }
                                    p { "{article.description}" }
                                    span { "Read more..." }
                                    ul { class: "tag-list",
                                        for tag in article.tag_list.iter() {
                                            li { class: "tag-default tag-pill tag-outline",
                                                key: "{tag}", // 使用标签字符串作为 key
                                                "{tag}"
                                            }
                                        }
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
}