// // src/api/create_edit.rs
// use dioxus::prelude::*;

// #[component]
// pub fn Create_edit() -> Element {
//     rsx! {
//         div { class: "editor-page",
//         div { class: "container page",
//             div { class: "row",
//                 div { class: "col-md-10 offset-md-1 col-xs-12",
//                     ul { class: "error-messages",
//                         li { "That title is required" }
//                     }
//                     form {
//                         fieldset {
//                             fieldset { class: "form-group",
//                                 input {
//                                     class: "form-control form-control-lg",
//                                     placeholder: "Article Title",
//                                     r#type: "text",
//                                 }
//                             }
//                             fieldset { class: "form-group",
//                                 input {
//                                     class: "form-control",
//                                     placeholder: "What's this article about?",
//                                     r#type: "text",
//                                 }
//                             }
//                             fieldset { class: "form-group",
//                                 textarea {
//                                     class: "form-control",
//                                     placeholder: "Write your article (in markdown)",
//                                     rows: "8",
//                                 }
//                             }
//                             fieldset { class: "form-group",
//                                 input {
//                                     class: "form-control",
//                                     placeholder: "Enter tags",
//                                     r#type: "text",
//                                 }
//                                 div { class: "tag-list",
//                                     span { class: "tag-default tag-pill",
//                                         i { class: "ion-close-round" }
//                                         " tag "
//                                     }
//                                 }
//                             }
//                             button {
//                                 class: "btn btn-lg pull-xs-right btn-primary",
//                                 r#type: "button",
//                                 "\n                Publish Article\n              "
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }
// }




// src/api/create_edit.rs
use dioxus::prelude::*;
use crate::api::article::{create_article, CreateArticlePayload, Article};
use crate::stores::app_state::AppState;

#[component]
pub fn Create_edit() -> Element {
    let mut app_state = use_context::<Signal<AppState>>();
    
    // 先安全获取用户
    let user_opt = app_state.read().user.read().clone();
    
    // 表单状态
    let mut title = use_signal(|| String::new());
    let mut description = use_signal(|| String::new());
    let mut body = use_signal(|| String::new());
    let mut tag_input = use_signal(|| String::new());
    let mut tags = use_signal(|| Vec::<String>::new());
    
    // UI 状态
    let mut is_submitting = use_signal(|| false);
    let mut message = use_signal(|| Option::<String>::None);
    let mut created_article = use_signal(|| None::<Article>);

    // 添加标签
    let mut add_tag = move |_| {
        let mut tag = tag_input.read().trim().to_string();
        if !tag.is_empty() && !tags.read().contains(&tag) {
            tags.write().push(tag);
            tag_input.set(String::new());
        }
    };

    // 移除标签
    let mut remove_tag = move |tag_to_remove: String| {
        tags.write().retain(|tag| tag != &tag_to_remove);
    };

    // 处理标签输入的回车键
    let handle_tag_keypress = move |evt: KeyboardEvent| {
        if evt.key() == Key::Enter {
            evt.prevent_default();
            add_tag(());
        }
    };

    // 创建文章处理函数
    let mut handle_create = {
        let mut title = title.clone();
        let mut description = description.clone();
        let mut body = body.clone();
        let mut tags = tags.clone();
        let mut app_state = app_state.clone();
        let mut message = message.clone();
        let mut is_submitting = is_submitting.clone();
        let mut created_article = created_article.clone();

        move || {
            // 判断登录状态
            let token_opt = app_state.read().user.read().as_ref().map(|u| u.token.clone());
            if token_opt.is_none() || token_opt.as_ref().unwrap().is_empty() {
                message.set(Some("❌ 用户未登录，无法创建文章".into()));
                return;
            }

            if *is_submitting.read() {
                return;
            }

            // 表单验证
            let mut errors = Vec::new();
            
            if title().trim().is_empty() {
                errors.push("标题不能为空");
            }
            
            if description().trim().is_empty() {
                errors.push("描述不能为空");
            }
            
            if body().trim().is_empty() {
                errors.push("文章内容不能为空");
            }

            if !errors.is_empty() {
                message.set(Some(format!("❌ {}", errors.join("，"))));
                return;
            }

            is_submitting.set(true);
            message.set(Some("正在创建文章...".into()));

            let token = token_opt.unwrap();

            // 创建文章 payload
            let payload = CreateArticlePayload {
                title: title().trim().to_string(),
                description: description().trim().to_string(),
                body: body().trim().to_string(),
                tag_list: tags(),
            };

            spawn(async move {
                match create_article(&token, payload).await {
                    Some(response) => {
                        // 创建成功，显示创建的文章
                        created_article.set(Some(response.article));
                        message.set(Some("✅ 文章创建成功！".into()));
                        
                        // 清空表单
                        title.set(String::new());
                        description.set(String::new());
                        body.set(String::new());
                        tags.set(Vec::new());
                    }
                    None => {
                        message.set(Some("❌ 创建文章失败，请重试".into()));
                    }
                }
                is_submitting.set(false);
            });
        }
    };

    // 表单提交事件
    let on_submit = {
        let mut handle_create = handle_create.clone();
        move |event: Event<FormData>| {
            event.prevent_default();
            handle_create();
        }
    };

    // 按钮点击事件
    let on_click = {
        let mut handle_create = handle_create.clone();
        move |_| {
            handle_create();
        }
    };

  rsx! {
        div { class: "editor-page",
            div { class: "container page",
                div { class: "row",
                    div { class: "col-md-10 offset-md-1 col-xs-12",
                        h1 { class: "text-xs-center", "Create Article" }
                        
                        // 显示消息
                        if let Some(msg) = &*message.read() {
                            ul { class: "error-messages",
                                li { "{msg}" }
                            }
                        }

                        // 如果文章创建成功，显示创建的文章
                        if let Some(article) = created_article.read().as_ref() {
                            div { class: "article-preview",
                                div { class: "article-meta",
                                    h1 { "{article.title}" }
                                    p { class: "article-description", "{article.description}" }
                                    
                                    div { class: "article-info",
                                        span { class: "author", "作者：{article.author.username}" }
                                        span { class: "date", "创建时间：{article.created_at}" }
                                    }
                                    
                                    if !article.tag_list.is_empty() {
                                        div { class: "tag-list",
                                            for tag in &article.tag_list {
                                                span { class: "tag-default tag-pill tag-outline", "{tag}" }
                                            }
                                        }
                                    }
                                }
                                
                                // 注意：Article 结构体中没有 body 字段
                                // 如果需要显示文章内容，需要单独获取文章详情
                                div { class: "article-info",
                                    p { "文章已成功创建！" }
                                    p { "Slug: {article.slug}" }
                                    if article.favorites_count > 0 {
                                        p { "收藏数: {article.favorites_count}" }
                                    }
                                }

                                hr {}
                                
                                button {
                                    class: "btn btn-outline-primary",
                                    onclick: move |_| {
                                        created_article.set(None);
                                        message.set(None);
                                    },
                                    "创建新文章"
                                }
                            }
                        } else {
                            // 创建文章表单
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
                                            placeholder: "Enter tags (press Enter to add)",
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
                                            "Publishing..."
                                        } else {
                                            "Publish Article"
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