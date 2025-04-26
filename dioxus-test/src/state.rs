use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Light
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AppState {
    pub counter: i32,
    pub theme: Theme,
    pub version: String,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            counter: 0,
            theme: Theme::Light,
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

// Create a hook to use the app state
pub fn use_app_state() -> Signal<AppState> {
    use_context_provider(|| Signal::new(AppState::new()))
}

// Function to toggle the theme
pub fn toggle_theme(mut state: Signal<AppState>) {
    let mut app_state = state.write();
    app_state.theme = match app_state.theme {
        Theme::Light => Theme::Dark,
        Theme::Dark => Theme::Light,
    };
}
