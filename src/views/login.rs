// // src/views/login.rs
// 问题代码移除使用enter进行登录就代码正确
// use dioxus::prelude::*;
// use dioxus_router::prelude::use_navigator;

// use crate::api::auth;
// use crate::stores::app_state::{AppState, User};
// use crate::Route;

// #[component]
// pub fn Login() -> Element {
//     let app_state_signal = use_context::<Signal<AppState>>();
//     let navigator = use_navigator();

//     let mut email = use_signal(|| "".to_string());
//     let mut password = use_signal(|| "".to_string());
//     let mut is_submitting = use_signal(|| false);
//     let mut login_result = use_signal(|| Option::<String>::None);

//     let mut handle_login = {
//         let email = email.clone();
//         let password = password.clone();
//         let mut is_submitting = is_submitting.clone();
//         let mut login_result = login_result.clone();
//         let mut app_state_signal = app_state_signal.clone();
//         let navigator = navigator.clone();

//         move || {
//             if *is_submitting.read() {
//                 return;
//             }

//             let email_val = email();
//             let password_val = password();

//             if email_val.trim().is_empty() || password_val.trim().is_empty() {
//                 login_result.set(Some("❌ 请填写所有字段".to_string()));
//                 return;
//             }

//             is_submitting.set(true);
//             login_result.set(Some("正在登录...".to_string()));

//             spawn(async move {
//                 match auth::login_user(&email_val, &password_val).await {
//                     Some(response) => {
//                         login_result.set(Some(format!("✅ 登录成功: {}", response.user.username)));
//                         app_state_signal.write().set_user(User {
//                             email: response.user.email,
//                             username: response.user.username,
//                             token: response.user.token,
//                             bio: response.user.bio,
//                             image: response.user.image,
//                         });
//                         navigator.push(Route::GlobalFeed {});
//                     }
//                     None => {
//                         login_result.set(Some("❌ 登录失败，请检查邮箱或密码".to_string()));
//                     }
//                 }
//                 is_submitting.set(false);
//             });
//         }
//     };

//     let on_submit = {
//         let mut handle_login = handle_login.clone();
//         move |event: Event<FormData>| {
//             event.prevent_default();
//             handle_login();
//         }
//     };

//     let on_click = {
//         let mut handle_login = handle_login.clone();
//         move |_| {
//             handle_login();
//         }
//     };

//     rsx! {
//         div { class: "auth-page",
//             div { class: "container page",
//                 div { class: "row",
//                     div { class: "col-md-6 offset-md-3 col-xs-12",
//                         h1 { class: "text-xs-center", "Sign in" }
//                         p { class: "text-xs-center",
//                             Link { to: Route::Register {}, "Need an account?" }
//                         }

//                         if let Some(msg) = &*login_result.read() {
//                             ul { class: "error-messages",
//                                 li { "{msg}" }
//                             }
//                         }

//                         form {
//                             onsubmit: on_submit,
//                             method: "post",
//                             fieldset {
//                                 class: "form-group",
//                                 input {
//                                     class: "form-control form-control-lg",
//                                     placeholder: "Email",
//                                     r#type: "email",
//                                     value: "{email()}",
//                                     required: true,
//                                     oninput: move |evt| email.set(evt.value()),
//                                     onkeydown: move |evt| {
//                                         if evt.key() == Key::Enter {
//                                             handle_login();
//                                         }
//                                     }
//                                 }
//                             }
//                             fieldset {
//                                 class: "form-group",
//                                 input {
//                                     class: "form-control form-control-lg",
//                                     placeholder: "Password",
//                                     r#type: "password",
//                                     value: "{password()}",
//                                     required: true,
//                                     oninput: move |evt| password.set(evt.value()),
//                                     onkeydown: move |evt| {
//                                         if evt.key() == Key::Enter {
//                                             handle_login();
//                                         }
//                                     }
//                                 }
//                             }
//                             button {
//                                 class: "btn btn-lg btn-primary pull-xs-right",
//                                 r#type: "submit",
//                                 disabled: *is_submitting.read(),
//                                 onclick: on_click,
//                                 "Sign in"
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }


// src/views/login.rs
use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;

use crate::api::auth;
use crate::stores::app_state::{AppState, User};
use crate::Route;

#[component]
pub fn Login() -> Element {
    let app_state_signal = use_context::<Signal<AppState>>();
    let navigator = use_navigator();

    let mut email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    let mut is_submitting = use_signal(|| false);
    let mut login_result = use_signal(|| Option::<String>::None);

    let mut handle_login = {
        let email = email.clone();
        let password = password.clone();
        let mut is_submitting = is_submitting.clone();
        let mut login_result = login_result.clone();
        let mut app_state_signal = app_state_signal.clone();
        let navigator = navigator.clone();

        move || {
            if *is_submitting.read() {
                return;
            }

            let email_val = email();
            let password_val = password();

            if email_val.trim().is_empty() || password_val.trim().is_empty() {
                login_result.set(Some("❌ 请填写所有字段".to_string()));
                return;
            }

            is_submitting.set(true);
            login_result.set(Some("正在登录...".to_string()));

            spawn(async move {
                match auth::login_user(&email_val, &password_val).await {
                    Some(response) => {
                        login_result.set(Some(format!("✅ 登录成功: {}", response.user.username)));
                        app_state_signal.write().set_user(User {
                            email: response.user.email,
                            username: response.user.username,
                            token: response.user.token,
                            bio: response.user.bio,
                            image: response.user.image,
                        });
                        navigator.push(Route::GlobalFeed {});
                    }
                    None => {
                        login_result.set(Some("❌ 登录失败，请检查邮箱或密码".to_string()));
                    }
                }
                is_submitting.set(false);
            });
        }
    };

    let on_submit = {
        let mut handle_login = handle_login.clone();
        move |event: Event<FormData>| {
            event.prevent_default();
            handle_login();
        }
    };

    let on_click = {
        let mut handle_login = handle_login.clone();
        move |_| {
            handle_login();
        }
    };

    rsx! {
        div { class: "auth-page",
            div { class: "container page",
                div { class: "row",
                    div { class: "col-md-6 offset-md-3 col-xs-12",
                        h1 { class: "text-xs-center", "Sign in" }
                        p { class: "text-xs-center",
                            Link { to: Route::Register {}, "Need an account?" }
                        }

                        if let Some(msg) = &*login_result.read() {
                            ul { class: "error-messages",
                                li { "{msg}" }
                            }
                        }

                        form {
                            onsubmit: on_submit,
                            fieldset {
                                class: "form-group",
                                input {
                                    class: "form-control form-control-lg",
                                    placeholder: "Email",
                                    r#type: "email",
                                    value: "{email()}",
                                    required: true,
                                    oninput: move |evt| email.set(evt.value())
                                }
                            }
                            fieldset {
                                class: "form-group",
                                input {
                                    class: "form-control form-control-lg",
                                    placeholder: "Password",
                                    r#type: "password",
                                    value: "{password()}",
                                    required: true,
                                    oninput: move |evt| password.set(evt.value())
                                }
                            }
                            button {
                                class: "btn btn-lg btn-primary pull-xs-right",
                                r#type: "submit",
                                disabled: *is_submitting.read(),
                                onclick: on_click,
                                "Sign in"
                            }
                        }
                    }
                }
            }
        }
    }
}



