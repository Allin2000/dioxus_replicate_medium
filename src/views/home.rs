// // src/views/home.rs
// use dioxus::prelude::*;
// use crate::components::TagList; 
// use crate::components::ArticleList;
// use crate::stores::app_state::{AppState, AuthStatus};

// #[component]
// pub fn Home() -> Element {
//     // 1. 从 Context 获取全局 AppState 信号
//     let app_state_signal = use_context::<Signal<AppState>>();

//     // 2. 安全地获取登录状态
//     let is_logged_in = matches!(*app_state_signal.read().auth_status.read(), AuthStatus::LoggedIn);

//     // 3. 本地状态，用于管理当前选中的 Feed 类型
//     // true 表示 "Your Feed" 活跃，false 表示 "Global Feed" 活跃
//     let mut show_your_feed = use_signal(|| is_logged_in); // 默认根据登录状态初始化

//     // let mut has_user_clicked = use_signal(|| true);  // 新状态：用户是否主动点击过


//     // 4. 使用 `use_effect` 响应全局 `AuthStatus` 的变化
//     // use_effect(move || {
//     //     let current_auth_status = *app_state_signal.read().auth_status.read();

//     //     // 当登录状态改变时，自动切换到相应的 Feed 类型
//     //     if current_auth_status == AuthStatus::LoggedIn && !*show_your_feed.read() {
//     //         show_your_feed.set(true); // 如果已登录且当前不在 Your Feed，则切换到 Your Feed
//     //     } else if current_auth_status == AuthStatus::LoggedOut && *show_your_feed.read() {
//     //         show_your_feed.set(false); // 如果已登出且当前在 Your Feed，则切换到 Global Feed
//     //     }
//     // });


// //     use_effect(move || {
// //     let current_auth_status = *app_state_signal.read().auth_status.read();

// //         if current_auth_status == AuthStatus::LoggedIn {
// //             show_your_feed.set(true);
// //         } else if current_auth_status == AuthStatus::LoggedOut  {
// //             show_your_feed.set(false);
// //         }
// // });




//     rsx! {
//         div { class: "home-page",
//             div { class: "banner",
//                 div { class: "container",
//                     h1 { class: "logo-font", "conduit" }
//                     p { "A place to share your knowledge." }
//                 }
//             }
//             div { class: "container page",
//                 div { class: "row",
//                     div { class: "col-md-9",
//                         div { class: "feed-toggle",
//                             ul { class: "nav nav-pills outline-active",
//                                 // 登录状态下才显示 "Your Feed"
//                                 if is_logged_in {
//                                     li { class: "nav-item",
//                                         a {
//                                             class: format!("nav-link {}", if *show_your_feed.read() { "active" } else { "" }),
//                                             href: "",
//                                             prevent_default: "onclick",
//                                             onclick: move |_|{
//                                                     show_your_feed.set(true);           // 用户主动点击了
//                                                   // 显式切换到 Your Feed
//                                             }, // 点击 Your Feed 只刷新文章
//                                             "Your Feed"
//                                         }
//                                     }
//                                 }
//                                 // "Global Feed" 选项卡：始终可见
//                                 li { class: "nav-item",
//                                     a {
//                                         class: format!("nav-link {}", if !*show_your_feed.read() { "active" } else { "" }),
//                                         href: "",
//                                         prevent_default: "onclick",
//                                         onclick: move |_| {
//                                                 show_your_feed.set(false);

//                                         }, // 点击 Global Feed 只刷新文章
//                                         "Global Feed"
//                                     }
//                                 }
//                             }
//                         }

//                         // 将 `show_your_feed` 状态传递给 ArticleList
//                         ArticleList { show_your_feed:*show_your_feed.read()}

//                         ul { class: "pagination",
//                             li { class: "page-item active",
//                                 a { class: "page-link", href: "", "1" }
//                             }
//                             li { class: "page-item",
//                                 a { class: "page-link", href: "", "2" }
//                             }
//                         }
//                     }
//                     div { class: "col-md-3",
//                         div { class: "sidebar",
//                             p { "Popular Tags" }
//                             div { class: "tag-list",
//                                 TagList { } 
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }




use dioxus::prelude::*;
use crate::components::TagList;
use crate::components::ArticleList;
use crate::stores::app_state::{AppState, AuthStatus};

#[component]
pub fn Home() -> Element {
    // 1. 从 Context 获取全局 AppState 信号
    let app_state_signal = use_context::<Signal<AppState>>();

    // 2. 安全地获取登录状态


    let is_logged_in = matches!(*app_state_signal.read().auth_status.read(), AuthStatus::LoggedIn);

    // 3. 本地状态：管理当前选中的 Feed 类型
    // true 表示 "Your Feed" 活跃，false 表示 "Global Feed" 活跃
    // 默认根据登录状态初始化
    let mut show_your_feed = use_signal(|| is_logged_in);

    // 新状态：用户是否主动点击过任意 Feed 选项卡。初始为 false。
    // 这个信号是关键，用来区分“自动切换”和“用户手动选择”。
    let mut has_user_clicked_feed_tab = use_signal(|| false);

    // 4. 使用 `use_effect` 响应全局 `AuthStatus` 的变化
    // 这个 effect 只在用户没有手动选择过 Feed 时（has_user_clicked_feed_tab 为 false）才自动切换。
    use_effect(move || {
        let current_auth_status = *app_state_signal.read().auth_status.read();

        if !*has_user_clicked_feed_tab.read() { // 仅当用户未手动点击时才进行自动切换
            if current_auth_status == AuthStatus::LoggedIn && !*show_your_feed.read() {
                // 如果已登录且当前不在 Your Feed，则自动切换到 Your Feed
                show_your_feed.set(true);
            } else if current_auth_status == AuthStatus::LoggedOut && *show_your_feed.read() {
                // 如果已登出且当前在 Your Feed，则自动切换到 Global Feed
                // show_your_feed.set(false);
            }
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
                                // 登录状态下才显示 "Your Feed" 选项卡
                                if is_logged_in {
                                    li { class: "nav-item",
                                        a {
                                            class: format!("nav-link {}", if *show_your_feed.read() { "active" } else { "" }),
                                            href: "",
                                            prevent_default: "onclick",
                                            onclick: move |_|{
                                                has_user_clicked_feed_tab.set(true); // 用户主动点击了，设置标志
                                                // show_your_feed.set(true);            // 显式切换到 Your Feed
                                            },
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
                                        onclick: move |_| {
                                            has_user_clicked_feed_tab.set(false); // 用户主动点击了，设置标志
                                            // show_your_feed.set(false);           // 显式切换到 Global Feed
                                        },
                                        "Global Feed"
                                    }
                                }
                            }
                        }

                        // 将 `show_your_feed` 和 `has_user_clicked_feed_tab` 状态传递给 ArticleList
                        ArticleList {
                            // show_your_feed: *show_your_feed.read(),
                            has_user_clicked_feed_tab: *has_user_clicked_feed_tab.read()
                        }

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
                                TagList { }
                            }
                        }
                    }
                }
            }
        }
    }
}



