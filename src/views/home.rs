// src/views/home.rs
use dioxus::prelude::*;
use crate::components::TagList; 
use crate::stores::app_state::{AppState, AuthStatus};
use crate::{Route}; // 



#[component]
pub fn Home() -> Element {

    let app_state_signal = use_context::<Signal<AppState>>();

    // 2. 安全地获取登录状态
    let is_logged_in = matches!(*app_state_signal.read().auth_status.read(), AuthStatus::LoggedIn);


    let current_route = use_route::<Route>();

    
    // 根据路由确定当前的feed类型

    let current_feed_type = match current_route{
        Route::YourFeed {} => "Your Feed",
        Route::GlobalFeed {} => "Global Feed",
        _ => "Global Feed", // 默认为Global Feed
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
                                            // Dioxus Link needs a `to` property pointing to a Route enum variant
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
                            }
                        }

                        // ArticleList {feed_type: current_feed_type.to_string()}
                        
                        Outlet::<Route> {}

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

