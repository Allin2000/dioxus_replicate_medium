use dioxus::prelude::*;

/// Represents the authentication status of the user.
#[derive(Clone, Copy, PartialEq)]
pub enum AuthStatus {
    LoggedIn,
    LoggedOut,
}




#[derive(Clone, PartialEq)]
pub struct AppState {
    pub auth_status: Signal<AuthStatus>, // This remains a regular Signal within the GlobalSignal
}

impl AppState {
    /// Creates a new instance of the AppState, defaulting to `LoggedOut`.
    pub fn new() -> Self {
        Self {
            auth_status: Signal::new(AuthStatus::LoggedIn), // User starts as logged out
        }
    }
}
