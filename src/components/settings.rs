use dioxus::prelude::*;
use crate::state::{use_app_state, Theme, toggle_theme};

#[component]
pub fn Settings() -> Element {
    // Get the app state
    let state = use_app_state();

    // Check if the theme is dark
    let is_dark = matches!(state.read().theme, Theme::Dark);

    // Create an event handler for the theme toggle
    let handle_toggle = move |_| {
        toggle_theme(state);
    };

    rsx! {
        div { class: "container mt-2",
            div { class: "card",
                div { class: "card-body",
                            // Theme settings
                            div { class: "mb-4",
                                h5 { "Appearance" }
                                div { class: "form-check form-switch",
                                    input {
                                        class: "form-check-input",
                                        r#type: "checkbox",
                                        id: "darkModeSwitch",
                                        checked: is_dark,
                                        onclick: handle_toggle
                                    }
                                    label {
                                        class: "form-check-label",
                                        r#for: "darkModeSwitch",
                                        "Dark Mode"
                                    }
                                }
                                p { class: "text-muted small", "Switch between light and dark themes." }
                            }

                            // Language settings
                            div { class: "mb-4",
                                h5 { "Language" }
                                select { class: "form-select",
                                    option { value: "en", "English" }
                                    option { value: "es", "Español" }
                                    option { value: "fr", "Français" }
                                    option { value: "de", "Deutsch" }
                                }
                                p { class: "text-muted small", "Select your preferred language for the application interface." }
                            }

                            // Time zone settings
                            div { class: "mb-4",
                                h5 { "Time Zone" }
                                select { class: "form-select",
                                    option { value: "utc", "UTC (Coordinated Universal Time)" }
                                    option { value: "est", "EST (Eastern Standard Time)" }
                                    option { value: "pst", "PST (Pacific Standard Time)" }
                                    option { value: "cet", "CET (Central European Time)" }
                                }
                                p { class: "text-muted small", "Select your time zone for accurate time displays." }
                            }

                            // Accessibility settings
                            div {
                                h5 { "Accessibility" }
                                div { class: "form-check mb-2",
                                    input {
                                        class: "form-check-input",
                                        r#type: "checkbox",
                                        id: "highContrastMode"
                                    }
                                    label {
                                        class: "form-check-label",
                                        r#for: "highContrastMode",
                                        "High Contrast Mode"
                                    }
                                }
                                div { class: "form-check mb-2",
                                    input {
                                        class: "form-check-input",
                                        r#type: "checkbox",
                                        id: "largerText"
                                    }
                                    label {
                                        class: "form-check-label",
                                        r#for: "largerText",
                                        "Larger Text"
                                    }
                                }
                                div { class: "form-check",
                                    input {
                                        class: "form-check-input",
                                        r#type: "checkbox",
                                        id: "reduceMotion"
                                    }
                                    label {
                                        class: "form-check-label",
                                        r#for: "reduceMotion",
                                        "Reduce Motion"
                                    }
                                }
                                p { class: "text-muted small", "Adjust settings to improve accessibility." }
                            }
                        }
                    }
        }
    }
}
