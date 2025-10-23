// src/views/edit.rs

use dioxus::prelude::*;
use dioxus_router::prelude::{use_route, Link};
use crate::Route;
use crate::api::article::{fetch_article_by_slug, update_article, Article, UpdateArticlePayload};
use crate::stores::app_state::AppState;

#[component]
pub fn Edit(slug: String) -> Element {
    let mut app_state = use_context::<Signal<AppState>>();
    
    let current_route = use_route::<Route>();
    let slug_from_route = match current_route {
        Route::Edit { slug } => Some(slug),
        _ => None,
    };

    let mut title = use_signal(String::new);
    let mut description = use_signal(String::new);
    let mut body = use_signal(String::new);
    let mut tag_input = use_signal(String::new);
    let mut tags = use_signal(Vec::<String>::new);

    let mut is_submitting = use_signal(|| false);
    let mut message = use_signal(|| None::<String>);
    let mut updated_article = use_signal(|| None::<Article>);

    // ✅ FIX: Clone the slug for the effect closure before it's moved.
    let slug_for_effect = slug_from_route.clone();
    use_effect(move || {
        // Now use the cloned value, `slug_for_effect`.
        if let Some(slug_ref) = &slug_for_effect { // Changed to borrow with &
            let slug_cloned = slug_ref.clone(); // Clone for the async block if needed
            let token = app_state.read().user.read().as_ref().map(|u| u.token.clone());
            spawn(async move {
                match fetch_article_by_slug(&slug_cloned, token).await { // Use slug_cloned here
                    Some(response) => {
                        let article = response.article;
                        title.set(article.title.clone());
                        description.set(article.description.clone());
                        body.set(article.body.clone());
                        tags.set(article.tag_list.clone());
                    }
                    None => {
                        message.set(Some("❌ Cannot load article content".to_string()));
                    }
                }
            });
        }
    });

    let mut add_tag = move |_| {
        let tag = tag_input.read().trim().to_string();
        if !tag.is_empty() && !tags.read().contains(&tag) {
            tags.write().push(tag);
            tag_input.set(String::new());
        }
    };

    let mut remove_tag = move |tag_to_remove: String| {
        tags.write().retain(|tag| tag != &tag_to_remove);
    };

    let handle_tag_keypress = move |evt: KeyboardEvent| {
        if evt.key() == Key::Enter {
            evt.prevent_default();
            add_tag(());
        }
    };
    
    // The original `slug_from_route` will be moved into this closure.
    let mut handle_update = move || {
        if *is_submitting.read() {
            return;
        }
        
        let slug = match &slug_from_route {
            Some(s) => s.clone(),
            None => {
                message.set(Some("❌ Article slug not found.".into()));
                return;
            }
        };

        let token_opt = app_state.read().user.read().as_ref().map(|u| u.token.clone());
        if token_opt.is_none() || token_opt.as_ref().unwrap().is_empty() {
            message.set(Some("❌ User is not logged in, cannot edit article".into()));
            return;
        }
        
        let mut errors = vec![];
        if title().trim().is_empty() {
            errors.push("Title cannot be empty");
        }
        if description().trim().is_empty() {
            errors.push("Description cannot be empty");
        }
        if body().trim().is_empty() {
            errors.push("Body cannot be empty");
        }

        if !errors.is_empty() {
            message.set(Some(format!("❌ {}", errors.join(", "))));
            return;
        }

        is_submitting.set(true);
        message.set(Some("Updating article...".into()));
        let token = token_opt.unwrap();

        let payload = UpdateArticlePayload {
            title: Some(title().trim().to_string()),
            description: Some(description().trim().to_string()),
            body: Some(body().trim().to_string()),
        };

        spawn(async move {
            match update_article(&token, &slug, payload).await {
                Some(response) => {
                    updated_article.set(Some(response.article));
                    message.set(Some("✅ Article updated successfully!".into()));
                }
                None => {
                    message.set(Some("❌ Update failed, please try again".into()));
                }
            }
            is_submitting.set(false);
        });
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
        div { class: "editor-page",
            div { class: "container page",
                div { class: "row",
                    div { class: "col-md-10 offset-md-1 col-xs-12",
                        h1 { class: "text-xs-center", "Edit Article" }

                        if let Some(msg) = &*message.read() {
                            ul { class: "error-messages",
                                li { "{msg}" }
                            }
                        }

                        if let Some(article) = updated_article.read().as_ref() {
                            div { class: "article-preview",
                                h2 { "{article.title}" }
                                p { "Article has been updated successfully! Slug: {article.slug}" }
                                Link { to: Route::Article { slug: article.slug.clone() }, class: "btn btn-outline-secondary", "View Article" }
                                " "
                                button {
                                    class: "btn btn-outline-primary",
                                    onclick: move |_| {
                                        updated_article.set(None);
                                        message.set(None);
                                    },
                                    "Continue Editing"
                                }
                            }
                        } else {
                            form {
                                onsubmit: on_submit,
                                fieldset {
                                    fieldset { class: "form-group",
                                        input {
                                            class: "form-control form-control-lg",
                                            placeholder: "Article Title",
                                            r#type: "text",
                                            value: "{title()}",
                                            oninput: move |evt| title.set(evt.value()),
                                        }
                                    }

                                    fieldset { class: "form-group",
                                        input {
                                            class: "form-control",
                                            placeholder: "What's this article about?",
                                            r#type: "text",
                                            value: "{description()}",
                                            oninput: move |evt| description.set(evt.value()),
                                        }
                                    }

                                    fieldset { class: "form-group",
                                        textarea {
                                            class: "form-control",
                                            placeholder: "Write your article (in markdown)",
                                            rows: "8",
                                            value: "{body()}",
                                            oninput: move |evt| body.set(evt.value()),
                                        }
                                    }

                                    fieldset { class: "form-group",
                                        input {
                                            class: "form-control",
                                            placeholder: "Enter tags",
                                            r#type: "text",
                                            value: "{tag_input()}",
                                            oninput: move |evt| tag_input.set(evt.value()),
                                            onkeypress: handle_tag_keypress,
                                        }

                                        div { class: "tag-list",
                                            for tag in tags.read().iter() {
                                                span {
                                                    class: "tag-default tag-pill",
                                                    style: "margin: 2px; cursor: pointer;",
                                                    onclick: {
                                                        let tag = tag.clone();
                                                        move |_| remove_tag(tag.clone())
                                                    },
                                                    i { class: "ion-close-round" }
                                                    " {tag} "
                                                }
                                            }
                                        }
                                    }

                                    button {
                                        class: "btn btn-lg pull-xs-right btn-primary",
                                        r#type: "submit",
                                        disabled: *is_submitting.read(),
                                        onclick: on_click,
                                        if *is_submitting.read() {
                                            "Updating..."
                                        } else {
                                            "Update Article"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}