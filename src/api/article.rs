// src/api/article.rs
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;


#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Article {
    pub slug: String,
    pub title: String,
    pub description: String,
    #[serde(rename = "tagList")] // 匹配 JSON 字段名
    pub tag_list: Vec<String>,
    #[serde(rename = "createdAt")] // 匹配 JSON 字段名
    pub created_at: String,
    #[serde(rename = "updatedAt")] // 匹配 JSON 字段名
    pub updated_at: String,
    #[serde(rename = "favorited")] // 匹配 JSON 字段名
    pub is_favorited: bool,
    #[serde(rename = "favoritesCount")] // 匹配 JSON 字段名
    pub favorites_count: u32,
    pub author: Author,
}


// RealWorld API 的 Author 结构，注意 bio 可能为 null，用 Option<String> 更安全
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Author {
    pub username: String,
    // pub bio: String, // 如果 API 返回 null 会panic，建议改为 Option<String>
    pub bio: Option<String>, // 建议改为 Option<String>
    pub image: String,
    #[serde(rename = "following")] // 匹配 JSON 字段名
    pub is_following: bool,
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
    pub author: Author,
}



// RealWorld API 获取文章列表的响应格式
#[derive(Debug, Deserialize, Clone)] // Added Clone
pub struct ArticlesResponse {
    pub articles: Vec<Article>,
    #[serde(rename = "articlesCount")] // 匹配 JSON 字段名
    pub articles_count: usize, // 或 u32
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