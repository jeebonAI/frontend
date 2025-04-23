use dioxus::prelude::*;

// Import Route from main.rs
use crate::Route;

#[component]
pub fn MainLayout() -> Element {
    rsx! {
        div { class: "app-container",
            // Header
            header { class: "app-header",
                div { class: "flex-1",
                    h1 { class: "text-xl font-bold", "DJibon" }
                }
                div { class: "flex-none hidden md:block",
                    p { "Personal Assistance and Communication Tool" }
                }
                div { class: "flex-none",
                    // Theme toggle button
                    label { class: "swap swap-rotate btn btn-circle btn-ghost",
                        // Theme toggle icon
                        input { r#type: "checkbox" }
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

            // Main content area with sidebar and content
            div { class: "main-content",
                // Sidebar navigation
                nav { class: "sidebar",
                    ul { class: "sidebar-menu",
                        li {
                            Link { class: "flex items-center gap-2", to: Route::Home {},
                                svg { class: "w-5 h-5", xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke: "currentColor",
                                    path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" }
                                }
                                "Home"
                            }
                        }
                        li {
                            Link { class: "flex items-center gap-2", to: Route::Profile {},
                                svg { class: "w-5 h-5", xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke: "currentColor",
                                    path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" }
                                }
                                "Profile"
                            }
                        }
                        li {
                            Link { class: "flex items-center gap-2", to: Route::Messages {},
                                svg { class: "w-5 h-5", xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke: "currentColor",
                                    path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z" }
                                }
                                "Messages"
                            }
                        }
                        li {
                            Link { class: "flex items-center gap-2", to: Route::Circles {},
                                svg { class: "w-5 h-5", xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke: "currentColor",
                                    path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" }
                                }
                                "Circles"
                            }
                        }
                        li {
                            Link { class: "flex items-center gap-2", to: Route::Trees {},
                                svg { class: "w-5 h-5", xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke: "currentColor",
                                    path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01" }
                                }
                                "Trees"
                            }
                        }
                        li {
                            Link { class: "flex items-center gap-2", to: Route::Settings {},
                                svg { class: "w-5 h-5", xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke: "currentColor",
                                    path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" }
                                    path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M15 12a3 3 0 11-6 0 3 3 0 016 0z" }
                                }
                                "Settings"
                            }
                        }
                    }
                }

                // Main content area
                div { class: "content",
                    Outlet::<Route> {}
                }
            }

            // Footer
            footer { class: "app-footer",
                div { class: "grid-flow-col gap-4",
                    p { "DJibon - Prototype Version" }
                }
                div { class: "grid-flow-col gap-4 md:place-self-center md:justify-self-end",
                    a { class: "link link-hover", "About" }
                    a { class: "link link-hover", "Contact" }
                    a { class: "link link-hover", "Privacy" }
                }
            }
        }
    }
}
