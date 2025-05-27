// src/api/article.rs
use reqwest::Client;
use serde::{Deserialize,Serialize};

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Article {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
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



/// 获取单篇文章详情
/// `token`: 可选的认证 token，如果提供，后端会根据用户身份返回收藏/关注状态
pub async fn fetch_article_by_slug(slug: &str, token: Option<String>) -> Option<SingleArticleResponse> {
    let client = Client::new();
    let url = format!("http://localhost:8000/api/articles/{}", slug);

    let mut request_builder = client.get(&url)
        .header("Accept", "application/json");

    // 如果提供了 token，则将其添加到请求头中
    if let Some(t) = token {
        request_builder = request_builder.header("Authorization", format!("Token {}", t)); // RealWorld API 使用 "Token " 前缀
    }

    let response = request_builder
        .send()
        .await
        .ok()?
        .json::<SingleArticleResponse>()
        .await
        .ok()?;

    Some(response) // 返回 SingleArticleResponse，Article 组件再从中提取 Article
}

// 请求 Payload
#[derive(Deserialize,Serialize, Debug, Clone)]
pub struct CreateArticleRequest {
    pub article: CreateArticlePayload,
}

#[derive(Deserialize,Serialize, Debug, Clone)]
pub struct CreateArticlePayload {
    pub title: String,
    pub description: String,
    pub body: String,
    #[serde(rename = "tagList")]
    pub tag_list: Vec<String>,
}

// 响应结构
#[derive(Deserialize, Debug, Clone)]
pub struct SingleArticleResponse {
    pub article: Article,
}

// 创建文章函数
pub async fn create_article(token: &str, payload: CreateArticlePayload) -> Option<SingleArticleResponse> {
    let client = Client::new();
    let request_body = CreateArticleRequest { article: payload };

    let response = client
        .post("http://localhost:8000/api/articles")
        .header("Authorization", format!("Token {}", token))
        .header("Accept", "application/json")
        .json(&request_body)
        .send()
        .await
        .ok()?
        .json::<SingleArticleResponse>()
        .await
        .ok()?;

    Some(response)
}




// 请求 Payload（字段都是可选的）
#[derive(Serialize, Debug, Clone, Default)]
pub struct UpdateArticleRequest {
    pub article: UpdateArticlePayload,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct UpdateArticlePayload {
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
}

// 使用 slug 作为路径参数
pub async fn update_article(token: &str, slug: &str, payload: UpdateArticlePayload) -> Option<SingleArticleResponse> {
    let client = Client::new();
    let url = format!("http://localhost:8000/api/articles/{}", slug);
    let request_body = UpdateArticleRequest { article: payload };

    let response = client
        .put(&url)
        .header("Authorization", format!("Token {}", token))
        .header("Accept", "application/json")
        .json(&request_body)
        .send()
        .await
        .ok()?
        .json::<SingleArticleResponse>()
        .await
        .ok()?;

    Some(response)
}



pub async fn delete_article(token: &str, slug: &str) -> bool {
    let client = Client::new();
    let url = format!("http://localhost:8000/api/articles/{}", slug);

    let response = client
        .delete(&url)
        .header("Authorization", format!("Token {}", token))
        .send()
        .await;

    response.map(|r| r.status().is_success()).unwrap_or(false)
}