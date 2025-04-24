use dioxus::prelude::*;

#[component]
pub fn Messages() -> Element {
    rsx! {
        div { class: "container my-3",
            h1 { class: "display-1", "Display 1" }
            p { class: "lead", "This is a lead paragraph." }
            div { class: "card",
                div { class: "card-body",
                    p { "This is a card." }
                    button { class: "btn btn-primary", "Test Button" }
                }
            }
            style {
                ".container {{ max-width: 1200px; margin: 0 auto; padding: 0 15px; }} .my-3 {{ margin-top: 1rem; margin-bottom: 1rem; }} .display-1 {{ font-size: 6rem; font-weight: 300; line-height: 1.2; }} .lead {{ font-size: 1.25rem; font-weight: 300; }} .card {{ border: 1px solid #dee2e6; border-radius: 0.25rem; }} .card-body {{ padding: 1.25rem; }} .btn-primary {{ background-color: #0d6efd; border-color: #0d6efd; color: white; padding: 0.375rem 0.75rem; border-radius: 0.25rem; }}"
            }
        }
    }
}