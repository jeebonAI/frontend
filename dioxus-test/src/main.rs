use dioxus::prelude::*;

mod components;
mod state;

use components::{NavBar, BottomNav, ErrorBoundary};
use state::{use_app_state, Theme, increment_counter, toggle_theme};

// Define our routes
#[derive(Clone, Debug, PartialEq, Routable)]
enum Route {
    #[layout(MainLayout)]
    #[route("/")]
    Home {},

    #[route("/about")]
    About {},

    #[route("/settings")]
    Settings {},

    #[route("/error-test")]
    ErrorTest {},

    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

// Define assets
const BOOTSTRAP_CSS: &str = "https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/css/bootstrap.min.css";
const BOOTSTRAP_JS: &str = "https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/js/bootstrap.bundle.min.js";
const BOOTSTRAP_ICONS: &str = "https://cdn.jsdelivr.net/npm/bootstrap-icons@1.11.3/font/bootstrap-icons.css";

// Include the CSS file
static STYLE: &str = include_str!("../public/style.css");

// Application with routing
fn main() {
    // Initialize logger
    wasm_logger::init(wasm_logger::Config::default());

    // Launch the application
    dioxus::launch(App);
}

// Home page component
#[component]
fn Home() -> Element {
    // Get the app state
    let state = use_app_state();

    // Read the counter value
    let counter = state.read().counter;

    // Create an event handler for the increment button
    let increment = move |_| {
        increment_counter(state);
    };

    rsx! {
        div { class: "container mt-5",
            h1 { "Home Page" }
            p { "This is the home page of our test application." }
            div { class: "mt-4",
                p { "Counter: {counter}" }
                button {
                    class: "btn btn-primary",
                    onclick: increment,
                    "Increment"
                }
            }
            div { class: "mt-3",
                Link { to: Route::About {}, class: "btn btn-outline-primary me-2", "Go to About" }
                Link { to: Route::Settings {}, class: "btn btn-outline-secondary", "Go to Settings" }
            }
        }
    }
}

// About page component
#[component]
fn About() -> Element {
    rsx! {
        div { class: "container mt-5",
            h1 { "About Page" }
            p { "This is the about page of our test application." }
            Link { to: Route::Home {}, class: "btn btn-primary", "Go to Home" }
        }
    }
}

// Settings page component
#[component]
fn Settings() -> Element {
    // Get the app state
    let state = use_app_state();

    // Check if the theme is dark
    let is_dark = matches!(state.read().theme, Theme::Dark);

    // Create an event handler for the theme toggle
    let handle_toggle = move |_| {
        toggle_theme(state);
    };

    rsx! {
        div { class: "container mt-5",
            h1 { "Settings" }
            div { class: "form-check form-switch mt-3",
                input {
                    class: "form-check-input",
                    r#type: "checkbox",
                    id: "darkModeSwitch",
                    checked: is_dark,
                    onclick: handle_toggle
                }
                label {
                    class: "form-check-label",
                    r#for: "darkModeSwitch",
                    "Dark Mode"
                }
            }
            div { class: "mt-4",
                Link { to: Route::Home {}, class: "btn btn-primary me-2", "Go to Home" }
                Link { to: Route::ErrorTest {}, class: "btn btn-danger", "Test Error Handling" }
            }
        }
    }
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
    // Get the app state
    let state = use_app_state();

    // Determine data-bs-theme attribute based on theme
    let theme_attr = match state.read().theme {
        Theme::Light => "light",
        Theme::Dark => "dark",
    };

    rsx! {
        div {
            class: "d-flex flex-column min-vh-100",
            "data-bs-theme": theme_attr,
            NavBar {}
            div { class: "container mt-4 mb-5 flex-grow-1",
                Outlet::<Route> {}
            }
            BottomNav {}
        }
    }
}

// Main app component
#[component]
fn App() -> Element {
    rsx! {
        // Include Bootstrap CSS and JS
        document::Link { rel: "stylesheet", href: BOOTSTRAP_CSS }
        document::Link { rel: "stylesheet", href: BOOTSTRAP_ICONS }
        document::Script { src: BOOTSTRAP_JS }

        // Include our custom CSS
        document::Style { { STYLE } }

        // Error boundary to catch and display errors
        ErrorBoundary {
            // Define our routes
            Router::<Route> {}
        }
    }
}
