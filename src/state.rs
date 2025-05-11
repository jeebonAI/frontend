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

// Determine theme from local storage or system preference
fn get_initial_theme() -> Theme {
    #[cfg(feature = "web")]
    {
        use web_sys::window;
        if let Some(win) = window() {
            // Try local storage first
            match win.local_storage() {
                Ok(Some(storage)) => {
                    log::info!("Accessed local storage successfully.");
                    match storage.get_item("theme") {
                        Ok(Some(theme_str)) => {
                            log::info!("Found theme in local storage: {}", theme_str);
                            return match theme_str.as_str() {
                                "dark" => Theme::Dark,
                                _ => Theme::Light,
                            };
                        }
                        Ok(None) => {
                            log::info!("No theme found in local storage.");
                        }
                        Err(e) => {
                            log::error!("Error getting item from local storage: {:?}", e);
                        }
                    }
                }
                Ok(None) => log::warn!("Local storage is disabled or not available (returned None)."),
                Err(e) => log::error!("Error accessing local storage (e.g., permission denied): {:?}", e),
            }

            // If no theme in local storage, check system preference
            if let Some(media_query) = win
                .match_media("(prefers-color-scheme: dark)")
                .ok()
                .flatten()
            {
                if media_query.matches() {
                    log::info!("System preference is dark theme.");
                    return Theme::Dark;
                }
            }
            log::info!("No theme in local storage, and system preference is not dark (or not detectable).");
        } else {
            log::warn!("web_sys::window() returned None, cannot get initial theme from storage/system.");
        }
    }
    // Default to light theme if not on web or if preferences can't be determined
    log::info!("Defaulting to light theme for get_initial_theme.");
    Theme::Light
}

// Save theme to local storage
fn save_theme_preference(
  theme: &Theme,
) {
    #[cfg(feature = "web")]
    {
        use web_sys::window;
        if let Some(win) = window() {
            match win.local_storage() {
                Ok(Some(storage)) => {
                    let theme_str = match theme { // Use `theme` here as it's under cfg(web)
                        Theme::Dark => "dark",
                        Theme::Light => "light",
                    };
                    match storage.set_item("theme", theme_str) {
                        Ok(_) => log::info!("Saved theme preference to local storage: {}", theme_str),
                        Err(e) => log::error!("Failed to save theme to local storage: {:?}", e),
                    }
                }
                Ok(None) => log::warn!("Local storage is disabled or not available, cannot save theme."),
                Err(e) => log::error!("Error accessing local storage for saving: {:?}", e),
            }
        } else {
            log::warn!("web_sys::window() returned None, cannot save theme preference.");
        }
    }
    // For non-web (e.g., mobile), _theme is unused, and no action is taken.
}

// REMOVE apply_dynamic_theme as it was part of the problematic unified approach
// pub fn apply_dynamic_theme(mut state: Signal<AppState>) { ... }

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

    // Save the new theme preference
    save_theme_preference(&app_state.theme);

    // For web platform, update the HTML document theme attribute for immediate effect
    // This was likely how it worked before for the web.
    #[cfg(feature = "web")]
    {
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            if let Some(html) = document.document_element() {
                let theme_value = match app_state.theme {
                    Theme::Dark => "dark",
                    Theme::Light => "light",
                };
                match html.set_attribute("data-bs-theme", theme_value) {
                    Ok(_) => log::info!("toggle_theme: Applied data-bs-theme: {}", theme_value),
                    Err(e) => log::error!("toggle_theme: Failed to set data-bs-theme: {:?}", e),
                }
            }
        }
    }
}
