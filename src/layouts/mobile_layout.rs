use dioxus::prelude::*;
use crate::Route;
use crate::components::AppLogo;

#[component]
pub fn MobileLayout() -> Element {
    // Theme state (true = light, false = dark)
    let mut theme = use_signal(|| false); // Default to dark theme

    // Apply theme to document
    let theme_class = if *theme.read() { "light" } else { "dark" };

    // Set data-theme attribute on the html element for DaisyUI theming
    use_effect(move || {
        let theme_value = if *theme.read() { "light" } else { "dark" };
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let html = document.document_element().expect("document should have an element");
        html.set_attribute("data-theme", theme_value).ok();
    });

    rsx! {
        div { class: "app-container {theme_class} flex flex-col h-screen overflow-hidden bg-base-100 text-base-content",
            // Header - fixed at top
            header { class: "navbar bg-primary text-primary-content shadow-md sticky top-0 z-30",
                div { class: "flex-1",
                    // Logo
                    div { class: "logo-container flex items-center gap-2",
                        AppLogo {}
                        div { class: "flex flex-col",
                            h1 { class: "text-xl font-bold", "DJibon" }
                            span { class: "text-xs opacity-70", "beta" }
                        }
                    }
                }

                // Theme toggle
                div { class: "flex-none",
                    label { class: "swap swap-rotate btn btn-circle btn-ghost btn-sm",
                        // Theme toggle icon
                        input {
                            r#type: "checkbox",
                            checked: *theme.read(),
                            onclick: move |_| {
                                let current = *theme.read();
                                theme.set(!current);
                            }
                        }
                        // Sun icon
                        svg { class: "swap-on fill-current w-5 h-5", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24",
                            path { d: "M5.64,17l-.71.71a1,1,0,0,0,0,1.41,1,1,0,0,0,1.41,0l.71-.71A1,1,0,0,0,5.64,17ZM5,12a1,1,0,0,0-1-1H3a1,1,0,0,0,0,2H4A1,1,0,0,0,5,12Zm7-7a1,1,0,0,0,1-1V3a1,1,0,0,0-2,0V4A1,1,0,0,0,12,5ZM5.64,7.05a1,1,0,0,0,.7.29,1,1,0,0,0,.71-.29,1,1,0,0,0,0-1.41l-.71-.71A1,1,0,0,0,4.93,6.34Zm12,.29a1,1,0,0,0,.7-.29l.71-.71a1,1,0,1,0-1.41-1.41L17,5.64a1,1,0,0,0,0,1.41A1,1,0,0,0,17.66,7.34ZM21,11H20a1,1,0,0,0,0,2h1a1,1,0,0,0,0-2Zm-9,8a1,1,0,0,0-1,1v1a1,1,0,0,0,2,0V20A1,1,0,0,0,12,19ZM18.36,17A1,1,0,0,0,17,18.36l.71.71a1,1,0,0,0,1.41,0,1,1,0,0,0,0-1.41ZM12,6.5A5.5,5.5,0,1,0,17.5,12,5.51,5.51,0,0,0,12,6.5Zm0,9A3.5,3.5,0,1,1,15.5,12,3.5,3.5,0,0,1,12,15.5Z" }
                        }
                        // Moon icon
                        svg { class: "swap-off fill-current w-5 h-5", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24",
                            path { d: "M21.64,13a1,1,0,0,0-1.05-.14,8.05,8.05,0,0,1-3.37.73A8.15,8.15,0,0,1,9.08,5.49a8.59,8.59,0,0,1,.25-2A1,1,0,0,0,8,2.36,10.14,10.14,0,1,0,22,14.05,1,1,0,0,0,21.64,13Zm-9.5,6.69A8.14,8.14,0,0,1,7.08,5.22v.27A10.15,10.15,0,0,0,17.22,15.63a9.79,9.79,0,0,0,2.1-.22A8.11,8.11,0,0,1,12.14,19.73Z" }
                        }
                    }
                }
            }

            // Main content area with padding for bottom nav
            div { class: "flex-1 overflow-y-auto pb-16",
                // Content container with max width for better mobile experience
                div { class: "container mx-auto px-4 py-4 max-w-md",
                    Outlet::<Route> {}
                }
            }

            // Bottom navigation - fixed at the bottom
            nav { class: "btm-nav btm-nav-sm bg-base-200 shadow-lg border-t border-base-300 fixed bottom-0 left-0 right-0 z-50",
                Link {
                    class: "text-center",
                    to: Route::Home {},
                    active_class: "active text-primary border-primary",
                    svg { class: "w-5 h-5 mx-auto", xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke: "currentColor",
                        path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" }
                    }
                    span { class: "btm-nav-label text-xs", "Home" }
                }

                Link {
                    class: "text-center",
                    to: Route::Messages {},
                    active_class: "active text-primary border-primary",
                    svg { class: "w-5 h-5 mx-auto", xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke: "currentColor",
                        path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z" }
                    }
                    span { class: "btm-nav-label text-xs", "Messages" }
                }

                Link {
                    class: "text-center",
                    to: Route::Circles {},
                    active_class: "active text-primary border-primary",
                    svg { class: "w-5 h-5 mx-auto", xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke: "currentColor",
                        path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" }
                    }
                    span { class: "btm-nav-label text-xs", "Circles" }
                }

                Link {
                    class: "text-center",
                    to: Route::Trees {},
                    active_class: "active text-primary border-primary",
                    svg { class: "w-5 h-5 mx-auto", xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke: "currentColor",
                        path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01" }
                    }
                    span { class: "btm-nav-label text-xs", "Trees" }
                }

                Link {
                    class: "text-center",
                    to: Route::Profile {},
                    active_class: "active text-primary border-primary",
                    svg { class: "w-5 h-5 mx-auto", xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke: "currentColor",
                        path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" }
                    }
                    span { class: "btm-nav-label text-xs", "Profile" }
                }
            }
        }
    }
}
