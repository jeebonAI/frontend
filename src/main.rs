use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(MainLayout)]
    #[route("/")]
    Home {},
    #[route("/profile")]
    Profile {},
    #[route("/messages")]
    Messages {},
    #[route("/circles")]
    Circles {},
    #[route("/trees")]
    Trees {},
    #[route("/settings")]
    Settings {},
}

const MAIN_CSS: Asset = asset!("/assets/main.css");
const FAVICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    // Initialize the logger for better error messages
    // Using wasm_logger instead of dioxus_logger for web compatibility
    wasm_logger::init(wasm_logger::Config::default());

    // Launch the app
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "icon", href: FAVICON }
        Router::<Route> {}
    }
}

/// Main layout with navigation
#[component]
fn MainLayout() -> Element {
    rsx! {
        div { class: "app-container",
            // Header
            header { class: "app-header",
                h1 { "DJibon" }
                p { "Personal Assistance and Communication Tool" }
            }

            // Main content area with sidebar and content
            div { class: "main-content",
                // Sidebar navigation
                nav { class: "sidebar",
                    ul {
                        li { Link { to: Route::Home {}, "Home" } }
                        li { Link { to: Route::Profile {}, "Profile" } }
                        li { Link { to: Route::Messages {}, "Messages" } }
                        li { Link { to: Route::Circles {}, "Circles" } }
                        li { Link { to: Route::Trees {}, "Trees" } }
                        li { Link { to: Route::Settings {}, "Settings" } }
                    }
                }

                // Main content area
                div { class: "content",
                    Outlet::<Route> {}
                }
            }

            // Footer
            footer { class: "app-footer",
                p { "DJibon - Prototype Version" }
            }
        }
    }
}

/// Home page
#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        div { class: "home-page",
            h2 { "Welcome to DJibon" }

            p { class: "intro",
                "This is a prototype of the DJibon Personal Assistance and Communication Tool. "
                "It demonstrates the basic structure and navigation of the application."
            }

            div { class: "demo-card",
                h3 { "Interactive Demo" }

                p { "Click the button below to increment the counter:" }

                div { class: "counter",
                    button {
                        onclick: move |_| count += 1,
                        "Increment"
                    }

                    p { "Count: {count}" }
                }
            }

            div { class: "features-list",
                h3 { "Key Features:" }

                ul {
                    li { "User profiles and authentication" }
                    li { "Real-time messaging" }
                    li { "Audio/video calls" }
                    li { "Circles for group communication" }
                    li { "Trees for hierarchical organization" }
                    li { "Works online and offline" }
                }
            }
        }
    }
}

/// Profile page
#[component]
fn Profile() -> Element {
    rsx! {
        div { class: "profile-page",
            h2 { "User Profile" }
            p { "This page will display and allow editing of the user profile." }
            p { class: "coming-soon", "Coming Soon" }
        }
    }
}

/// Messages page
#[component]
fn Messages() -> Element {
    rsx! {
        div { class: "messages-page",
            h2 { "Messages" }
            p { "This page will display conversations and allow messaging." }
            p { class: "coming-soon", "Coming Soon" }
        }
    }
}

/// Circles page
#[component]
fn Circles() -> Element {
    rsx! {
        div { class: "circles-page",
            h2 { "Circles" }
            p { "This page will display and manage social circles." }
            p { class: "coming-soon", "Coming Soon" }
        }
    }
}

/// Trees page
#[component]
fn Trees() -> Element {
    rsx! {
        div { class: "trees-page",
            h2 { "Trees" }
            p { "This page will display and manage hierarchical trees." }
            p { class: "coming-soon", "Coming Soon" }
        }
    }
}

/// Settings page
#[component]
fn Settings() -> Element {
    rsx! {
        div { class: "settings-page",
            h2 { "Settings" }
            p { "This page will allow configuration of application settings." }
            p { class: "coming-soon", "Coming Soon" }
        }
    }
}
