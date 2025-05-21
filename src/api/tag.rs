// src/api/tag.rs
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct TagResponse {
    pub tags: Vec<String>,
}

/// 获取热门标签列表
pub async fn fetch_tags() -> Option<Vec<String>> {
    let client = Client::new();
        let response = client
        .get("http://localhost:8000/api/tags")
        .send()
        .await
        .ok()? // 请求失败返回 None
        .json::<TagResponse>()
        .await
        .ok()?; // 解析失败返回 None

    Some(response.tags)
}


