// src/api/profile.rs

use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Profile {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    #[serde(rename = "following")]
    pub following: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProfileResponse {
    pub profile: Profile,
}

/// 根据用户名获取用户资料
pub async fn fetch_profile(username: &str, token: Option<&str>) -> Option<ProfileResponse> {
    let client = Client::new();
    let url = format!("http://localhost:8000/api/profiles/{}", username);

    let mut request_builder = client.get(&url);

    // Only add the Authorization header if a token is provided
    if let Some(t) = token {
        request_builder = request_builder.header("Authorization", format!("Token {}", t));
    }

    // Use `ok()?` to convert `Result` to `Option` and propagate `None` on error
    // `error_for_status()` ensures that only 2xx responses are considered successful
    let resp = request_builder
        .send()
        .await
        .ok()? // Handles network errors, timeouts, etc.
        .error_for_status()
        .ok()? // Handles non-2xx HTTP status codes
        .json::<ProfileResponse>()
        .await
        .ok()?; // Handles JSON deserialization errors

    Some(resp)
}