// src/views/your_feed.rs
use dioxus::prelude::*;
use crate::components::ArticleList;
use crate::components::ArticleListWithPagination; // 导入新的组件

#[component]
pub fn YourFeed() -> Element {
    rsx! {
            ArticleListWithPagination {
                feed_type: "your".to_string(), // Or "feed" as in Vue example
            }

    }
}