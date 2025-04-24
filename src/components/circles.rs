use dioxus::prelude::*;

#[component]
pub fn Circles() -> Element {
    rsx! {
        div { class: "container my-3",
            // Circles Header
            div { class: "text-center mb-4",
                h2 { class: "display-6 fw-bold", "Circles" }
                p { class: "lead text-muted",
                    "This page will display and manage social circles."
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