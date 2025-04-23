use dioxus::prelude::*;
use crate::Route;
use crate::components::AppLogo;

#[component]
pub fn MobileLayout() -> Element {
    // Theme state (light/dark)
    let mut theme_dark = use_signal(|| false); // Default to light theme

    // Toggle theme function
    let toggle_theme = move |_| {
        let current = *theme_dark.read();
        theme_dark.set(!current);

        // Apply dark mode class to body
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");

        if !current {
            // Set dark theme
            body.set_attribute("class", "bg-dark text-light").ok();
        } else {
            // Set light theme
            body.set_attribute("class", "").ok();
        }
    };

    rsx! {
        div { class: "mobile-container",
            // Header
            header { class: "app-header d-flex justify-content-between align-items-center",
                // Logo and brand
                div { class: "d-flex align-items-center",
                    AppLogo {}
                    div { class: "ms-2",
                        span { class: "fw-bold", "DJibon" }
                        span { class: "badge bg-secondary ms-1", "beta" }
                    }
                }

                // Theme toggle button
                button {
                    class: "dark-mode-toggle",
                    onclick: toggle_theme,
                    i { class: if *theme_dark.read() { "bi bi-sun" } else { "bi bi-moon" } }
                }
            }

            // Main content
            main { class: "main-content",
                Outlet::<Route> {}
            }

            // Bottom navigation - styled like the image
            nav { class: "bottom-nav",
                // Home
                div { class: "nav-item",
                    Link {
                        to: Route::Home {},
                        class: "nav-link",
                        active_class: "active",
                        i { class: "bi bi-house-door" }
                        span { "Home" }
                    }
                }

                // Features (Trees)
                div { class: "nav-item",
                    Link {
                        to: Route::Trees {},
                        class: "nav-link",
                        active_class: "active",
                        i { class: "bi bi-star" }
                        span { "Features" }
                    }
                }

                // Pages (Circles)
                div { class: "nav-item",
                    Link {
                        to: Route::Circles {},
                        class: "nav-link",
                        active_class: "active",
                        i { class: "bi bi-heart" }
                        span { "Pages" }
                    }
                }

                // Search (Messages)
                div { class: "nav-item",
                    Link {
                        to: Route::Messages {},
                        class: "nav-link",
                        active_class: "active",
                        i { class: "bi bi-search" }
                        span { "Search" }
                    }
                }

                // Settings
                div { class: "nav-item",
                    Link {
                        to: Route::Settings {},
                        class: "nav-link",
                        active_class: "active",
                        i { class: "bi bi-gear" }
                        span { "Settings" }
                    }
                }
            }
        }
    }
}
