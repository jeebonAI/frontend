use dioxus::prelude::*;
use crate::Route;
use crate::state::{use_app_state, Theme};

#[component]
pub fn BottomNav() -> Element {
    // Get the app state
    let state = use_app_state();

    // Get the current route
    let route = use_route::<Route>();

    // Helper function to determine if a link is active
    let _is_active = |r: &Route| -> bool {
        std::mem::discriminant(r) == std::mem::discriminant(&route)
    };

    // Determine data-bs-theme attribute based on theme
    let theme_attr = match state.read().theme {
        Theme::Light => "light",
        Theme::Dark => "dark",
    };

    rsx! {
        nav {
            class: "navbar fixed-bottom border-top pt-2",
            "data-bs-theme": theme_attr,
            div {
                class: "container-fluid px-0",
                div { class: "row w-100 text-center g-0",
                    div { class: "col",
                        Link {
                            to: Route::About {},
                            class: "nav-link d-flex flex-column align-items-center",
                            active_class: "active fw-bold text-primary",
                            i { class: "bi bi-person fs-4" }
                            span { class: "small", "Profile" }
                        }
                    }
                    div { class: "col",
                        Link {
                            to: Route::Settings {},
                            class: "nav-link d-flex flex-column align-items-center",
                            active_class: "active fw-bold text-primary",
                            i { class: "bi bi-chat-dots fs-4" }
                            span { class: "small", "Comms" }
                        }
                    }
                    div { class: "col",
                        Link {
                            to: Route::ErrorTest {},
                            class: "nav-link d-flex flex-column align-items-center",
                            active_class: "active fw-bold text-primary",
                            i { class: "bi bi-people fs-4" }
                            span { class: "small", "Circles" }
                        }
                    }
                    div { class: "col",
                        Link {
                            to: Route::Home {},
                            class: "nav-link d-flex flex-column align-items-center",
                            active_class: "active fw-bold text-primary",
                            i { class: "bi bi-diagram-3 fs-4" }
                            span { class: "small", "Trees" }
                        }
                    }
                    div { class: "col",
                        Link {
                            to: Route::Settings {},
                            class: "nav-link d-flex flex-column align-items-center",
                            active_class: "active fw-bold text-primary",
                            i { class: "bi bi-gear fs-4" }
                            span { class: "small", "Settings" }
                        }
                    }
                }
            }
        }
    }
}
