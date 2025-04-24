use dioxus::prelude::*;
use super::Person;

#[component]
pub fn FamilySection(
    title: String,
    members: Vec<Person>,
    on_add_member: EventHandler<()>,
    on_add_relative: EventHandler<String>
) -> Element {
    rsx! {
        div { class: "family-section mb-4",
            div { class: "d-flex justify-content-between align-items-center mb-2",
                h5 { class: "mb-0", "{title}" }
                button {
                    class: "btn btn-sm btn-outline-primary",
                    onclick: move |_| on_add_member.call(()),
                    i { class: "bi bi-plus me-1" }
                    "Add {title}"
                }
            }
            
            if members.is_empty() {
                div { class: "text-muted fst-italic", "No {title.to_lowercase()} added yet." }
            } else {
                div {
                    for person in members.iter() {
                        super::FamilyMember {
                            person: person.clone(),
                            is_central: false,
                            on_add_relative: on_add_relative.clone()
                        }
                    }
                }
            }
        }
    }
}
