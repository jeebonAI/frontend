use dioxus::prelude::*;
use crate::routes::Route;

#[component]
pub fn MainLayout() -> Element {
    let mut sidebar_open = use_signal(|| false);

    rsx! {
        div { class: "d-flex flex-column min-vh-100",
            nav { class: "navbar navbar-expand-lg navbar-dark bg-primary",
                div { class: "container-fluid",
                    button {
                        class: "navbar-toggler",
                        r#type: "button",
                        onclick: move |_| sidebar_open.set(!sidebar_open()),
                        span { class: "navbar-toggler-icon" }
                    }
                    a { class: "navbar-brand", href: "#", "DJibon" }
                    div { class: "d-none d-lg-block",
                        span { class: "navbar-text", "Personal Assistance and Communication Tool" }
                    }
                }
            }
            div {
                class: if sidebar_open() { "offcanvas offcanvas-start show" } else { "offcanvas offcanvas-start" },
                tabindex: "-1",
                id: "sidebar",
                "aria-labelledby": "sidebarLabel",
                div { class: "offcanvas-header",
                    h5 { class: "offcanvas-title", id: "sidebarLabel", "DJibon Menu" }
                    button {
                        class: "btn-close text-reset",
                        r#type: "button",
                        onclick: move |_| sidebar_open.set(false),
                        "aria-label": "Close"
                    }
                }
                div { class: "offcanvas-body",
                    ul { class: "nav flex-column",
                        li { class: "nav-item",
                            Link {
                                to: Route::Home {},
                                class: "nav-link text-dark",
                                active_class: "active bg-primary text-white",
                                onclick: move |_| sidebar_open.set(false),
                                "Home"
                            }
                        }
                        li { class: "nav-item",
                            Link {
                                to: Route::Profile {},
                                class: "nav-link text-dark",
                                active_class: "active bg-primary text-white",
                                onclick: move |_| sidebar_open.set(false),
                                "Profile"
                            }
                        }
                        li { class: "nav-item",
                            Link {
                                to: Route::Messages {},
                                class: "nav-link text-dark",
                                active_class: "active bg-primary text-white",
                                onclick: move |_| sidebar_open.set(false),
                                "Messages"
                            }
                        }
                        li { class: "nav-item",
                            Link {
                                to: Route::Circles {},
                                class: "nav-link text-dark",
                                active_class: "active bg-primary text-white",
                                onclick: move |_| sidebar_open.set(false),
                                "Circles"
                            }
                        }
                        li { class: "nav-item",
                            Link {
                                to: Route::Trees {},
                                class: "nav-link text-dark",
                                active_class: "active bg-primary text-white",
                                onclick: move |_| sidebar_open.set(false),
                                "Trees"
                            }
                        }
                        li { class: "nav-item",
                            Link {
                                to: Route::Settings {},
                                class: "nav-link text-dark",
                                active_class: "active bg-primary text-white",
                                onclick: move |_| sidebar_open.set(false),
                                "Settings"
                            }
                        }
                    }
                }
            }
            main { class: "flex-grow-1 p-3",
                Outlet::<Route> {}
            }
            footer { class: "bg-light text-center p-3",
                p { class: "mb-0", "DJibon - Prototype Version" }
            }
        }
        style {
            ".nav-link:hover {{ background-color: #e9ecef; color: #0d6efd !important; transition: background-color 0.3s ease, color 0.3s ease; }} .offcanvas-body .nav-link {{ padding: 0.75rem 1.25rem; font-size: 1.1rem; }} .offcanvas {{ width: 250px; }}"
        }
    }
}