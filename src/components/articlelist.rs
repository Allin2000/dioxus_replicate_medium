// src/components/articlelist.rs
use dioxus::prelude::*;
use crate::api::article::{fetch_articles};


#[component]
// 接收一个布尔值 prop，用于判断显示哪种 Feed
pub fn ArticleList(has_user_clicked_feed_tab: bool) -> Element {
    // 根据 show_your_feed 的值来决定渲染内容
    if has_user_clicked_feed_tab {
        // 如果 show_your_feed 为 true，表示显示“你的 Feed”，此时只显示占位符
        rsx!(p { "Your feed is empty. Start following users or publish your first article!" })
    } else {
        // 如果 show_your_feed 为 false，表示显示“全局 Feed”，执行原有的 API 获取逻辑
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

}



// use dioxus::prelude::*;
// use crate::api::article::{fetch_articles};

// #[component]
// // 接收两个布尔值 prop
// pub fn ArticleList(show_your_feed: bool, has_user_clicked_feed_tab: bool) -> Element {
//     // 渲染逻辑：
//     // 仅当满足以下两个条件时，才显示“你的 Feed 为空”的消息：
//     // 1. `show_your_feed` 为 true（表示当前选中的是“你的 Feed”选项卡）
//     // 2. 并且 `has_user_clicked_feed_tab` 为 true（表示用户明确点击了某个 Feed 选项卡，
//     //    并且这个选项卡恰好是“你的 Feed”）。
//     // 这样做的目的是：
//     // - 当用户初次登录，自动切换到 Your Feed 时 (show_your_feed=true, has_user_clicked_feed_tab=false)，
//     //   不会显示“空”消息，而是会显示全局文章（因为 your feed 可能没有文章）。
//     // - 只有当用户明确点击了 Your Feed 选项卡时 (show_your_feed=true, has_user_clicked_feed_tab=true)，
//     //   才会显示“空”消息。
//     if show_your_feed && has_user_clicked_feed_tab {
//         rsx!(p { "Your feed is empty. Start following users or publish your first article!" })
//     } else {
//         // 在所有其他情况下（即 show_your_feed 为 false，或者 show_your_feed 为 true 但不是用户主动点击 Your Feed），
//         // 都显示“全局 Feed”的文章列表
//         let articles = use_resource(|| async {
//             let resp = fetch_articles(Default::default()).await?;
//             Some(resp.articles)
//         });

//         match articles.value()() {
//             Some(Some(articles)) if articles.is_empty() => {
//                 rsx!(p { "No articles are here... yet." })
//             },
//             Some(Some(articles)) => rsx! {
//                 for article in articles.iter() {
//                     div { class: "article-preview",
//                         key: "{article.slug}",
//                         div { class: "article-meta",
//                             a { href: format!("/profile/{}", article.author.username),
//                                 img { src: "{article.author.image}" }
//                             }
//                             div { class: "info",
//                                 a { class: "author", href: format!("/profile/{}", article.author.username),
//                                     "{article.author.username}"
//                                 }
//                                 span { class: "date", "{article.created_at}" }
//                             }
//                             button { class: "btn btn-outline-primary btn-sm pull-xs-right",
//                                 i { class: "ion-heart" }
//                                 " {article.favorites_count} "
//                             }
//                         }
//                         a { class: "preview-link", href: format!("/article/{}", article.slug),
//                             h1 { "{article.title}" }
//                             p { "{article.description}" }
//                             span { "Read more..." }
//                             ul { class: "tag-list",
//                                 for tag in article.tag_list.iter() {
//                                     li { class: "tag-default tag-pill tag-outline",
//                                         key: "{tag}",
//                                         "{tag}"
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//             },
//             Some(None) => rsx! { p { "Failed to load articles." } },
//             None => rsx! { p { "Loading..." } },
//         }
//     }
// }