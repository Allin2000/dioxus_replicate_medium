// src/views/home.rs
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