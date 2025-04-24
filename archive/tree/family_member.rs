use dioxus::prelude::*;
use super::Person;

#[component]
pub fn FamilyMember(person: Person, is_central: bool, on_add_relative: EventHandler<String>) -> Element {
    let mut show_options = use_signal(|| false);
    
    let toggle_options = move |_| {
        // Fix the borrow issue by using a temporary variable
        let current = *show_options.read();
        show_options.set(!current);
    };
    
    let add_relative = move |_| {
        on_add_relative.call(person.id.clone());
        show_options.set(false);
    };
    
    rsx! {
        div { 
            class: if is_central { "family-member central" } else { "family-member" },
            onclick: toggle_options,
            
            // Avatar/profile image
            div { class: "member-avatar",
                if let Some(image) = &person.profile_image {
                    img { src: "{image}", alt: "{person.name}" }
                } else {
                    i { class: "bi bi-person" }
                }
            }
            
            // Member info
            div { class: "member-info",
                div { class: "member-name", "{person.name}" }
                if let Some(year) = person.birth_year {
                    div { class: "member-year", "b. {year}" }
                }
                div { class: "member-relation", "{person.relationship}" }
            }
            
            // Options menu (shows on click)
            if *show_options.read() {
                div { class: "member-options",
                    div { class: "options-menu",
                        button { 
                            class: "btn btn-sm btn-outline-primary",
                            onclick: add_relative,
                            i { class: "bi bi-plus-circle me-1" }
                            "Add Relative"
                        }
                        button { 
                            class: "btn btn-sm btn-outline-secondary ms-2",
                            i { class: "bi bi-pencil me-1" }
                            "Edit"
                        }
                    }
                }
            }
        }
    }
}
