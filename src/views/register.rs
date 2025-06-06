// src/views/register.rs
use dioxus::prelude::*;
use crate::api::auth::register_user;
use crate::stores::app_state::{AppState, User};
use crate::Route;

#[component]
pub fn Register() -> Element {
    let app_state_signal = use_context::<Signal<AppState>>();
    let navigator = use_navigator();

    let mut username = use_signal(|| "".to_string());
    let mut email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    let mut is_submitting = use_signal(|| false);
    let mut register_result = use_signal(|| Option::<String>::None);

    let mut handle_register = {
        let username = username.clone();
        let email = email.clone();
        let password = password.clone();
        let mut is_submitting = is_submitting.clone();
        let mut register_result = register_result.clone();
        let mut app_state_signal = app_state_signal.clone();
        let navigator = navigator.clone();

        move || {
            if *is_submitting.read() {
                return;
            }

            let username_val = username();
            let email_val = email();
            let password_val = password();

            if username_val.trim().is_empty() || email_val.trim().is_empty() || password_val.trim().is_empty() {
                register_result.set(Some("❌ 请填写所有字段".to_string()));
                return;
            }

            is_submitting.set(true);
            register_result.set(Some("正在注册...".to_string()));

            spawn(async move {
                match register_user(&username_val, &email_val, &password_val).await {
                    Some(response) => {
                        register_result.set(Some(format!("✅ 注册成功: {}", response.user.username)));
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
                        register_result.set(Some("❌ 注册失败，请检查邮箱或密码".to_string()));
                    }
                }
                is_submitting.set(false);
            });
        }
    };

    let on_submit = {
        let mut handle_register = handle_register.clone();
        move |event: Event<FormData>| {
            event.prevent_default();
            handle_register();
        }
    };

    let on_click = {
        let mut handle_register = handle_register.clone();
        move |_| {
            handle_register();
        }
    };

    rsx! {
        div { class: "auth-page",
            div { class: "container page",
                div { class: "row",
                    div { class: "col-md-6 offset-md-3 col-xs-12",
                        h1 { class: "text-xs-center", "Sign up" }
                        p { class: "text-xs-center",
                            Link { to: Route::Login {}, "Have an account?" }
                        }

                        if let Some(msg) = &*register_result.read() {
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
                                    placeholder: "Username",
                                    r#type: "text",
                                    value: "{username()}",
                                    required: true,
                                    oninput: move |evt| username.set(evt.value()),
                                    onkeydown: move |evt| {
                                        if evt.key() == Key::Enter {
                                            handle_register();
                                        }
                                    }
                                }
                            }
                            fieldset {
                                class: "form-group",
                                input {
                                    class: "form-control form-control-lg",
                                    placeholder: "Email",
                                    r#type: "email",
                                    value: "{email()}",
                                    required: true,
                                    oninput: move |evt| email.set(evt.value()),
                                    onkeydown: move |evt| {
                                        if evt.key() == Key::Enter {
                                            handle_register();
                                        }
                                    }
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
                                    oninput: move |evt| password.set(evt.value()),
                                    onkeydown: move |evt| {
                                        if evt.key() == Key::Enter {
                                            handle_register();
                                        }
                                    }
                                }
                            }
                            button {
                                class: "btn btn-lg btn-primary pull-xs-right",
                                r#type: "submit",
                                disabled: *is_submitting.read(),
                                onclick: on_click,
                                "Sign up"
                            }
                        }
                    }
                }
            }
        }
    }
}