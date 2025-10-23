// src/stores/app_state.rs
use dioxus::prelude::*;
use serde::Serialize;
use serde::Deserialize;
use serde_json;

/// Represents the authentication status of the user.
#[derive(Debug,Clone, Copy, PartialEq)]
pub enum AuthStatus {
    LoggedIn,
    LoggedOut,
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub token: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}


#[derive(Debug, Clone, PartialEq)]
pub struct AppState {
    pub auth_status: Signal<AuthStatus>,
    pub user: Signal<Option<User>>, // 使用 Option<User> 来表示用户是否登录
    // ... 其他全局状态，例如错误信息等
}




impl AppState {
pub fn new() -> Self {
    let initial_user: Option<User> = web_sys::window()
        .and_then(|win| win.local_storage().ok().flatten()) // 先获取 localStorage
        .and_then(|store| store.get_item("user").ok().flatten()) // 尝试获取 user 项
        .and_then(|user_json| serde_json::from_str(&user_json).ok()); // 解析 JSON

    let initial_auth_status = if initial_user.is_some() {
        AuthStatus::LoggedIn
    } else {
        AuthStatus::LoggedOut
    };

    Self {
        auth_status: Signal::new(initial_auth_status),
        user: Signal::new(initial_user),
    }
}

    // Set the user and save to localStorage
pub fn set_user(&mut self, user: User) {
    if let Ok(user_json) = serde_json::to_string(&user) {
        if let Some(local_storage) = web_sys::window()
            .and_then(|win| win.local_storage().ok().flatten()) {
                let _ = local_storage.set_item("user", &user_json);
        }
    }
    self.user.set(Some(user));
    self.auth_status.set(AuthStatus::LoggedIn);
}
    // Clear the user and remove from localStorage
pub fn clear_user(&mut self) {
    if let Some(local_storage) = web_sys::window()
        .and_then(|win| win.local_storage().ok().flatten()) {
            let _ = local_storage.remove_item("user");
    }
    self.user.set(None);
    self.auth_status.set(AuthStatus::LoggedOut);
}


}




