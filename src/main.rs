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
    wasm_logger::init(wasm_logger::Config::default());
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Register service worker on component mount
    use_effect(|| {
        register_service_worker();
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

fn register_service_worker() {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen_futures::JsFuture;
        use js_sys::Promise;
        
        let _ = wasm_bindgen_futures::spawn_local(async move {
            // Updated path without leading slash
            let js_code = r#"
                (function() {
                    if ('serviceWorker' in navigator) {
                        return navigator.serviceWorker.register('sw.js')
                            .then(function(reg) {
                                console.log('Service worker registered successfully', reg);
                                return reg;
                            })
                            .catch(function(err) {
                                console.error('Service worker registration failed:', err);
                                throw err;
                            });
                    } else {
                        console.warn('Service workers are not supported');
                        throw new Error('Service workers not supported');
                    }
                })()
            "#;
            
            match js_sys::eval(js_code) {
                Ok(promise_val) => {
                    if let Some(promise) = promise_val.dyn_ref::<Promise>() {
                        match JsFuture::from(promise.clone()).await {
                            Ok(_) => log::info!("Service worker registered successfully"),
                            Err(e) => log::error!("Service worker registration failed: {:?}", e),
                        }
                    } else {
                        log::error!("Failed to get Promise from eval result");
                    }
                },
                Err(e) => log::error!("Failed to evaluate JS: {:?}", e),
            }
        });
    }
}
