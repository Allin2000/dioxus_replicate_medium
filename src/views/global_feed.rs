// src/views/global_feed.rs
use dioxus::prelude::*;
use crate::components::ArticleList;
use crate::components::ArticleListWithPagination; // 导入新的组件

#[component]
pub fn GlobalFeed() -> Element {
    rsx! {

            ArticleListWithPagination {
                feed_type: "global".to_string(), // Or "all" as in Vue example
            }

    }
}