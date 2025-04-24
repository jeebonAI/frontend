use dioxus::prelude::*;
use crate::routes::Route;

#[component]
pub fn MainLayout() -> Element {
    rsx! {
        // Main container
        div { class: "d-flex flex-column min-vh-100",
            // Top navbar with toggle button (visible on smaller screens)
            nav { class: "navbar navbar-expand-lg navbar-light bg-light d-lg-none border-bottom",
                div { class: "container-fluid",
                    a { class: "navbar-brand", href: "#", 
                        svg {
                            width: "30",
                            height: "30",
                            view_box: "0 0 24 24",
                            fill: "currentColor",
                            class: "me-2",
                            
                            // Improved tree of life logo
                            path {
                                d: "M12 2c0 0-9 6.5-9 13c0 4.4 4 7 9 7s9-2.6 9-7C21 8.5 12 2 12 2zm0 18c-3.9 0-7-1.7-7-5c0-4.9 7-10.5 7-10.5s7 5.6 7 10.5C19 18.3 15.9 20 12 20z"
                            }
                            // Inner details of the tree
                            path {
                                d: "M12 6.5c0 0-5 3.5-5 7.5c0 2.5 2.2 4 5 4s5-1.5 5-4C17 10 12 6.5 12 6.5zm0 9.5c-1.9 0-3-0.9-3-2c0-2.2 3-4.8 3-4.8s3 2.6 3 4.8C15 15.1 13.9 16 12 16z"
                            }
                            // Center circle
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
            
            // Content area with sidebar and main content
            div { class: "d-flex flex-grow-1 overflow-hidden",
                // Sidebar - offcanvas on mobile, visible on larger screens
                div { 
                    class: "offcanvas-lg offcanvas-start",
                    id: "sidebarMenu",
                    tabindex: "-1",
                    style: "width: 280px;",
                    
                    // Offcanvas header (only visible when in offcanvas mode)
                    div { class: "offcanvas-header d-lg-none",
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
                            h5 { class: "offcanvas-title mb-0", "DJibon" }
                        }
                        button {
                            r#type: "button",
                            class: "btn-close",
                            "data-bs-dismiss": "offcanvas",
                            "data-bs-target": "#sidebarMenu",
                            "aria-label": "Close"
                        }
                    }
                    
                    // Sidebar content
                    div { class: "offcanvas-body d-flex flex-column h-100 p-0 overflow-hidden",
                        // Logo/Brand (hidden on small screens as it's in the navbar)
                        div { class: "d-none d-lg-block p-3",
                            a { 
                                class: "d-flex align-items-center mb-3 mb-md-0 me-md-auto link-dark text-decoration-none",
                                href: "#",
                                svg {
                                    width: "36",
                                    height: "36",
                                    view_box: "0 0 24 24",
                                    fill: "currentColor",
                                    class: "me-2",
                                    
                                    // Improved tree of life logo
                                    path {
                                        d: "M12 2c0 0-9 6.5-9 13c0 4.4 4 7 9 7s9-2.6 9-7C21 8.5 12 2 12 2zm0 18c-3.9 0-7-1.7-7-5c0-4.9 7-10.5 7-10.5s7 5.6 7 10.5C19 18.3 15.9 20 12 20z"
                                    }
                                    // Inner details of the tree
                                    path {
                                        d: "M12 6.5c0 0-5 3.5-5 7.5c0 2.5 2.2 4 5 4s5-1.5 5-4C17 10 12 6.5 12 6.5zm0 9.5c-1.9 0-3-0.9-3-2c0-2.2 3-4.8 3-4.8s3 2.6 3 4.8C15 15.1 13.9 16 12 16z"
                                    }
                                    // Center circle
                                    circle {
                                        cx: "12",
                                        cy: "13",
                                        r: "1.5"
                                    }
                                }
                                span { class: "fs-4", "DJibon" }
                            }
                            hr {}
                        }
                        
                        // Navigation menu - make it scrollable if needed
                        div { class: "nav-container overflow-auto flex-grow-1 p-3",
                            ul { class: "nav nav-pills flex-column",
                                li { class: "nav-item",
                                    Link {
                                        to: Route::Home {},
                                        class: "nav-link link-dark",
                                        active_class: "active",
                                        "data-bs-dismiss": "offcanvas",
                                        "data-bs-target": "#sidebarMenu",
                                        i { class: "bi bi-house-door me-2" }
                                        "Home"
                                    }
                                }
                                li { class: "nav-item",
                                    Link {
                                        to: Route::Profile {},
                                        class: "nav-link link-dark",
                                        active_class: "active",
                                        "data-bs-dismiss": "offcanvas",
                                        "data-bs-target": "#sidebarMenu",
                                        i { class: "bi bi-person me-2" }
                                        "Profile"
                                    }
                                }
                                li { class: "nav-item",
                                    Link {
                                        to: Route::Messages {},
                                        class: "nav-link link-dark",
                                        active_class: "active",
                                        "data-bs-dismiss": "offcanvas",
                                        "data-bs-target": "#sidebarMenu",
                                        i { class: "bi bi-chat-dots me-2" }
                                        "Messages"
                                    }
                                }
                                li { class: "nav-item",
                                    Link {
                                        to: Route::Circles {},
                                        class: "nav-link link-dark",
                                        active_class: "active",
                                        "data-bs-dismiss": "offcanvas",
                                        "data-bs-target": "#sidebarMenu",
                                        i { class: "bi bi-people me-2" }
                                        "Circles"
                                    }
                                }
                                li { class: "nav-item",
                                    Link {
                                        to: Route::Trees {},
                                        class: "nav-link link-dark",
                                        active_class: "active",
                                        "data-bs-dismiss": "offcanvas",
                                        "data-bs-target": "#sidebarMenu",
                                        i { class: "bi bi-diagram-3 me-2" }
                                        "Trees"
                                    }
                                }
                                li { class: "nav-item",
                                    Link {
                                        to: Route::Settings {},
                                        class: "nav-link link-dark",
                                        active_class: "active",
                                        "data-bs-dismiss": "offcanvas",
                                        "data-bs-target": "#sidebarMenu",
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
                                    class: "d-flex align-items-center link-dark text-decoration-none dropdown-toggle",
                                    href: "#",
                                    "data-bs-toggle": "dropdown",
                                    "aria-expanded": "false",
                                    // Placeholder avatar using Bootstrap Icons
                                    i { 
                                        class: "bi bi-person-circle fs-4 me-2",
                                        style: "font-size: 32px;" 
                                    }
                                    "User"
                                }
                                ul { class: "dropdown-menu text-small shadow",
                                    li { 
                                        Link {
                                            to: Route::Settings {},
                                            class: "dropdown-item",
                                            "data-bs-dismiss": "offcanvas",
                                            "data-bs-target": "#sidebarMenu",
                                            "Settings"
                                        }
                                    }
                                    li { 
                                        Link {
                                            to: Route::Profile {},
                                            class: "dropdown-item",
                                            "data-bs-dismiss": "offcanvas",
                                            "data-bs-target": "#sidebarMenu",
                                            "Profile"
                                        }
                                    }
                                    li { 
                                        Link {
                                            to: Route::SystemInfo {},
                                            class: "dropdown-item",
                                            "data-bs-dismiss": "offcanvas",
                                            "data-bs-target": "#sidebarMenu",
                                            "System Info"
                                        }
                                    }
                                    li { hr { class: "dropdown-divider" } }
                                    li { 
                                        a { 
                                            class: "dropdown-item",
                                            href: "#",
                                            "data-bs-dismiss": "offcanvas",
                                            "data-bs-target": "#sidebarMenu",
                                            onclick: move |_| {
                                                log::info!("User logged out");
                                                // Add actual logout logic here when implemented
                                            },
                                            "Sign out"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                // Main content
                div { class: "flex-grow-1 p-3 overflow-auto",
                    main { class: "container-fluid",
                        Outlet::<Route> {}
                    }
                    
                    // Footer
                    footer { class: "mt-auto text-center p-3",
                        p { class: "mb-0 text-muted", "DJibon - Prototype Version" }
                    }
                }
            }

            
            // Bottom navigation - visible on all screen sizes for testing
            nav { class: "navbar fixed-bottom navbar-light bg-light", 
                div { class: "container-fluid",
                    div { class: "row w-100 text-center",
                        div { class: "col",
                            Link { 
                                to: Route::Home {}, 
                                class: "nav-link", 
                                active_class: "active fw-bold text-primary",
                                i { class: "bi bi-house d-block" }
                                "Home"
                            }
                        }
                        div { class: "col",
                            Link { 
                                to: Route::Profile {}, 
                                class: "nav-link", 
                                active_class: "active fw-bold text-primary",
                                i { class: "bi bi-person d-block" }
                                "Profile"
                            }
                        }
                        div { class: "col",
                            Link { 
                                to: Route::Messages {}, 
                                class: "nav-link", 
                                active_class: "active fw-bold text-primary",
                                i { class: "bi bi-chat d-block" }
                                "Messages"
                            }
                        }
                        div { class: "col",
                            Link { 
                                to: Route::Circles {}, 
                                class: "nav-link", 
                                active_class: "active fw-bold text-primary",
                                i { class: "bi bi-people d-block" }
                                "Circles"
                            }
                        }
                        div { class: "col",
                            Link { 
                                to: Route::Trees {}, 
                                class: "nav-link", 
                                active_class: "active fw-bold text-primary",
                                i { class: "bi bi-diagram-3 d-block" }
                                "Trees"
                            }
                        }
                    }
                }
            }
        }
    }
}
