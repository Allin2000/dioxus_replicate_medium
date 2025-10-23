// src/components/pagination.rs
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct PaginationProps {
    pub current_page: usize,
    pub total_count: usize,
    #[props(default = 10)]
    pub page_size: usize,
    pub on_page_change: EventHandler<usize>,
    #[props(default = 5)]
    pub max_visible_pages: usize, // 最多显示多少页码
}

#[component]
pub fn Pagination(props: PaginationProps) -> Element {
    let total_pages = (props.total_count as f32 / props.page_size as f32).ceil() as usize;

    if total_pages <= 1 {
        return rsx! { Fragment {} };
    }

    // 计算显示的页码范围
    let (start_page, end_page) = calculate_page_range(
        props.current_page,
        total_pages,
        props.max_visible_pages,
    );

    rsx! {
        ul { class: "pagination",
            // 上一页按钮
            if props.current_page > 1 {
                li { class: "page-item",
                    a {
                        class: "page-link",
                        href: "#",
                        onclick: move |_| props.on_page_change.call(props.current_page - 1),
                        "‹"
                    }
                }
            }

            // 第一页（如果不在范围内）
            if start_page > 1 {
                li { class: "page-item",
                    a {
                        class: "page-link",
                        href: "#",
                        onclick: move |_| props.on_page_change.call(1),
                        "1"
                    }
                }
                if start_page > 2 {
                    li { class: "page-item disabled",
                        span { class: "page-link", "..." }
                    }
                }
            }

            // 页码范围
            for i in start_page..=end_page {
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

            // 最后一页（如果不在范围内）
            if end_page < total_pages {
                if end_page < total_pages - 1 {
                    li { class: "page-item disabled",
                        span { class: "page-link", "..." }
                    }
                }
                li { class: "page-item",
                    a {
                        class: "page-link",
                        href: "#",
                        onclick: move |_| props.on_page_change.call(total_pages),
                        "{total_pages}"
                    }
                }
            }

            // 下一页按钮
            if props.current_page < total_pages {
                li { class: "page-item",
                    a {
                        class: "page-link",
                        href: "#",
                        onclick: move |_| props.on_page_change.call(props.current_page + 1),
                        "›"
                    }
                }
            }
        }
    }
}

fn calculate_page_range(current_page: usize, total_pages: usize, max_visible: usize) -> (usize, usize) {
    if total_pages <= max_visible {
        return (1, total_pages);
    }

    let half = max_visible / 2;
    let start = if current_page <= half {
        1
    } else if current_page + half >= total_pages {
        total_pages - max_visible + 1
    } else {
        current_page - half
    };

    let end = (start + max_visible - 1).min(total_pages);
    (start, end)
}