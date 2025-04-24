use dioxus::prelude::*;
use super::Person;

#[component]
pub fn AddRelativeModal(
    show: Signal<bool>,
    target_person_id: Signal<String>,
    on_close: EventHandler<()>
) -> Element {
    let mut new_person = use_signal(|| Person {
        id: uuid::Uuid::new_v4().to_string(),
        name: String::new(),
        birth_year: None,
        profile_image: None,
        relationship: String::new(),
    });

    let handle_submit = move |_| {
        // Here you would add the new person to your data store
        // For now, we'll just close the modal
        on_close.call(());
    };

    rsx! {
        div {
            class: if *show.read() { "modal fade show d-block" } else { "modal fade" },
            tabindex: "-1",
            
            div { class: "modal-dialog",
                div { class: "modal-content",
                    div { class: "modal-header",
                        h5 { class: "modal-title", "Add New Relative" }
                        button {
                            r#type: "button",
                            class: "btn-close",
                            onclick: move |_| on_close.call(()),
                        }
                    }
                    
                    div { class: "modal-body",
                        // Display the target person ID for debugging
                        p { "Adding relative to person: {target_person_id.read()}" }
                        
                        div { class: "mb-3",
                            label { class: "form-label", "Name" }
                            input {
                                class: "form-control",
                                r#type: "text",
                                placeholder: "Enter name",
                                oninput: move |evt| new_person.write().name = evt.value().clone(),
                            }
                        }
                        
                        div { class: "mb-3",
                            label { class: "form-label", "Birth Year" }
                            input {
                                class: "form-control",
                                r#type: "number",
                                placeholder: "Enter birth year",
                                oninput: move |evt| {
                                    if let Ok(year) = evt.value().parse::<i32>() {
                                        new_person.write().birth_year = Some(year);
                                    } else {
                                        new_person.write().birth_year = None;
                                    }
                                },
                            }
                        }
                        
                        div { class: "mb-3",
                            label { class: "form-label", "Relationship" }
                            select {
                                class: "form-select",
                                onchange: move |evt| new_person.write().relationship = evt.value().clone(),
                                
                                option { value: "", "Select relationship" }
                                option { value: "parent", "Parent" }
                                option { value: "child", "Child" }
                                option { value: "sibling", "Sibling" }
                                option { value: "spouse", "Spouse" }
                            }
                        }
                    }
                    
                    div { class: "modal-footer",
                        button {
                            r#type: "button",
                            class: "btn btn-secondary",
                            onclick: move |_| on_close.call(()),
                            "Cancel"
                        }
                        button {
                            r#type: "button",
                            class: "btn btn-primary",
                            onclick: handle_submit,
                            "Add Relative"
                        }
                    }
                }
            }
            
            // Modal backdrop
            div {
                class: if *show.read() { "modal-backdrop fade show" } else { "modal-backdrop fade" },
            }
        }
    }
}
