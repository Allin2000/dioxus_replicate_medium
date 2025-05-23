use dioxus::prelude::*;
use crate::components::ArticleList;

#[component]
pub fn YourFeed() -> Element {
    rsx! {
        // div { class: "home-my-feed",
            // Pass the type prop to ArticleList
            ArticleList {
                feed_type: "your".to_string(), // Or "feed" as in Vue example
            }
        // }
    }
}