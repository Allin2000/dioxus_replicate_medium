// src/views/global_feed.rs
use dioxus::prelude::*;
use crate::components::ArticleList;
use crate::components::ArticleListWithPagination; // 导入新的组件

#[component]
pub fn GlobalFeed() -> Element {
    rsx! {
        // div { class: "home-global",
            // Pass the type prop to ArticleList
            // ArticleList {
            //     feed_type: "global".to_string(), // Or "all" as in Vue example
            // }
            ArticleListWithPagination {
                feed_type: "global".to_string(), // Or "all" as in Vue example
            }

        // }
    }
}