use dioxus::prelude::*;

#[component]
pub fn SystemInfo() -> Element {
    // You can replace these with actual values from your application
    let version = "0.1.0";
    let release_date = "2023-06-15";
    let framework = "Dioxus";
    let framework_version = "0.6.0";
    
    rsx! {
        div { class: "system-info-page",
            h2 { "System Information" }
            
            div { class: "card mb-4",
                div { class: "card-header",
                    "Application Details"
                }
                div { class: "card-body",
                    table { class: "table table-striped",
                        tbody {
                            tr {
                                th { scope: "row", "Application Name" }
                                td { "DJibon PACT" }
                            }
                            tr {
                                th { scope: "row", "Version" }
                                td { "{version}" }
                            }
                            tr {
                                th { scope: "row", "Release Date" }
                                td { "{release_date}" }
                            }
                        }
                    }
                }
            }
            
            div { class: "card",
                div { class: "card-header",
                    "Technical Details"
                }
                div { class: "card-body",
                    table { class: "table table-striped",
                        tbody {
                            tr {
                                th { scope: "row", "Framework" }
                                td { "{framework}" }
                            }
                            tr {
                                th { scope: "row", "Framework Version" }
                                td { "{framework_version}" }
                            }
                            tr {
                                th { scope: "row", "Build Type" }
                                td { "Development" }
                            }
                            tr {
                                th { scope: "row", "Platform" }
                                td { "Web (WASM)" }
                            }
                        }
                    }
                }
            }
        }
    }
}