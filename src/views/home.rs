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


// src/views/home.rs
use dioxus::prelude::*;
use crate::components::TagList; 
use crate::components::ArticleList;
// use crate::stores::app_state::{AppState, AuthStatus,HomeState};

use crate::stores::app_state::{APP_STATE, HOME_STATE, AuthStatus};

#[component]
pub fn Home() -> Element {
    // 1. 从 Context 获取全局 AppState 信号
    // let app_state_signal = use_context::<Signal<AppState>>();

    // let mut home_state_signal = use_context::<Signal<HomeState>>();
   
    // // let mut has_user_clicked_feed_tab = home_state_signal.read().has_user_clicked_feed_tab;

    // let mut has_user_clicked_feed_tab = use_signal(|| *home_state_signal.read().has_user_clicked_feed_tab.read());


    // //      //    并同步更新到共享的 home_state_signal 中。
    // // use_effect(move || {
    // //     // 当局部 has_user_clicked_feed_tab 发生变化时，
    // //     // 更新 home_state_signal 内部的 has_user_clicked_feed_tab
    // //     *home_state_signal.write().has_user_clicked_feed_tab.write() = *has_user_clicked_feed_tab.read();
    // // });



    // // 2. 安全地获取登录状


  

    // // 2. 安全地获取登录状态
    // let is_logged_in = matches!(*app_state_signal.read().auth_status.read(), AuthStatus::LoggedIn);



    // New state: track if the user has explicitly clicked a feed tab.
    // let mut has_user_clicked_feed_tab = use_signal(|| false);

    // 4. Use `use_effect` to react to global `AuthStatus` changes and
    // also to initialize or reset `show_your_feed` if no explicit click has occurred.

        // 1. 直接访问全局 APP_STATE
    let is_logged_in = matches!(*APP_STATE.read().auth_status.read(), AuthStatus::LoggedIn);

    // 2. 直接访问全局 HOME_STATE，并读取 `has_user_clicked_feed_tab`
    // 注意这里直接使用 HOME_STATE.read() 来获取其内部的 HomeState 结构
    // 然后再读取 has_user_clicked_feed_tab 信号
    let has_user_clicked_feed_tab = *HOME_STATE.read().has_user_clicked_feed_tab.read();

    // 3. 使用 `use_effect` 来响应 `AuthStatus` 变化，并在未主动点击时调整 `has_user_clicked_feed_tab`
    use_effect(move || {
        spawn(async move { // 启动一个新的异步任务
            let current_auth_status = *APP_STATE.read().auth_status.read(); // 读取当前认证状态
            
            // 获取 HOME_STATE 内部 Signal 的可变守卫
            let mut home_state_guard = HOME_STATE.write();
            let mut has_user_clicked_feed_tab_signal = home_state_guard.has_user_clicked_feed_tab;

            // 仅当用户未明确点击 Feed Tab 时才自动切换
            // 注意：这里的逻辑确保我们不会覆盖用户明确的选择。
            // 如果 Tab 值已经是 true 且用户已登录，则不执行任何操作。
            // 如果 Tab 值已经是 false 且用户已注销，则不执行任何操作。
            // 这可以防止不必要的重新渲染/写入。
            if !*has_user_clicked_feed_tab_signal.read() && current_auth_status == AuthStatus::LoggedIn {
                has_user_clicked_feed_tab_signal.set(true); // 如果已登录且未点击，则默认显示你的 Feed
            } else if *has_user_clicked_feed_tab_signal.read() && current_auth_status == AuthStatus::LoggedOut {
                has_user_clicked_feed_tab_signal.set(false); // 如果已注销且在你的 Feed 上，则切换到全局 Feed
            }
        });
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
                                            class: format!("nav-link {}", if has_user_clicked_feed_tab { "active" } else { "" }),
                                            href: "",
                                            prevent_default: "onclick",
                                            onclick: move |_|{
                                                // let mut has_user_clicked_feed_tab = has_user_clicked_feed_tab.clone();
                                                // has_user_clicked_feed_tab.set(true); // 用户主动点击了，设置标志
                                                 // 修改全局 HOME_STATE 中的 has_user_clicked_feed_tab
                                                *HOME_STATE.write().has_user_clicked_feed_tab.write() = true;
                                                // show_your_feed.set(true);            // 显式切换到 Your Feed
                                            },
                                            "Your Feed"
                                        }
                                    }
                                }
                                // "Global Feed" 选项卡：始终可见
                                li { class: "nav-item",
                                    a {
                                        class: format!("nav-link {}", if !has_user_clicked_feed_tab { "active" } else { "" }),
                                        href: "",
                                        prevent_default: "onclick",
                                        onclick: move |_| {
                                            // let mut has_user_clicked_feed_tab = has_user_clicked_feed_tab.clone();
                                               // 修改全局 HOME_STATE 中的 has_user_clicked_feed_tab
                                            *HOME_STATE.write().has_user_clicked_feed_tab.write() = false;
                                        },
                                        "Global Feed"
                                    }
                                }
                            }
                        }

                        // 将 `show_your_feed` 和 `has_user_clicked_feed_tab` 状态传递给 ArticleList
                        ArticleList {
                            // show_your_feed: *show_your_feed.read(),
                            has_user_clicked_feed_tab: has_user_clicked_feed_tab
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



