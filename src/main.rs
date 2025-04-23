use dioxus::prelude::*;

// Import modules
mod pages;
mod layouts;
mod components;

// Import components from modules
use layouts::main_layout::MainLayout;
use pages::home::Home;
use pages::profile::Profile;
use pages::messages::Messages;
use pages::circles::Circles;
use pages::trees::Trees;
use pages::settings::Settings;

// Define assets
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const FAVICON: Asset = asset!("/assets/favicon.ico");

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
        // Include stylesheets in the head
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "icon", href: FAVICON }
        Router::<Route> {}
    }
}














