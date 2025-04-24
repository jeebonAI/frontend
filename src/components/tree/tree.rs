use dioxus::prelude::*;
use super::Person;
use super::{FamilyMember, FamilySection, AddRelativeModal};

#[component]
pub fn Tree() -> Element {
    let mut show_modal = use_signal(|| false);
    let mut target_person_id = use_signal(|| String::new());
    let mut view_mode = use_signal(|| "list"); // Add view mode state: "list" or "tree"
    
    let handle_add_relative = move |id: String| {
        target_person_id.set(id);
        show_modal.set(true);
    };
    
    let handle_close_modal = move |_| {
        show_modal.set(false);
    };
    
    // Sample data
    let central_person = Person {
        id: "central".to_string(),
        name: "Central Person".to_string(),
        birth_year: Some(1980),
        profile_image: None,
        relationship: "self".to_string(),
    };
    
    let parents = vec![
        Person {
            id: "father".to_string(),
            name: "Father".to_string(),
            birth_year: Some(1950),
            profile_image: None,
            relationship: "father".to_string(),
        },
        Person {
            id: "mother".to_string(),
            name: "Mother".to_string(),
            birth_year: Some(1952),
            profile_image: None,
            relationship: "mother".to_string(),
        },
    ];
    
    let siblings = vec![
        Person {
            id: "brother".to_string(),
            name: "Brother".to_string(),
            birth_year: Some(1982),
            profile_image: None,
            relationship: "brother".to_string(),
        },
    ];
    
    let children = vec![
        Person {
            id: "daughter".to_string(),
            name: "Daughter".to_string(),
            birth_year: Some(2010),
            profile_image: None,
            relationship: "daughter".to_string(),
        },
        Person {
            id: "son".to_string(),
            name: "Son".to_string(),
            birth_year: Some(2012),
            profile_image: None,
            relationship: "son".to_string(),
        },
    ];
    
    rsx! {
        div { class: "container my-4 family-tree-container",
            // Add view toggle buttons
            div { class: "d-flex justify-content-between align-items-center mb-3",
                h2 { class: "mb-0", "Family Tree" }
                div { class: "btn-group",
                    button {
                        class: if *view_mode.read() == "list" { "btn btn-primary" } else { "btn btn-outline-primary" },
                        onclick: move |_| view_mode.set("list"),
                        i { class: "bi bi-list me-1" }
                        "List View"
                    }
                    button {
                        class: if *view_mode.read() == "tree" { "btn btn-primary" } else { "btn btn-outline-primary" },
                        onclick: move |_| view_mode.set("tree"),
                        i { class: "bi bi-diagram-3 me-1" }
                        "Tree View"
                    }
                }
            }
            
            // Conditional rendering based on view mode
            if *view_mode.read() == "list" {
                // List view (current implementation)
                div {
                    // Central person (self)
                    div { class: "mb-4",
                        FamilyMember {
                            person: central_person,
                            is_central: true,
                            on_add_relative: handle_add_relative
                        }
                    }
                    
                    // Family sections
                    FamilySection {
                        title: "Parents".to_string(),
                        members: parents,
                        on_add_member: |_| {},
                        on_add_relative: handle_add_relative
                    }
                    
                    FamilySection {
                        title: "Siblings".to_string(),
                        members: siblings,
                        on_add_member: |_| {},
                        on_add_relative: handle_add_relative
                    }
                    
                    FamilySection {
                        title: "Children".to_string(),
                        members: children,
                        on_add_member: |_| {},
                        on_add_relative: handle_add_relative
                    }
                }
            } else {
                // Tree view (visual representation)
                div { class: "family-tree-visual",
                    // Simple visual tree representation
                    div { class: "tree-container",
                        // Parents level
                        div { class: "tree-level parents-level",
                            for parent in parents.iter() {
                                div { class: "tree-node parent-node",
                                    div { class: "tree-node-content",
                                        i { class: "bi bi-person-circle fs-3" }
                                        div { "{parent.name}" }
                                        div { class: "small text-muted", "b. {parent.birth_year.unwrap_or(0)}" }
                                    }
                                }
                            }
                        }
                        
                        // Connection lines from parents to central person
                        div { class: "tree-connections",
                            div { class: "vertical-line" }
                            div { class: "horizontal-line parents-line" }
                        }
                        
                        // Central person level
                        div { class: "tree-level central-level",
                            div { class: "tree-node central-node",
                                div { class: "tree-node-content central-content",
                                    i { class: "bi bi-person-circle fs-2" }
                                    div { "{central_person.name}" }
                                    div { class: "small text-muted", "b. {central_person.birth_year.unwrap_or(0)}" }
                                }
                            }
                        }
                        
                        // Connection lines from central person to children
                        div { class: "tree-connections",
                            div { class: "vertical-line" }
                            div { class: "horizontal-line children-line" }
                        }
                        
                        // Children level
                        div { class: "tree-level children-level",
                            for child in children.iter() {
                                div { class: "tree-node child-node",
                                    div { class: "tree-node-content",
                                        i { class: "bi bi-person-circle fs-3" }
                                        div { "{child.name}" }
                                        div { class: "small text-muted", "b. {child.birth_year.unwrap_or(0)}" }
                                    }
                                }
                            }
                        }
                    }
                    
                    // Add CSS for the tree view
                    style {
                        "
                        .family-tree-visual {{
                            padding: 20px;
                            overflow-x: auto;
                        }}
                        .tree-container {{
                            display: flex;
                            flex-direction: column;
                            align-items: center;
                            min-width: 600px;
                        }}
                        .tree-level {{
                            display: flex;
                            justify-content: center;
                            width: 100%;
                            gap: 20px;
                            margin: 10px 0;
                            position: relative;
                        }}
                        .tree-node {{
                            padding: 10px;
                            border-radius: 8px;
                            background-color: #f8f9fa;
                            border: 1px solid #dee2e6;
                            min-width: 120px;
                            text-align: center;
                            z-index: 2;
                        }}
                        .central-node {{
                            background-color: #e7f5ff;
                            border-color: #74c0fc;
                        }}
                        .tree-connections {{
                            position: relative;
                            height: 40px;
                            width: 100%;
                            display: flex;
                            flex-direction: column;
                            align-items: center;
                        }}
                        .vertical-line {{
                            height: 100%;
                            width: 2px;
                            background-color: #adb5bd;
                        }}
                        .horizontal-line {{
                            position: absolute;
                            height: 2px;
                            background-color: #adb5bd;
                            width: 80%;
                            top: 50%;
                            left: 10%;
                        }}
                        .parents-level, .children-level {{
                            justify-content: space-around;
                        }}
                        "
                    }
                }
            }
            
            // Modal for adding relatives
            if *show_modal.read() {
                AddRelativeModal {
                    show: show_modal,
                    target_person_id: target_person_id,
                    on_close: handle_close_modal
                }
            }
        }
    }
}
