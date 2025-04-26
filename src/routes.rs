use dioxus::prelude::*;
use crate::state::{use_app_state, Theme, toggle_theme};
use crate::components::NavBar;

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[layout(NavBar)]
    #[route("/")]
    Home {},

    #[route("/about")]
    About {},

    #[route("/settings")]
    Settings {},

    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "container mt-4",
            h1 { "Home Page" }
            p { "This is the home page of our test application." }
        }
    }
}

#[component]
pub fn About() -> Element {
    rsx! {
        div {
            class: "container mt-4",
            h1 { "About Page" }
            p { "This is the about page of our test application." }
        }
    }
}

#[component]
pub fn Settings() -> Element {
    // Get the app state
    let state = use_app_state();

    // Check if the theme is dark
    let is_dark = matches!(state.read().theme, Theme::Dark);

    // Create an event handler for the theme toggle
    let handle_toggle = move |_| {
        toggle_theme(state);
    };

    rsx! {
        div {
            class: "container mt-4",
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
        }
    }
}

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        div {
            class: "container mt-4",
            h1 { "Page Not Found" }
            p { "The page {route.join(\"/\")} was not found." }
        }
    }
}
