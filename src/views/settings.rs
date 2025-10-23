// src/views/settings.rs
use dioxus::prelude::*;
use crate::api::auth::{update_user, UpdateUserPayload};
use crate::stores::app_state::{AppState, User};
use crate::Route;


#[component]
pub fn Settings() -> Element {
    let mut app_state = use_context::<Signal<AppState>>();
    let navigator = use_navigator();

    // 先安全获取用户
    let user_opt = app_state.read().user.read().clone();

    // 初始化 message 和 is_submitting
    let mut message = use_signal(|| Option::<String>::None);
    let mut is_submitting = use_signal(|| false);

    // 如果没登录，初始化为空，避免编译报错
    let (mut image, mut username, mut bio, mut email, mut password) = if let Some(user) = &user_opt {
        (
            use_signal(|| user.image.clone().unwrap_or_default()),
            use_signal(|| user.username.clone()),
            use_signal(|| user.bio.clone().unwrap_or_default()),
            use_signal(|| user.email.clone()),
            use_signal(|| "".to_string()),
        )
    } else {
        (
            use_signal(|| "".to_string()),
            use_signal(|| "".to_string()),
            use_signal(|| "".to_string()),
            use_signal(|| "".to_string()),
            use_signal(|| "".to_string()),
        )
    };

    let mut handle_update = {
        let image = image.clone();
        let username = username.clone();
        let bio = bio.clone();
        let email = email.clone();
        let password = password.clone();
        let mut app_state = app_state.clone();
        let mut message = message.clone();
        let mut is_submitting = is_submitting.clone();
        let navigator = navigator.clone();

        move || {
            // 判断登录状态
            let token_opt = app_state.read().user.read().as_ref().map(|u| u.token.clone());
            if token_opt.is_none() || token_opt.as_ref().unwrap().is_empty() {
                message.set(Some("❌ 用户未登录，无法更新设置".into()));
                return;
            }

            if *is_submitting.read() {
                return;
            }

            is_submitting.set(true);
            message.set(Some("正在更新设置...".into()));

            let token = token_opt.unwrap();

            let payload = UpdateUserPayload {
                image: Some(image()),
                username: Some(username()),
                bio: Some(bio()),
                email: Some(email()),
                password: if password().is_empty() { None } else { Some(password()) },
            };

            spawn(async move {
                match update_user(&token, payload).await {
                    Some(response) => {
                        app_state.write().set_user(User {
                            email: response.user.email,
                            username: response.user.username,
                            token:response.user.token,
                            bio: response.user.bio,
                            image: response.user.image,
                        });
                        message.set(Some("✅ 更新成功！".into()));
                        navigator.push(Route::GlobalFeed {});
                    }
                    None => {
                        message.set(Some("❌ 更新失败".into()));
                    }
                }
                is_submitting.set(false);
            });
        }
    };

    let on_submit = {
        let mut handle_update = handle_update.clone();
        move |event: Event<FormData>| {
            event.prevent_default();
            handle_update();
        }
    };

    let on_click = {
        let mut handle_update = handle_update.clone();
        move |_| {
            handle_update();
        }
    };

    rsx! {
        div { class: "settings-page",
            div { class: "container page",
                div { class: "row",
                    div { class: "col-md-6 offset-md-3 col-xs-12",
                        h1 { class: "text-xs-center", "Your Settings" }

                        if let Some(msg) = &*message.read() {
                            ul { class: "error-messages",
                                li { "{msg}" }
                            }
                        }

                        form {
                            onsubmit: on_submit,
                            fieldset {
                                fieldset { class: "form-group",
                                    input {
                                        class: "form-control",
                                        placeholder: "URL of profile picture",
                                        r#type: "text",
                                        value: "{image()}",
                                        oninput: move |e| image.set(e.value()),
                                    }
                                }
                                fieldset { class: "form-group",
                                    input {
                                        class: "form-control form-control-lg",
                                        placeholder: "Your Name",
                                        r#type: "text",
                                        value: "{username()}",
                                        oninput: move |e| username.set(e.value()),
                                    }
                                }
                                fieldset { class: "form-group",
                                    textarea {
                                        class: "form-control form-control-lg",
                                        placeholder: "Short bio about you",
                                        rows: "8",
                                        value: "{bio()}",
                                        oninput: move |e| bio.set(e.value()),
                                    }
                                }
                                fieldset { class: "form-group",
                                    input {
                                        class: "form-control form-control-lg",
                                        placeholder: "Email",
                                        r#type: "email",
                                        value: "{email()}",
                                        oninput: move |e| email.set(e.value()),
                                    }
                                }
                                fieldset { class: "form-group",
                                    input {
                                        class: "form-control form-control-lg",
                                        placeholder: "New Password",
                                        r#type: "password",
                                        value: "{password()}",
                                        oninput: move |e| password.set(e.value()),
                                    }
                                }
                                button {
                                    class: "btn btn-lg btn-primary pull-xs-right",
                                    r#type: "submit",
                                    disabled: *is_submitting.read(),
                                    onclick: on_click,
                                    "Update Settings"
                                }
                            }
                        }

                        hr {}
                        button {
                            class: "btn btn-outline-danger",
                            onclick: move |_| {
                                app_state.write().clear_user();
                                navigator.push(Route::Login {});
                            },
                            "Or click here to logout."
                        }
                    }
                }
            }
        }
    }
}