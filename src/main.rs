use dioxus::prelude::*;
use routes::Route;

mod components;
mod routes;

// Define assets
const BOOTSTRAP_CSS: Asset = asset!("/assets/bootstrap.min.css");
const BOOTSTRAP_JS: Asset = asset!("/assets/bootstrap.bundle.min.js");
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MANIFEST: Asset = asset!("/assets/manifest.json");
// Removed Cytoscape assets as they're now loaded in the Tree component

fn main() {
    // Initialize logger for debugging with more verbose output
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));

    // Add global error handler for WebAssembly
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::prelude::*;

        // Set up a global error handler to catch and log hydration errors
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_namespace = window)]
            fn addEventListener(event: &str, handler: &Closure<dyn FnMut(JsValue)>);
        }

        // Create an error handler closure
        let error_handler = Closure::wrap(Box::new(move |e: JsValue| {
            log::error!("Uncaught error: {:?}", e);
            // The application will continue running despite the error
        }) as Box<dyn FnMut(JsValue)>);

        // Register the error handler
        addEventListener("error", &error_handler);

        // Leak the closure so it remains valid for the lifetime of the application
        error_handler.forget();
    }

    // Launch the application with standard hydration
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Add error handling for hydration issues
    use_effect(|| {
        #[cfg(target_arch = "wasm32")]
        {
            log::info!("App component mounted - setting up hydration error handling");
        }

        // No cleanup needed
        ()
    });

    rsx! {
        document::Link { rel: "stylesheet", href: BOOTSTRAP_CSS }
        document::Link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/bootstrap-icons@1.11.3/font/bootstrap-icons.css"
        }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "manifest", href: MANIFEST }
        document::Meta { name: "theme-color", content: "#4285f4" }
        document::Script { src: BOOTSTRAP_JS }
        Router::<Route> {}
    }
}


