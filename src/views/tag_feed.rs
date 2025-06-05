// src/views/tag_feed.rs
use dioxus::prelude::*;
use crate::components::ArticleList;
use crate::components::ArticleListWithPagination;

#[component]
pub fn TagFeed(tag: String) -> Element {
    rsx! {

        ArticleList {
              feed_type: format!("tag:{}", tag),
        }
        // ArticleListWithPagination { 
        //     feed_type: format!("tag:{}", tag),

        // }
    }
}