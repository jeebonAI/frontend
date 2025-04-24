use dioxus::prelude::*;
use routes::Route;

mod components;
mod routes;

// Define assets
const BOOTSTRAP_CSS: Asset = asset!("/assets/bootstrap.min.css");
const BOOTSTRAP_JS: Asset = asset!("/assets/bootstrap.bundle.min.js");
const FAVICON: Asset = asset!("/assets/favicon.ico");
// Remove the bootstrap-icons.css asset since we'll use CDN

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: BOOTSTRAP_CSS }
        // Use CDN for Bootstrap Icons
        document::Link { 
            rel: "stylesheet", 
            href: "https://cdn.jsdelivr.net/npm/bootstrap-icons@1.11.3/font/bootstrap-icons.css" 
        }
        document::Link { rel: "icon", href: FAVICON }
        document::Script { src: BOOTSTRAP_JS }
        Router::<Route> {}
    }
}
