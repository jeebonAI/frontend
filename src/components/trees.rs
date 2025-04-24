use dioxus::prelude::*;

#[component]
pub fn Trees() -> Element {
    rsx! {
        div { class: "container my-3",
            // Trees Header
            div { class: "text-center mb-4",
                h2 { class: "display-6 fw-bold", "Trees" }
                p { class: "lead text-muted",
                    "This page will display and manage hierarchical trees."
                }
            }

            // Coming Soon Card
            div { class: "card shadow-sm",
                div { class: "card-body text-center",
                    p { class: "text-danger mb-0", "Coming Soon" }
                }
            }
        }
    }
}