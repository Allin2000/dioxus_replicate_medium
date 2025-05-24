// use dioxus::prelude::*;

// #[component]
// pub fn Register() -> Element {
//     rsx! {
//         div { class: "auth-page",
//         div { class: "container page",
//             div { class: "row",
//                 div { class: "col-md-6 offset-md-3 col-xs-12",
//                     h1 { class: "text-xs-center", "Sign up" }
//                     p { class: "text-xs-center",
//                         a { href: "/login", "Have an account?" }
//                     }
//                     ul { class: "error-messages",
//                         li { "That email is already taken" }
//                     }
//                     form {
//                         fieldset { class: "form-group",
//                             input {
//                                 class: "form-control form-control-lg",
//                                 placeholder: "Username",
//                                 r#type: "text",
//                             }
//                         }
//                         fieldset { class: "form-group",
//                             input {
//                                 class: "form-control form-control-lg",
//                                 placeholder: "Email",
//                                 r#type: "text",
//                             }
//                         }
//                         fieldset { class: "form-group",
//                             input {
//                                 class: "form-control form-control-lg",
//                                 placeholder: "Password",
//                                 r#type: "password",
//                             }
//                         }
//                         button { class: "btn btn-lg btn-primary pull-xs-right",
//                             "Sign up"
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }
// }


use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;

use crate::api::auth;
use crate::stores::app_state::{AppState, User};
use crate::Route;

#[component]
pub fn Register() -> Element {
    let app_state_signal = use_context::<Signal<AppState>>();
    let navigator = use_navigator();

    let mut username = use_signal(|| "".to_string());
    let mut email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    let mut error_messages = use_signal(|| Vec::<String>::new());
    let is_submitting = use_signal(|| false);

    let on_submit = {
        let username = username.clone();
        let email = email.clone();
        let password = password.clone();
        let mut error_messages = error_messages.clone();
        let mut is_submitting = is_submitting.clone();
        let mut app_state_signal = app_state_signal.clone();
        let navigator = navigator.clone();

        move |event: Event<FormData>| {
            event.prevent_default();

            if *is_submitting.read() {
                return;
            }

            is_submitting.set(true);
            error_messages.set(Vec::new());

            let username_val = username.read().clone();
            let email_val = email.read().clone();
            let password_val = password.read().clone();

            spawn(async move {
                match auth::register_user(&username_val, &email_val, &password_val).await {
                    Some(response) => {
                        log::info!("Register successful: {:?}", response);
                        app_state_signal.write().set_user(User {
                            email: response.user.email,
                            token: response.user.token,
                            username: response.user.username,
                            bio: response.user.bio,
                            image: response.user.image,
                        });
                        navigator.push(Route::GlobalFeed {});
                    }
                    None => {
                        log::error!("Register failed");
                        error_messages.set(vec!["Invalid email or password".to_string()]);
                    }
                }

                is_submitting.set(false);
            });
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

                        ul { class: "error-messages",
                            for msg in error_messages.read().iter() {
                                li { "{msg}" }
                            }
                        }

                        form {
                            onsubmit: on_submit,
                            fieldset { class: "form-group",
                                input {
                                    class: "form-control form-control-lg",
                                    placeholder: "Username",
                                    r#type: "text",
                                    value: "{username.read()}",
                                    required: true,
                                    oninput: move |evt| username.set(evt.value()),
                                }
                            }
                            fieldset { class: "form-group",
                                input {
                                    class: "form-control form-control-lg",
                                    placeholder: "Email",
                                    r#type: "email",
                                    value: "{email.read()}",
                                    required: true,
                                    oninput: move |evt| email.set(evt.value()),
                                }
                            }
                            fieldset { class: "form-group",
                                input {
                                    class: "form-control form-control-lg",
                                    placeholder: "Password",
                                    r#type: "password",
                                    value: "{password.read()}",
                                    required: true,
                                    oninput: move |evt| password.set(evt.value()),
                                }
                            }
                            button {
                                class: "btn btn-lg btn-primary pull-xs-right",
                                r#type: "submit",
                                disabled: *is_submitting.read(),
                                "Sign up"
                            }
                        }
                    }
                }
            }
        }
    }
}