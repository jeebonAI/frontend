use dioxus::prelude::*;
use routes::Route;

mod components;
mod routes;

// Define assets
const BOOTSTRAP_CSS: Asset = asset!("/assets/bootstrap.min.css");
const BOOTSTRAP_ICONS_CSS: Asset = asset!("/assets/bootstrap-icons.css");
const BOOTSTRAP_JS: Asset = asset!("/assets/bootstrap.bundle.min.js");
const FAVICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: BOOTSTRAP_CSS }
        document::Link { rel: "stylesheet", href: BOOTSTRAP_ICONS_CSS }
        document::Link { rel: "icon", href: FAVICON }
        document::Script { src: BOOTSTRAP_JS }
        Router::<Route> {}
    }
}
