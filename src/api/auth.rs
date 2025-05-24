// src/api/auth.rs
use reqwest::{Client, Error as ReqwestError, header::{AUTHORIZATION, CONTENT_TYPE}};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use log::error; // Add this import




#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegisterUserPayload {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginUserPayload {
    pub email: String,
    pub password: String,
}



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginUserRequest {
    pub user: LoginUserPayload,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegisterUserRequest {
    pub user: RegisterUserPayload,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub email: String,
    pub username: String,
    
    #[serde(default)]
    pub bio: Option<String>,

    #[serde(default)]
    pub image: Option<String>,
    
    pub token: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserAuthResponse {
    pub user: User,
}




/// 注册用户
pub async fn register_user(username: &str, email: &str, password: &str) -> Option<UserAuthResponse> {
    let client = Client::new();

    let payload = RegisterUserRequest {
        user: RegisterUserPayload {
            email: email.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        },
    };

    let response = client
        .post("http://127.0.0.1:8000/api/users")
        .header(CONTENT_TYPE, "application/json")
        .header("Accept", "application/json")
        .json(&payload)
        .send()
        .await
        .ok()?
        .json::<UserAuthResponse>()
        .await
        .ok()?;

    Some(response)



}

/// 登录用户
pub async fn login_user(email: &str, password: &str) -> Option<UserAuthResponse> {
    let client = Client::new();

    let payload = LoginUserRequest {
        user: LoginUserPayload {
            email: email.to_string(),
            password: password.to_string(),
      },
    };


    log::info!("login_user called with payload: {:?}", payload);

    // let response = client
    //     .post("http://127.0.0.1:8000/api/users/login")
    //     .header(CONTENT_TYPE, "application/json")
    //     .json(&payload)
    //     .send()
    //     .await
    //     .ok()?
    //     .json::<UserAuthResponse>()
    //     .await
    //     .ok()?;


     let response_result = client
        .post("http://127.0.0.1:8000/api/users/login")
        .header(CONTENT_TYPE, "application/json")
        .json(&payload)
        .send()
        .await;


     let response = match response_result {
        Ok(res) => res,
        Err(e) => {
            log::error!("Reqwest send error in login_user: {:?}", e);
            return None; // 请求发送失败
        }
    };


     let status = response.status();
    let response_text = response.text().await.unwrap_or_else(|e| {
        log::error!("Failed to get response text in login_user: {:?}", e);
        "Failed to get response text".to_string()
    });

    log::info!("API Response Status in login_user: {}", status);
    log::info!("API Raw Response Text in login_user: {}", response_text);

    if status.is_success() {
        match serde_json::from_str::<UserAuthResponse>(&response_text) {
            Ok(user_auth_response) => {
                log::info!("Successfully parsed login response: {:?}", user_auth_response);
                Some(user_auth_response)
            },
            Err(e) => {
                log::error!("Failed to parse JSON response in login_user: {:?}", e);
                return None // JSON 解析失败
            }
        }
    } else {
        log::error!("Login API returned non-success status {}: {}", status, response_text);
        // 这里可以根据实际的错误响应格式，尝试解析出后端返回的错误消息
        return None // 后端返回非成功状态码
    }


    // log::info!("login_user called with payload: {:?}", response);

    // return Some(response)
}

/// 获取当前用户信息（需要认证）
pub async fn get_current_user(token: &str) -> Option<UserAuthResponse> {
    let client = Client::new();

    let response = client
        .get("http://localhost:8000/api/user")
        .header(AUTHORIZATION, format!("Token {}", token))
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await
        .ok()?
        .json::<UserAuthResponse>()
        .await
        .ok()?;

    Some(response)
}
