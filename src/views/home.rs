// // src/views/home.rs
// use dioxus::prelude::*;
// use crate::components::TagList; 
// use crate::stores::app_state::{AppState, AuthStatus};
// use crate::{Route}; // 



// #[component]
// pub fn Home() -> Element {

//     let app_state_signal = use_context::<Signal<AppState>>();

//     // 2. 安全地获取登录状态
    // // let is_logged_in = matches!(*app_state_signal.read().auth_status.read(), AuthStatus::LoggedIn);
    // let is_logged_in = match *app_state_signal.read().auth_status.read() {
    // AuthStatus::LoggedIn => true,
    // _ => false,
    // };



//     let current_route = use_route::<Route>();

    
//     // 根据路由确定当前的feed类型

//     let current_feed_type = match current_route{
//         Route::YourFeed {} => "Your Feed",
//         Route::GlobalFeed {} => "Global Feed",
//         _ => "Global Feed", // 默认为Global Feed
//     };



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
//                                         Link {
//                                             // Dioxus Link needs a `to` property pointing to a Route enum variant
//                                             to: Route::YourFeed {},
//                                             class: format!("nav-link {}", if current_feed_type == "Your Feed" { "active" } else { "" }),
//                                             "Your Feed"
//                                         }
//                                     }
//                                 }
//                                 // "Global Feed" 选项卡：始终可见
//                                 li { class: "nav-item",
//                                   Link {
//                                          to: Route::GlobalFeed {},
//                                         class: format!("nav-link {}", if current_feed_type == "Global Feed" { "active" } else { "" }),
                        
//                                         "Global Feed"
//                                     }
//                                 }
//                             }
//                         }

//                         // ArticleList {feed_type: current_feed_type.to_string()}
                        
//                         Outlet::<Route> {}

//                         // ul { class: "pagination",
//                         //     li { class: "page-item active",
//                         //         a { class: "page-link", href: "", "1" }
//                         //     }
//                         //     li { class: "page-item",
//                         //         a { class: "page-link", href: "", "2" }
//                         //     }
//                         // }
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




// // src/views/home.rs
// use dioxus::prelude::*;
// use crate::components::TagList; 
// use crate::stores::app_state::{AppState, AuthStatus};
// use crate::{Route}; 

// #[component]
// pub fn Home() -> Element {
//     let app_state_signal = use_context::<Signal<AppState>>();

//     // 安全地获取登录状态
    // // let is_logged_in = matches!(*app_state_signal.read().auth_status.read(), AuthStatus::LoggedIn);
    // let is_logged_in = match *app_state_signal.read().auth_status.read() {
    // AuthStatus::LoggedIn => true,
    // _ => false,
    // };


//     let current_route = use_route::<Route>();
//     let nav = use_navigator();
    
//     // 用于跟踪当前选中的标签
//     let mut selected_tag = use_signal(|| None::<String>);
    
//     // 根据路由确定当前的feed类型
//     let current_feed_type = match current_route {
//         Route::YourFeed {} => {
//             selected_tag.set(None); // 清除选中的标签
//             "Your Feed"
//         },
//         Route::GlobalFeed {} => {
//             selected_tag.set(None); // 清除选中的标签
//             "Global Feed"
//         },
//         _ => "Global Feed", // 默认为Global Feed
//     };

//     // 处理标签点击事件
//     let handle_tag_click = move |tag: String| {
//         selected_tag.set(Some(tag.clone()));
//         // 这里你可以根据需要导航到特定的标签页面
//         // 或者通过其他方式传递标签信息给文章列表组件
//         // nav.push(Route::TagFeed { tag: tag.clone() }); // 如果你有标签路由的话
//     };

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
//                                         Link {
//                                             to: Route::YourFeed {},
//                                             class: format!("nav-link {}", if current_feed_type == "Your Feed" && selected_tag().is_none() { "active" } else { "" }),
//                                             "Your Feed"
//                                         }
//                                     }
//                                 }
//                                 // "Global Feed" 选项卡：始终可见
//                                 li { class: "nav-item",
//                                     Link {
//                                         to: Route::GlobalFeed {},
//                                         class: format!("nav-link {}", if current_feed_type == "Global Feed" && selected_tag().is_none() { "active" } else { "" }),
//                                         "Global Feed"
//                                     }
//                                 }
//                                 // 如果有选中的标签，显示标签选项卡
//                                 if let Some(tag) = selected_tag() {
//                                     li { class: "nav-item",
//                                         a {
//                                             class: "nav-link active",
//                                             href: "#",
//                                             onclick: move |_| selected_tag.set(None),
//                                             span { class: "ion-pound" }
//                                             " {tag}"
//                                         }
//                                     }
//                                 }
//                             }
//                         }

//                         // 这里你需要传递选中的标签给文章列表组件
//                         // ArticleList {
//                         //     feed_type: if let Some(tag) = selected_tag() {
//                         //         format!("tag:{}", tag)
//                         //     } else {
//                         //         current_feed_type.to_string()
//                         //     }
//                         // }
                        
//                         Outlet::<Route> {}
//                     }
//                     div { class: "col-md-3",
//                         div { class: "sidebar",
//                             p { "Popular Tags" }
//                             div { class: "tag-list",
//                                 TagList { 
//                                     on_tag_click: handle_tag_click
//                                 } 
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }






// 更新后的 src/views/home.rs
use dioxus::prelude::*;
use crate::components::TagList; 
use crate::stores::app_state::{AppState, AuthStatus};
use crate::{Route}; 

#[component]
pub fn Home() -> Element {
    let app_state_signal = use_context::<Signal<AppState>>();

    // 安全地获取登录状态
    // let is_logged_in = matches!(*app_state_signal.read().auth_status.read(), AuthStatus::LoggedIn);
    let is_logged_in = match *app_state_signal.read().auth_status.read() {
    AuthStatus::LoggedIn => true,
    _ => false,
    };

    let current_route = use_route::<Route>();
    let nav = use_navigator();
    
    // 根据路由确定当前的feed类型和选中的标签
    let (current_feed_type, selected_tag) = match current_route {
        Route::YourFeed {} => ("Your Feed", None),
        Route::GlobalFeed {} => ("Global Feed", None),
        Route::TagFeed { tag } => ("Tag Feed", Some(tag.clone())),
        _ => ("Global Feed", None),
    };

    // 处理标签点击事件
    let handle_tag_click = move |tag: String| {
        // 导航到标签页面
        nav.push(Route::TagFeed { tag });
    };

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
                                        Link {
                                            to: Route::YourFeed {},
                                            class: format!("nav-link {}", if current_feed_type == "Your Feed" { "active" } else { "" }),
                                            "Your Feed"
                                        }
                                    }
                                }
                                // "Global Feed" 选项卡：始终可见
                                li { class: "nav-item",
                                    Link {
                                        to: Route::GlobalFeed {},
                                        class: format!("nav-link {}", if current_feed_type == "Global Feed" { "active" } else { "" }),
                                        "Global Feed"
                                    }
                                }
                                // 如果有选中的标签，显示标签选项卡
                                if let Some(tag) = &selected_tag {
                                    li { class: "nav-item",
                                        a {
                                            class: "nav-link active",
                                            href: "#",
                                            span { class: "ion-pound" }
                                            " {tag}"
                                        }
                                    }
                                }
                            }
                        }

                        Outlet::<Route> {}
                    }
                    div { class: "col-md-3",
                        div { class: "sidebar",
                            p { "Popular Tags" }
                            div { class: "tag-list",
                                TagList { 
                                    on_tag_click: handle_tag_click
                                } 
                            }
                        }
                    }
                }
            }
        }
    }
}