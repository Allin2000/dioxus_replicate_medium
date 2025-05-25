// src/views/your_feed.rs
use dioxus::prelude::*;
use crate::components::ArticleList;

#[component]
pub fn YourFeed() -> Element {
    rsx! {
            ArticleList {
                feed_type: "your".to_string(), // Or "feed" as in Vue example
            }
    }
}