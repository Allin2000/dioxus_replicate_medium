// src/views/profile.rs
use std::convert::TryInto;
use dioxus::prelude::*;
use crate::Route;
use crate::api::profile::{fetch_profile, follow_user, unfollow_user, Profile as ProfileData};
use crate::stores::app_state::AppState;
use crate::api::article::{fetch_articles, favorite_article, unfavorite_article, ArticleQuery};

#[component]
pub fn Profile(username: String) -> Element {
    let app_state = use_context::<Signal<AppState>>();
    let current_route = use_route();

    let username = match current_route {
        Route::Profile { username } => Some(username),
        _ => None,
    };

    let profile_state = use_signal(|| None::<ProfileData>);
    let mut tab_state = use_signal(|| "my_articles".to_string());
    let mut current_page = use_signal(|| 0); // 👈 新增页码 signal
    const ARTICLES_PER_PAGE: usize = 10;

    use_effect({
        let username = username.clone();
        let mut profile_state = profile_state.clone();
        let app_state = app_state.clone();

        move || {
            let token = app_state.read().user.read().as_ref().map(|u| u.token.clone());
            let username = username.clone();
            spawn(async move {
                if let Some(u) = username {
                    if let Some(response) = fetch_profile(&u, token.as_deref()).await {
                        profile_state.set(Some(response.profile));
                    }
                }
            });
        }
    });

    let token = app_state.read().user.read().as_ref().map(|u| u.token.clone());
    let tab = tab_state.read().clone();
    let profile = profile_state.read().clone();

    let articles_query = use_signal(|| (tab_state(), profile_state(), current_page())); // 👈 加入 current_page

    let articles = use_resource({
        let app_state = app_state.clone();
        let articles_query = articles_query.clone();

        move || {
            let (tab, profile_opt, page) = articles_query();
            let token = app_state.read().user.read().as_ref().map(|u| u.token.clone());

            async move {
                if let Some(profile) = profile_opt {
                    let query = ArticleQuery {
                        author: if tab == "my_articles" {
                            Some(profile.username.clone())
                        } else {
                            None
                        },
                        favorited: if tab == "favorited_articles" {
                            Some(profile.username.clone())
                        } else {
                            None
                        },
                        limit: Some(ARTICLES_PER_PAGE.try_into().unwrap()),
                        offset: Some((page * ARTICLES_PER_PAGE).try_into().unwrap()),
                        ..Default::default()
                    };

                    fetch_articles(query).await.map(|res| res.articles)
                } else {
                    None
                }
            }
        }
    });

    use_effect({
        let mut articles_query = articles_query.clone();
        let tab_state = tab_state.clone();
        let profile_state = profile_state.clone();
        let current_page = current_page.clone();
        move || {
            articles_query.set((tab_state(), profile_state(), current_page()));
        }
    });

    let toggle_follow = {
        let mut profile_state = profile_state.clone();
        let token = app_state.read().user.read().as_ref().map(|u| u.token.clone());
        move |_| {
            let profile = profile_state.read();
            if let Some(profile) = profile.as_ref() {
                let username = profile.username.clone();
                let is_following = profile.following;
                let token = token.clone();
                let mut updated_profile = profile.clone();

                spawn(async move {
                    if let Some(token) = token {
                        let updated_profile_response = if is_following {
                            unfollow_user(&username, &token).await
                        } else {
                            follow_user(&username, &token).await
                        };

                        if let Some(new_profile_response) = updated_profile_response {
                            updated_profile.following = new_profile_response.profile.following;
                            profile_state.set(Some(updated_profile));
                        }
                    }
                });
            }
        }
    };

    let Some(profile) = profile_state() else {
        return rsx!(div { "Loading..." });
    };

    let logged_in_username = app_state.read().user.read().as_ref().map(|u| u.username.clone());
    let is_own_profile = Some(profile.username.clone()) == logged_in_username;

    rsx! {
        div { class: "profile-page",
            div { class: "user-info",
                div { class: "container",
                    div { class: "row",
                        div { class: "col-xs-12 col-md-10 offset-md-1",
                            img { class: "user-img" }
                            h4 { "{profile.username}" }
                            if let Some(bio) = &profile.bio {
                                p { "{bio}" }
                            }

                            if is_own_profile {
                                button {
                                    class: "btn btn-sm btn-outline-secondary action-btn",
                                    onclick: move |_| {use_navigator().push(Route::Settings {});},
                                    i { class: "ion-gear-a" }
                                    " Edit Profile Settings"
                                }
                            } else {
                                button {
                                    class: "btn btn-sm btn-outline-secondary action-btn",
                                    onclick: toggle_follow,
                                    i { class: "ion-plus-round" }
                                    if profile.following {
                                        " Unfollow {profile.username}"
                                    } else {
                                        " Follow {profile.username}"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            div { class: "container",
                div { class: "row",
                    div { class: "col-xs-12 col-md-10 offset-md-1",
                        div { class: "articles-toggle",
                            ul { class: "nav nav-pills outline-active",
                                li { class: "nav-item",
                                    a {
                                        class: if tab_state() == "my_articles" { "nav-link active" } else { "nav-link" },
                                        onclick: move |_| {
                                            tab_state.set("my_articles".to_string());
                                            current_page.set(0); // 👈 切换 tab 时重置页码
                                        },
                                        "My Articles"
                                    }
                                }
                                li { class: "nav-item",
                                    a {
                                        class: if tab_state() == "favorited_articles" { "nav-link active" } else { "nav-link" },
                                        onclick: move |_| {
                                            tab_state.set("favorited_articles".to_string());
                                            current_page.set(0); // 👈 切换 tab 时重置页码
                                        },
                                        "Favorited Articles"
                                    }
                                }
                            }
                        }

                        match articles.value()() {
                            Some(Some(articles)) if articles.is_empty() => rsx!(p { "No articles are here... yet." }),
                            Some(Some(articles)) => rsx! {
                                for article in articles.iter() {
                                    div { class: "article-preview", key: "{article.slug}",
                                        div { class: "article-meta",
                                            a { href: format!("/profile/{}", article.author.username),
                                                img { src: "{article.author.image}" }
                                            }
                                            div { class: "info",
                                                a { class: "author", href: format!("/profile/{}", article.author.username),
                                                    "{article.author.username}"
                                                }
                                                span { class: "date", "{article.created_at}" }
                                            }
                                            button { class: "btn btn-outline-primary btn-sm pull-xs-right",
                                                i { class: "ion-heart" }
                                                " {article.favorites_count} "
                                            }
                                        }
                                        a { class: "preview-link", href: format!("/article/{}", article.slug),
                                            h1 { "{article.title}" }
                                            p { "{article.description}" }
                                            span { "Read more..." }
                                            ul { class: "tag-list",
                                                for tag in article.tag_list.iter() {
                                                    li { class: "tag-default tag-pill tag-outline", key: "{tag}", "{tag}" }
                                                }
                                            }
                                        }
                                    }
                                }

                                // 👇 分页按钮
                    div { class: "pagination",
                    button {
                    class: "btn btn-outline-secondary btn-sm",
                    disabled: *current_page.read() == 0,
                onclick: {
                    let mut current_page = current_page.clone();
                move |_| {
                    let page = *current_page.read();
                    if page > 0 {
                        current_page.set(page - 1);
                    }
                }
            },
                "Prev"
                }
            span { style: "margin: 0 10px;", "Page {current_page() + 1}" }
            button {
                    class: "btn btn-outline-secondary btn-sm",
                disabled: articles.len() < ARTICLES_PER_PAGE,
                onclick: {
                    let mut current_page = current_page.clone();
                    move |_| {
                        let page = *current_page.read();
                        current_page.set(page + 1);
                    }
                },
                "Next"
                    }
                        }
                            },
                            Some(None) => rsx!(p { "Failed to load articles." }),
                            None => rsx!(p { "Loading..." }),
                        }
                    }
                }
            }
        }
    }
}




