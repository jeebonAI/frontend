use dioxus::prelude::*;

mod components;
mod state;

use components::{NavBar, BottomNav, Home, Profile, Comms, Circles, Tree, Settings, SystemInfo};
use state::{use_app_state, Theme};
use dioxus::prelude::{ErrorBoundary, VNode};

// Define our routes
#[derive(Clone, Debug, PartialEq, Routable)]
enum Route {
    #[layout(MainLayout)]
    #[route("/")]
    Home {},

    #[route("/profile")]
    Profile {},

    #[route("/comms")]
    Comms {},

    #[route("/circles")]
    Circles {},

    #[route("/trees")]
    Tree {},

    #[route("/settings")]
    Settings {},

    #[route("/system-info")]
    SystemInfo {},

    #[route("/error-test")]
    ErrorTest {},

    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

// Define assets
const BOOTSTRAP_CSS: &str = "https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/css/bootstrap.min.css";
const BOOTSTRAP_JS: &str = "https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/js/bootstrap.bundle.min.js";
const BOOTSTRAP_ICONS: &str = "https://cdn.jsdelivr.net/npm/bootstrap-icons@1.11.3/font/bootstrap-icons.css";
const STYLE: &str = "/assets/style.css";

// Application with routing
fn main() {
    // Initialize platform-specific logger
    #[cfg(feature = "web")]
    wasm_logger::init(wasm_logger::Config::default());

    #[cfg(any(feature = "mobile", feature = "desktop"))]
    tracing_subscriber::fmt::init();

    // Platform-specific configuration
    #[cfg(all(feature = "mobile", target_os = "android"))]
    {
        // Android-specific initialization if needed
        tracing::info!("Starting Jeebon on Android");
    }

    // Launch the application
    dioxus::launch(App);
}

// Error test page component
#[component]
fn ErrorTest() -> Element {
    // Create a state to track if we're showing an error
    let mut show_error = use_signal(|| false);

    // Function to toggle error display
    let toggle_error = move |_| {
        let current = *show_error.read();
        show_error.set(!current);
    };

    // Read the current value
    let is_showing_error = *show_error.read();

    if is_showing_error {
        rsx! {
            div { class: "container mt-5",
                div { class: "alert alert-danger",
                    h4 { "Error" }
                    p { "This is a simulated error message." }
                    p { "In a real application, this would be caught by an error boundary." }
                }
                button {
                    class: "btn btn-primary mt-3",
                    onclick: toggle_error,
                    "Clear Error"
                }
            }
        }
    } else {
        rsx! {
            div { class: "container mt-5",
                h1 { "Error Test" }
                p { "This page demonstrates error handling in the application." }
                p { "Click the button below to simulate an error:" }
                button {
                    class: "btn btn-danger",
                    onclick: toggle_error,
                    "Simulate Error"
                }
            }
        }
    }
}

// Not found page component
#[component]
fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        div { class: "container mt-5",
            h1 { "Page Not Found" }
            p { "The page {route.join(\"/\")} was not found." }
            Link { to: Route::Home {}, class: "btn btn-primary", "Go to Home" }
        }
    }
}

// Main layout component
#[component]
fn MainLayout() -> Element {

    // Platform-specific classes
    #[cfg(feature = "mobile")]
    let container_class = "container-fluid px-2 mb-5 flex-grow-1";

    #[cfg(not(feature = "mobile"))]
    let container_class = "container mb-5 flex-grow-1";

    rsx! {
        div {
            class: "d-flex flex-column vh-100 pb-5 position-relative overflow-hidden",
            NavBar {}
            div {
                class: "{container_class} overflow-auto",
                Outlet::<Route> {}
            }
            BottomNav {}
        }
    }
}

// Main app component
#[component]
fn App() -> Element {
    // Get the app state to determine initial theme
    let state = use_app_state();

    // Determine initial theme attribute
    let theme_attr = match state.read().theme {
        Theme::Light => "light",
        Theme::Dark => "dark",
    };


    // React to theme changes and update the <html> element's data-bs-theme attribute
    #[cfg(feature = "web")]
    use_effect({
        move || {

            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(html) = document.document_element() {
                        let _ = html.set_attribute("data-bs-theme", theme_attr);
                    }
                }
            }
        }
    });

    // Mobile platform: Log theme change (replace with native theming if needed)
    #[cfg(feature = "mobile")]
    {
        tracing::info!("Theme changed to: {}", theme_attr);
        // If using a WebView with Bootstrap, the data-bs-theme attribute can still be applied
        // to a root element in your mobile app's WebView.
        // If rendering native components, integrate with native theming APIs here.
        // Example for Android (pseudo-code):
        // use capacitor::AppCompatDelegate;
        // AppCompatDelegate::setDefaultNightMode(
        //     if theme_attr == "dark" {
        //         AppCompatDelegate::MODE_NIGHT_YES
        //     } else {
        //         AppCompatDelegate::MODE_NIGHT_NO
        //     }
        // );
    }

    // Set the theme on the document element (web only)
    #[cfg(feature = "web")]
    {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(html) = document.document_element() {
                    let _ = html.set_attribute("data-bs-theme", theme_attr);
                }
            }
        }
    }

    // For mobile platforms, we could add platform-specific initialization here
    // #[cfg(feature = "mobile")]
    // {
    //     // Mobile-specific theme initialization could be added here
    // }

    rsx! {
        document::Link { rel: "stylesheet", href: STYLE }
        document::Link { rel: "stylesheet", href: BOOTSTRAP_CSS }
        document::Link { rel: "stylesheet", href: BOOTSTRAP_ICONS }
        document::Script { src: BOOTSTRAP_JS }

        // Add viewport meta tag for mobile responsiveness
        document::Meta { name: "viewport", content: "width=device-width, height=device-height, initial-scale=1.0, maximum-scale=1.0, user-scalable=no, viewport-fit=cover" }
        document::Meta { name: "theme-color", content: "#ffffff" }

        // Add mobile-specific meta tags
        {if cfg!(feature = "mobile") {
            rsx!(
                document::Meta { name: "mobile-web-app-capable", content: "yes" }
                document::Meta { name: "apple-mobile-web-app-capable", content: "yes" }
                document::Meta { name: "apple-mobile-web-app-status-bar-style", content: "black-translucent" }
            )
        } else {
            rsx!()
        }}

        // Custom CSS can be added here if needed
        // document::Style { {"custom CSS here"} }

        // Error boundary to catch and display errors
        ErrorBoundary {
            // Define our routes
            Router::<Route> {}
        }
    }
}
