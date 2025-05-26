// use dioxus::prelude::*;

// // const HEADER_SVG: Asset = asset!("/assets/header.svg");



// #[component]
// pub fn Header() -> Element {
//     rsx! {
//         nav { class: "navbar navbar-light",
//                 div { class: "container",
//                     a { class: "navbar-brand", href: "/", "conduit" }
//                     ul { class: "nav navbar-nav pull-xs-right",
//                         li { class: "nav-item",
//                             a { class: "nav-link active", href: "/", "Home" }
//                         }
//                         li { class: "nav-item",
//                             a { class: "nav-link", href: "/login", "Sign in" }
//                         }
//                         li { class: "nav-item",
//                             a { class: "nav-link", href: "/register", "Sign up" }
//                         }
//                     }
//                 }
//         }

//     }
// }




use dioxus::prelude::*;
use crate::stores::app_state::{AppState, AuthStatus};

#[component]
pub fn Header() -> Element {
    // 使用 try_use_context 来安全地获取上下文
    let app_state = match try_use_context::<Signal<AppState>>() {
        Some(state) => state,
        None => {
            // 如果上下文不存在，返回一个错误提示或默认状态
            log::error!("AppState context not found!");
            return rsx! {
                nav { class: "navbar navbar-light",
                    div { class: "container",
                        a { class: "navbar-brand", href: "/", "conduit" }
                        p { "Error: AppState not found" }
                    }
                }
            };
        }
    };
    
    // let auth_status = app_state.read().auth_status.read();
    // let user = app_state.read().user.read();
    let app_state_read = app_state.read();
    let auth_status = *app_state_read.auth_status.read();
    let user = app_state_read.user.read().clone();

    rsx! {
        nav { class: "navbar navbar-light",
            div { class: "container",
                a { class: "navbar-brand", href: "/", "conduit" }
                ul { class: "nav navbar-nav pull-xs-right",
                    li { class: "nav-item",
                        a {
                            class: "nav-link active",
                            href: "/", "Home"
                        }
                    }

                    if auth_status == AuthStatus::LoggedOut {
                        li { class: "nav-item",
                            a { class: "nav-link", href: "/login", "Sign in" }
                        }
                        li { class: "nav-item",
                            a { class: "nav-link", href: "/register", "Sign up" }
                        }
                    }

                    if auth_status == AuthStatus::LoggedIn {
                        li { class: "nav-item",
                            a { class: "nav-link", href: "/create_edit",
                                i { class: "ion-compose" }
                                "New Article"
                            }
                        }
                        li { class: "nav-item",
                            a { class: "nav-link", href: "/settings",
                                i { class: "ion-gear-a" }
                                "Settings"
                            }
                        }
                        li { class: "nav-item",
                            if let Some(user_info) = user.as_ref() {
                                a { class: "nav-link", href: "/profile/{user_info.username}",
                                    if let Some(image_url) = user_info.image.as_ref() {
                                        img {
                                            class: "user-pic",
                                            src: "{image_url}",
                                        }
                                    }
                                    "{user_info.username}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}