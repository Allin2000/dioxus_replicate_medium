use dioxus::prelude::*;
use dioxus_signals::GlobalSignal; // Import GlobalSignal

/// Represents the authentication status of the user.
#[derive(Clone, Copy, PartialEq)]
pub enum AuthStatus {
    LoggedIn,
    LoggedOut,
}

// Define the global AppState signal
// This will be initialized once when first accessed
pub static APP_STATE: GlobalSignal<AppState> = GlobalSignal::new(|| AppState::new());

/// The global application state.
/// Holds reactive data that needs to be accessed across different parts of the app.
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

// Define the global HomeState signal
// This will be initialized once when first accessed
pub static HOME_STATE: GlobalSignal<HomeState> = GlobalSignal::new(|| HomeState::new());

#[derive(Debug, Clone, PartialEq)] // PartialEq is needed for GlobalSignal to detect changes
pub struct HomeState {
    pub has_user_clicked_feed_tab: Signal<bool>, // This remains a regular Signal within the GlobalSignal
}

impl HomeState {
    pub fn new() -> Self {
        Self {
            // Default to false, login status will implicitly determine "Your Feed" visibility
            has_user_clicked_feed_tab: Signal::new(false),
        }
    }
}
