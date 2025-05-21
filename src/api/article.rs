// src/api/article.rs
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct ArticleAuthor {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ArticlePreview {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub tagList: Vec<String>,
    pub createdAt: String,
    pub updatedAt: String,
    pub favorited: bool,
    pub favoritesCount: u32,
    pub author: ArticleAuthor,
}

#[derive(Deserialize, Debug)]
pub struct ArticlesResponse {
    pub articles: Vec<ArticlePreview>,
    pub articlesCount: usize,
}

/// 查询参数
#[derive(Default)]
pub struct ArticleQuery {
    pub tag: Option<String>,
    pub author: Option<String>,
    pub favorited: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

/// 获取文章列表
pub async fn fetch_articles(query: ArticleQuery) -> Option<ArticlesResponse> {
    let client = Client::new();

    let mut req = client.get("http://localhost:8000/api/articles");

    let mut params = vec![];

    if let Some(tag) = query.tag {
        params.push(("tag", tag));
    }
    if let Some(author) = query.author {
        params.push(("author", author));
    }
    if let Some(favorited) = query.favorited {
        params.push(("favorited", favorited));
    }
    if let Some(limit) = query.limit {
        params.push(("limit", limit.to_string()));
    }
    if let Some(offset) = query.offset {
        params.push(("offset", offset.to_string()));
    }

    if !params.is_empty() {
        req = req.query(&params);
    }

    let response = req.send().await.ok()?.json::<ArticlesResponse>().await.ok()?;
    Some(response)
}