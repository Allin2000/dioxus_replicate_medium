// use dioxus::prelude::*;
// use crate::components::TagList;
// use crate::components::ArticleList;

// #[component]
// pub fn Home() -> Element {

//     rsx! {
//         div { class: "home-page",
//         div { class: "banner",
//             div { class: "container",
//                 h1 { class: "logo-font", "conduit" }
//                 p { "A place to share your knowledge." }
//             }
//         }
//         div { class: "container page",
//             div { class: "row",
//                 div { class: "col-md-9",
//                     div { class: "feed-toggle",
//                         ul { class: "nav nav-pills outline-active",
//                             li { class: "nav-item",
//                                 a { class: "nav-link", href: "", "Your Feed" }
//                             }
//                             li { class: "nav-item",
//                                 a {
//                                     class: "nav-link active",
//                                     href: "",
//                                     "Global Feed"
//                                 }
//                             }
//                         }
//                     }
                    
//                     ArticleList {}

//                     ul { class: "pagination",
//                         li { class: "page-item active",
//                             a { class: "page-link", href: "", "1" }
//                         }
//                         li { class: "page-item",
//                             a { class: "page-link", href: "", "2" }
//                         }
//                     }
//                 }
//                 div { class: "col-md-3",
//                     div { class: "sidebar",
//                         p { "Popular Tags" }
//                         div { class: "tag-list",
//                                 TagList {  }
//                             }
//                         }
//                 }
//             }
//         }
//     }
//     }
// }



// use crate::components::Hero;
// use dioxus::prelude::*;

// #[component]
// pub fn Home() -> Element {
//     rsx! {
//         Hero {}

//     }
// }

// use crate::components::Hero;



// src/views/home.rs
use dioxus::prelude::*;
use crate::components::TagList; // 如果 TagList 还没有实现或暂时不需要，可以先注释掉
use crate::components::ArticleList;
use crate::stores::app_state::{AppState, AuthStatus};

#[component]
pub fn Home() -> Element {
    // 1. 从 Context 获取全局 AppState 信号
    let app_state_signal = use_context::<Signal<AppState>>();

    // 2. 安全地获取登录状态
    let is_logged_in = matches!(*app_state_signal.read().auth_status.read(), AuthStatus::LoggedIn);

    // 3. 本地状态，用于管理当前选中的 Feed 类型
    // true 表示 "Your Feed" 活跃，false 表示 "Global Feed" 活跃
    let show_your_feed = use_signal(|| is_logged_in); // 默认根据登录状态初始化

    // 4. 使用 `use_effect` 响应全局 `AuthStatus` 的变化
    use_effect(move || {
        let current_auth_status = *app_state_signal.read().auth_status.read();

        // 当登录状态改变时，自动切换到相应的 Feed 类型
        if current_auth_status == AuthStatus::LoggedIn && !*show_your_feed.read() {
            show_your_feed.set(true); // 如果已登录且当前不在 Your Feed，则切换到 Your Feed
        } else if current_auth_status == AuthStatus::LoggedOut && *show_your_feed.read() {
            show_your_feed.set(false); // 如果已登出且当前在 Your Feed，则切换到 Global Feed
        }
    });

    rsx! {
        div { class: "home-page",
            div { class: "banner",
                div { class: "container",
                    h1 { class: "logo-font", "conduit" }
                    p { "A place to share your knowledge." }
                }
            }
            div { class: "container page",
                div { class: "row",
                    div { class: "col-md-9",
                        div { class: "feed-toggle",
                            ul { class: "nav nav-pills outline-active",
                                // 登录状态下才显示 "Your Feed"
                                if is_logged_in {
                                    li { class: "nav-item",
                                        a {
                                            class: format!("nav-link {}", if *show_your_feed.read() { "active" } else { "" }),
                                            href: "",
                                            prevent_default: "onclick",
                                            onclick: move |_| show_your_feed.set(true), // 点击 Your Feed 设为 true
                                            "Your Feed"
                                        }
                                    }
                                }
                                // "Global Feed" 选项卡：始终可见
                                li { class: "nav-item",
                                    a {
                                        class: format!("nav-link {}", if !*show_your_feed.read() { "active" } else { "" }),
                                        href: "",
                                        prevent_default: "onclick",
                                        onclick: move |_| show_your_feed.set(false), // 点击 Global Feed 设为 false
                                        "Global Feed"
                                    }
                                }
                            }
                        }

                        // 将 `show_your_feed` 状态传递给 ArticleList
                        ArticleList { show_your_feed: *show_your_feed.read() }

                        ul { class: "pagination",
                            li { class: "page-item active",
                                a { class: "page-link", href: "", "1" }
                            }
                            li { class: "page-item",
                                a { class: "page-link", href: "", "2" }
                            }
                        }
                    }
                    div { class: "col-md-3",
                        div { class: "sidebar",
                            p { "Popular Tags" }
                            div { class: "tag-list",
                                TagList { } // 如果 TagList 没有实现，可以先注释掉
                                p {"Loading Tags..."} // 临时占位符
                            }
                        }
                    }
                }
            }
        }
    }
}