use dioxus::prelude::*;
use crate::Route;
use crate::state::{use_app_state, Theme};

#[component]
pub fn NavBar() -> Element {
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
        // Top navbar with toggle button
        nav {
            class: "navbar navbar-expand-lg border-bottom",
            "data-bs-theme": theme_attr,
            div {
                class: "container-fluid",
                Link {
                    to: Route::Home {},
                    class: "navbar-brand d-flex align-items-center",
                    // Tree of life logo
                    svg {
                        width: "30",
                        height: "30",
                        view_box: "0 0 24 24",
                        fill: "currentColor",
                        class: "me-2",

                        // Tree of life logo
                        path {
                            d: "M12 2c0 0-9 6.5-9 13c0 4.4 4 7 9 7s9-2.6 9-7C21 8.5 12 2 12 2zm0 18c-3.9 0-7-1.7-7-5c0-4.9 7-10.5 7-10.5s7 5.6 7 10.5C19 18.3 15.9 20 12 20z"
                        }
                        path {
                            d: "M12 6.5c0 0-5 3.5-5 7.5c0 2.5 2.2 4 5 4s5-1.5 5-4C17 10 12 6.5 12 6.5zm0 9.5c-1.9 0-3-0.9-3-2c0-2.2 3-4.8 3-4.8s3 2.6 3 4.8C15 15.1 13.9 16 12 16z"
                        }
                        circle {
                            cx: "12",
                            cy: "13",
                            r: "1.5"
                        }
                    }
                    "DJibon"
                }
                button {
                    class: "navbar-toggler",
                    r#type: "button",
                    "data-bs-toggle": "offcanvas",
                    "data-bs-target": "#sidebarMenu",
                    "aria-controls": "sidebarMenu",
                    "aria-expanded": "false",
                    "aria-label": "Toggle navigation",
                    span { class: "navbar-toggler-icon" }
                }
            }
        }

        // Sidebar offcanvas menu
        div {
            class: "offcanvas offcanvas-start",
            "data-bs-theme": theme_attr,
            id: "sidebarMenu",
            tabindex: "-1",
            "aria-labelledby": "sidebarMenuLabel",

            // Offcanvas header
            div { class: "offcanvas-header",
                div { class: "d-flex align-items-center",
                    svg {
                        width: "30",
                        height: "30",
                        view_box: "0 0 24 24",
                        fill: "currentColor",
                        class: "me-2",

                        // Tree of life logo
                        path {
                            d: "M12 2c0 0-9 6.5-9 13c0 4.4 4 7 9 7s9-2.6 9-7C21 8.5 12 2 12 2zm0 18c-3.9 0-7-1.7-7-5c0-4.9 7-10.5 7-10.5s7 5.6 7 10.5C19 18.3 15.9 20 12 20z"
                        }
                        path {
                            d: "M12 6.5c0 0-5 3.5-5 7.5c0 2.5 2.2 4 5 4s5-1.5 5-4C17 10 12 6.5 12 6.5zm0 9.5c-1.9 0-3-0.9-3-2c0-2.2 3-4.8 3-4.8s3 2.6 3 4.8C15 15.1 13.9 16 12 16z"
                        }
                        circle {
                            cx: "12",
                            cy: "13",
                            r: "1.5"
                        }
                    }
                    h5 { class: "offcanvas-title mb-0", id: "sidebarMenuLabel", "DJibon" }
                }
                button {
                    r#type: "button",
                    class: "btn-close",
                    "data-bs-dismiss": "offcanvas",
                    "aria-label": "Close"
                }
            }

            // Offcanvas body
            div { class: "offcanvas-body d-flex flex-column h-100 p-0 overflow-hidden",
                // Navigation menu - make it scrollable if needed
                div { class: "nav-container overflow-auto flex-grow-1 p-3",
                    ul {
                        class: "nav nav-pills flex-column",
                        li {
                            class: "nav-item",
                            Link {
                                to: Route::Home {},
                                class: "nav-link",
                                active_class: "active",
                                "data-bs-dismiss": "offcanvas",
                                i { class: "bi bi-house-door me-2" }
                                "Home"
                            }
                        }
                        li {
                            class: "nav-item",
                            Link {
                                to: Route::About {},
                                class: "nav-link",
                                active_class: "active",
                                "data-bs-dismiss": "offcanvas",
                                i { class: "bi bi-person me-2" }
                                "Profile"
                            }
                        }
                        li {
                            class: "nav-item",
                            Link {
                                to: Route::Settings {},
                                class: "nav-link",
                                active_class: "active",
                                "data-bs-dismiss": "offcanvas",
                                i { class: "bi bi-chat-dots me-2" }
                                "Comms"
                            }
                        }
                        li {
                            class: "nav-item",
                            Link {
                                to: Route::ErrorTest {},
                                class: "nav-link",
                                active_class: "active",
                                "data-bs-dismiss": "offcanvas",
                                i { class: "bi bi-people me-2" }
                                "Circles"
                            }
                        }
                        li {
                            class: "nav-item",
                            Link {
                                to: Route::Home {},
                                class: "nav-link",
                                active_class: "active",
                                "data-bs-dismiss": "offcanvas",
                                i { class: "bi bi-diagram-3 me-2" }
                                "Trees"
                            }
                        }
                        li {
                            class: "nav-item",
                            Link {
                                to: Route::Settings {},
                                class: "nav-link",
                                active_class: "active",
                                "data-bs-dismiss": "offcanvas",
                                i { class: "bi bi-gear me-2" }
                                "Settings"
                            }
                        }
                    }
                }

                // User profile at bottom - Fixed position
                div { class: "border-top p-3 mt-auto",
                    div { class: "dropdown",
                        a {
                            class: "d-flex align-items-center text-decoration-none dropdown-toggle",
                            href: "#",
                            "data-bs-toggle": "dropdown",
                            "aria-expanded": "false",
                            // Placeholder avatar using Bootstrap Icons
                            i {
                                class: "bi bi-person-circle fs-4 me-2"
                            }
                            "User"
                        }
                        ul { class: "dropdown-menu text-small shadow",
                            li {
                                Link {
                                    to: Route::Settings {},
                                    class: "dropdown-item",
                                    "data-bs-dismiss": "offcanvas",
                                    "Settings"
                                }
                            }
                            li {
                                Link {
                                    to: Route::About {},
                                    class: "dropdown-item",
                                    "data-bs-dismiss": "offcanvas",
                                    "Profile"
                                }
                            }
                            li {
                                Link {
                                    to: Route::ErrorTest {},
                                    class: "dropdown-item",
                                    "data-bs-dismiss": "offcanvas",
                                    "System Info"
                                }
                            }
                            li { hr { class: "dropdown-divider" } }
                            li {
                                a {
                                    class: "dropdown-item",
                                    href: "#",
                                    "data-bs-dismiss": "offcanvas",
                                    "Sign out"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
