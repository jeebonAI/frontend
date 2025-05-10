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
            theme: get_initial_theme(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

// Get the initial theme based on local storage or system preference
fn get_initial_theme() -> Theme {
    // For web platform, try to get theme from local storage or system preference
    #[cfg(feature = "web")]
    {
        use web_sys::{window};

        // Try to get theme from local storage
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(theme)) = storage.get_item("theme") {
                    return match theme.as_str() {
                        "dark" => Theme::Dark,
                        _ => Theme::Light,
                    };
                }
            }

            // If no theme in local storage, check system preference
            if let Some(media_query) = window
                .match_media("(prefers-color-scheme: dark)")
                .ok()
                .flatten()
            {
                if media_query.matches() {
                    return Theme::Dark;
                }
            }
        }
    }

    // For mobile platforms, we could add platform-specific code here
    // #[cfg(feature = "mobile")]
    // {
    //     // Mobile-specific theme detection could be added here
    // }

    // Default to light theme if not on web or if preferences can't be determined
    Theme::Light
}

// Save theme to local storage
fn save_theme_preference(theme: &Theme) {
    // For web platform, save to local storage
    #[cfg(feature = "web")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item(
                    "theme",
                    match theme {
                        Theme::Dark => "dark",
                        Theme::Light => "light",
                    },
                );
            }
        }
    }

    // For mobile platforms, we could add platform-specific code here
    #[cfg(feature = "mobile")]
    {
        // Mobile-specific theme persistence could be added here
        tracing::info!("Mobile Specific Theme persistence code placeholder.");
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

    // Save the theme preference
    save_theme_preference(&app_state.theme);

    // For web platform, update the HTML document theme attribute for immediate effect
    #[cfg(feature = "web")]
    {
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            if let Some(html) = document.document_element() {
                let theme_value = match app_state.theme {
                    Theme::Dark => "dark",
                    Theme::Light => "light",
                };
                html.set_attribute("data-bs-theme", theme_value).ok();
            }
        }
    }

    // For mobile platforms, we could add platform-specific code here
    // #[cfg(feature = "mobile")]
    // {
    //     // Mobile-specific theme application could be added here
    // }
}
