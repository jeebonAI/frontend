use dioxus::prelude::*;

#[component]
pub fn SystemInfo() -> Element {
    rsx! {
        div { class: "container mt-4",
            h2 { "System Information" }
            div { class: "card",
                div { class: "card-body",
                    p { "This page displays system information and statistics." }
                    // Placeholder for actual system information
                    div { class: "alert alert-info",
                        "System information will be displayed here."
                    }
                }
            }
        }
    }
}