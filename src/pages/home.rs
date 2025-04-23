use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        div { class: "page-container",
            // Hero section
            div { class: "hero bg-base-200 rounded-box mb-8",
                div { class: "hero-content text-center",
                    div { class: "max-w-md",
                        h1 { class: "text-4xl font-bold", "Welcome to DJibon" }
                        p { class: "py-6",
                            "This is a prototype of the DJibon Personal Assistance and Communication Tool. "
                            "It demonstrates the basic structure and navigation of the application."
                        }
                        button { class: "btn btn-primary", "Get Started" }
                    }
                }
            }

            // Interactive demo card
            div { class: "card bg-base-100 shadow-xl mb-8",
                div { class: "card-body",
                    h2 { class: "card-title", "Interactive Demo" }
                    p { "Click the button below to increment the counter:" }
                    
                    div { class: "flex items-center gap-4 my-4",
                        button {
                            class: "btn btn-primary",
                            onclick: move |_| count += 1,
                            "Increment"
                        }
                        
                        // Counter display with badge
                        div { class: "badge badge-lg badge-primary badge-outline", "Count: {count}" }
                    }
                }
            }

            // Features section with cards
            div { class: "mb-8",
                h2 { class: "text-2xl font-bold mb-4", "Key Features" }
                
                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                    // Feature 1
                    div { class: "card bg-base-100 shadow-md hover:shadow-xl transition-all duration-300",
                        div { class: "card-body",
                            div { class: "flex items-center justify-center w-12 h-12 rounded-full bg-primary text-primary-content mb-4",
                                svg { class: "w-6 h-6", xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke: "currentColor",
                                    path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" }
                                }
                            }
                            h3 { class: "card-title", "User Profiles" }
                            p { "Complete user profiles with authentication and personalization options." }
                        }
                    }
                    
                    // Feature 2
                    div { class: "card bg-base-100 shadow-md hover:shadow-xl transition-all duration-300",
                        div { class: "card-body",
                            div { class: "flex items-center justify-center w-12 h-12 rounded-full bg-secondary text-secondary-content mb-4",
                                svg { class: "w-6 h-6", xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke: "currentColor",
                                    path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z" }
                                }
                            }
                            h3 { class: "card-title", "Messaging" }
                            p { "Real-time messaging with support for text, images, and other media types." }
                        }
                    }
                    
                    // Feature 3
                    div { class: "card bg-base-100 shadow-md hover:shadow-xl transition-all duration-300",
                        div { class: "card-body",
                            div { class: "flex items-center justify-center w-12 h-12 rounded-full bg-accent text-accent-content mb-4",
                                svg { class: "w-6 h-6", xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke: "currentColor",
                                    path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z" }
                                }
                            }
                            h3 { class: "card-title", "Calls" }
                            p { "Audio and video calls with excellent quality and reliability." }
                        }
                    }
                    
                    // Feature 4
                    div { class: "card bg-base-100 shadow-md hover:shadow-xl transition-all duration-300",
                        div { class: "card-body",
                            div { class: "flex items-center justify-center w-12 h-12 rounded-full bg-primary text-primary-content mb-4",
                                svg { class: "w-6 h-6", xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke: "currentColor",
                                    path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" }
                                }
                            }
                            h3 { class: "card-title", "Circles" }
                            p { "Create and manage social circles for group communication and sharing." }
                        }
                    }
                    
                    // Feature 5
                    div { class: "card bg-base-100 shadow-md hover:shadow-xl transition-all duration-300",
                        div { class: "card-body",
                            div { class: "flex items-center justify-center w-12 h-12 rounded-full bg-secondary text-secondary-content mb-4",
                                svg { class: "w-6 h-6", xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke: "currentColor",
                                    path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01" }
                                }
                            }
                            h3 { class: "card-title", "Trees" }
                            p { "Organize information and relationships in hierarchical tree structures." }
                        }
                    }
                    
                    // Feature 6
                    div { class: "card bg-base-100 shadow-md hover:shadow-xl transition-all duration-300",
                        div { class: "card-body",
                            div { class: "flex items-center justify-center w-12 h-12 rounded-full bg-accent text-accent-content mb-4",
                                svg { class: "w-6 h-6", xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke: "currentColor",
                                    path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M3 15a4 4 0 004 4h9a5 5 0 10-.1-9.999 5.002 5.002 0 10-9.78 2.096A4.001 4.001 0 003 15z" }
                                }
                            }
                            h3 { class: "card-title", "Online/Offline" }
                            p { "Works seamlessly both online and offline with automatic synchronization." }
                        }
                    }
                }
            }
            
            // Stats section
            div { class: "stats shadow w-full",
                div { class: "stat",
                    div { class: "stat-figure text-primary",
                        svg { class: "inline-block w-8 h-8 stroke-current", xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24",
                            path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" }
                        }
                    }
                    div { class: "stat-title", "Total Users" }
                    div { class: "stat-value text-primary", "25.6K" }
                    div { class: "stat-desc", "21% more than last month" }
                }
                
                div { class: "stat",
                    div { class: "stat-figure text-secondary",
                        svg { class: "inline-block w-8 h-8 stroke-current", xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24",
                            path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M13 10V3L4 14h7v7l9-11h-7z" }
                        }
                    }
                    div { class: "stat-title", "Page Views" }
                    div { class: "stat-value text-secondary", "2.6M" }
                    div { class: "stat-desc", "14% more than last month" }
                }
                
                div { class: "stat",
                    div { class: "stat-figure text-secondary",
                        div { class: "avatar online",
                            div { class: "w-16 rounded-full",
                                img { src: "https://daisyui.com/images/stock/photo-1534528741775-53994a69daeb.jpg" }
                            }
                        }
                    }
                    div { class: "stat-value", "86%" }
                    div { class: "stat-title", "Tasks done" }
                    div { class: "stat-desc text-secondary", "31 tasks remaining" }
                }
            }
        }
    }
}
