use dioxus::prelude::*;
use routes::Route;

mod components;
mod routes;

// Define assets
const BOOTSTRAP_CSS: Asset = asset!("/assets/bootstrap.min.css");
const BOOTSTRAP_JS: Asset = asset!("/assets/bootstrap.bundle.min.js");
const FAVICON: Asset = asset!("/assets/favicon.ico");
// Removed Cytoscape assets as they're now loaded in the Tree component

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: BOOTSTRAP_CSS }
        document::Link { 
            rel: "stylesheet", 
            href: "https://cdn.jsdelivr.net/npm/bootstrap-icons@1.11.3/font/bootstrap-icons.css" 
        }
        document::Link { rel: "icon", href: FAVICON }
        document::Script { src: BOOTSTRAP_JS }
        // Removed Cytoscape scripts
        Router::<Route> {}
    }
}
