// src/components/articlepagination.rs
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct PaginationProps {
    pub current_page: usize,
    pub total_count: usize,
    #[props(default = 10)]
    pub page_size: usize,
    pub on_page_change: EventHandler<usize>,
}

#[component]
pub fn Pagination(props: PaginationProps) -> Element {
    let total_pages = (props.total_count as f32 / props.page_size as f32).ceil() as usize;

    if total_pages <= 1 {
        return rsx! { Fragment {} }; // ✅ 返回一个空的 Fragment
    }

    rsx! {
        ul { class: "pagination",
            for i in 1..=total_pages {
                li {
                    class: if i == props.current_page { "page-item active" } else { "page-item" },
                    key: "{i}",
                    a {
                        class: "page-link",
                        href: "#",
                        onclick: move |_| props.on_page_change.call(i),
                        "{i}"
                    }
                }
            }
        }
    }
}