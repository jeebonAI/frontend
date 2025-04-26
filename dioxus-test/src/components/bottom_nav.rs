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

    // Define a custom enum to represent the bottom nav items
    #[allow(dead_code)]
    enum BottomNavItem {
        Profile,
        Comms,
        Circles,
        Trees,
        Settings,
    }

    // Determine which bottom nav item is active based on the current route
    let active_item = match route {
        Route::Profile {} => BottomNavItem::Profile,
        Route::Comms {} => BottomNavItem::Comms,
        Route::Circles {} => BottomNavItem::Circles,
        Route::Tree {} => BottomNavItem::Trees,
        Route::Settings {} => BottomNavItem::Settings,
        _ => BottomNavItem::Trees, // Default to Trees for any other route
    };

    // Check which item is active
    let is_profile = matches!(active_item, BottomNavItem::Profile);
    let is_comms = matches!(active_item, BottomNavItem::Comms);
    let is_circles = matches!(active_item, BottomNavItem::Circles);
    let is_trees = matches!(active_item, BottomNavItem::Trees);
    let is_settings = matches!(active_item, BottomNavItem::Settings);

    rsx! {
        nav {
            class: "navbar fixed-bottom border-top pt-2 bg-white shadow-sm",
            "data-bs-theme": theme_attr,
            div {
                class: "container-fluid px-0",
                div { class: "row w-100 text-center g-0",
                    div { class: "col",
                        Link {
                            to: Route::Profile {},
                            class: if is_profile { "nav-link d-flex flex-column align-items-center active text-primary" } else { "nav-link d-flex flex-column align-items-center" },
                            i { class: "bi bi-person fs-4" }
                            span { class: "small", "Profile" }
                        }
                    }
                    div { class: "col",
                        Link {
                            to: Route::Comms {},
                            class: if is_comms { "nav-link d-flex flex-column align-items-center active text-primary" } else { "nav-link d-flex flex-column align-items-center" },
                            i { class: "bi bi-chat-dots fs-4" }
                            span { class: "small", "Comms" }
                        }
                    }
                    div { class: "col",
                        Link {
                            to: Route::Circles {},
                            class: if is_circles { "nav-link d-flex flex-column align-items-center active text-primary" } else { "nav-link d-flex flex-column align-items-center" },
                            i { class: "bi bi-people fs-4" }
                            span { class: "small", "Circles" }
                        }
                    }
                    div { class: "col",
                        Link {
                            to: Route::Tree {},
                            class: if is_trees { "nav-link d-flex flex-column align-items-center active text-primary" } else { "nav-link d-flex flex-column align-items-center" },
                            i { class: "bi bi-diagram-3 fs-4" }
                            span { class: "small", "Trees" }
                        }
                    }
                    div { class: "col",
                        Link {
                            to: Route::Settings {},
                            class: if is_settings { "nav-link d-flex flex-column align-items-center active text-primary" } else { "nav-link d-flex flex-column align-items-center" },
                            i { class: "bi bi-gear fs-4" }
                            span { class: "small", "Settings" }
                        }
                    }
                }
            }
        }
    }
}
