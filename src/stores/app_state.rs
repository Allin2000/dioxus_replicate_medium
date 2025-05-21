use dioxus::prelude::*;

/// Represents the authentication status of the user.
#[derive(Clone, Copy, PartialEq)] // Derive necessary traits for use with Signals and contexts
pub enum AuthStatus {
    LoggedIn,
    LoggedOut,
}

/// The global application state.
/// Holds reactive data that needs to be accessed across different parts of the app.
#[derive(Clone, PartialEq)] // Needs Clone and PartialEq for context provision
pub struct AppState {
    pub auth_status: Signal<AuthStatus>,
    // In a real application, you might add:
    // pub user_data: Signal<Option<User>>, // To store authenticated user details
    // pub auth_token: Signal<Option<String>>, // To store JWT token
}

impl AppState {
    /// Creates a new instance of the AppState, defaulting to `LoggedOut`.
    pub fn new() -> Self {
        Self {
            auth_status: Signal::new(AuthStatus::LoggedOut), // User starts as logged out
        }
    }
}