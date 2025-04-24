use dioxus::prelude::*;

#[component]
pub fn Settings() -> Element {
    rsx! {
        div { class: "container my-3",
            // Settings Header
            div { class: "text-center mb-4",
                h2 { class: "display-6 fw-bold", "Settings" }
                p { class: "lead text-muted",
                    "This page will allow configuration of application settings."
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