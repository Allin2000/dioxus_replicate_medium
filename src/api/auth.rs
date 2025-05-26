// src/api/auth.rs
use reqwest::{Client};
use serde::{Deserialize, Serialize};



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
pub struct UserRegi {
    pub username: String,
    pub email: String,
  
    #[serde(default)]
    pub bio: Option<String>,

    #[serde(default)]
    pub image: Option<String>,
    
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserAuthResponse {
    pub user: User,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserRegiResponse {
    pub user: UserRegi,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateUserPayload {
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    pub bio: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateUserRequest {
    pub user: UpdateUserPayload,
}




/// 注册用户
pub async fn register_user(username: &str, email: &str, password: &str) -> Option<UserAuthResponse> {
    let client = Client::new();

    let payload = RegisterUserRequest {
        user: RegisterUserPayload {
            username: username.to_string(),
            email: email.to_string(),
            password: password.to_string(),

        },
    };


    let response = client
        .post("http://127.0.0.1:8000/api/users")
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


    let response = client
        .post("http://127.0.0.1:8000/api/users/login")
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




/// 获取当前用户信息（需要认证）
pub async fn get_current_user(token: &str) -> Option<UserAuthResponse> {
    let client = Client::new();

    let response = client
        .get("http://127.0.0.1:8000/api/user")
        .header("AUTHORIZATION", format!("Token {}", token))
        .header("Accept", "application/json")
        .send()
        .await
        .ok()?
        .json::<UserAuthResponse>()
        .await
        .ok()?;

    Some(response)
}


/// 更新当前用户信息
pub async fn update_user(token: &str, update: UpdateUserPayload) -> Option<UserAuthResponse> {
    let client = Client::new();

    let request = UpdateUserRequest { user: update };

    let response = client
        .put("http://127.0.0.1:8000/api/user")
        .header("AUTHORIZATION", format!("Token {}", token))
        .header("Accept", "application/json")
        .json(&request)
        .send()
        .await
        .ok()?
        .json::<UserAuthResponse>()
        .await
        .ok()?;

    Some(response)
}
