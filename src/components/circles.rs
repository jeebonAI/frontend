use dioxus::prelude::*;

#[component]
fn CirclePreview(name: String, description: String, member_count: i32, icon: String) -> Element {
    rsx! {
        div { class: "circle-preview",
            div { class: "circle-icon",
                i { class: "bi {icon}" }
            }
            div { class: "circle-content",
                div { class: "circle-header",
                    span { class: "name", {name} }
                    span { class: "member-count", "{member_count} members" }
                }
                p { class: "description", {description} }
            }
            
            style {
                "
                .circle-preview {{
                    display: flex;
                    padding: 15px;
                    border-bottom: 1px solid #eee;
                    align-items: center;
                }}
                .circle-icon {{
                    width: 50px;
                    height: 50px;
                    border-radius: 50%;
                    background-color: #f0f0f0;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    font-size: 1.5rem;
                }}
                .circle-content {{
                    margin-left: 15px;
                    flex: 1;
                }}
                .circle-header {{
                    display: flex;
                    justify-content: space-between;
                    margin-bottom: 5px;
                }}
                .name {{
                    font-weight: bold;
                }}
                .member-count {{
                    color: #777;
                    font-size: 0.8rem;
                }}
                .description {{
                    margin: 0;
                    color: #555;
                    font-size: 0.9rem;
                }}
                "
            }
        }
    }
}

#[component]
fn PersonalCirclesTab() -> Element {
    rsx! {
        div { class: "circles-tab",
            CirclePreview {
                name: "Contacts".to_string(),
                description: "Your personal contacts list".to_string(),
                member_count: 42,
                icon: "bi-person-lines-fill".to_string()
            }
            CirclePreview {
                name: "Family".to_string(),
                description: "Close family members".to_string(),
                member_count: 8,
                icon: "bi-house-heart".to_string()
            }
            CirclePreview {
                name: "Work".to_string(),
                description: "Professional contacts".to_string(),
                member_count: 17,
                icon: "bi-briefcase".to_string()
            }
        }
    }
}

#[component]
fn PrivateCirclesTab() -> Element {
    rsx! {
        div { class: "circles-tab",
            CirclePreview {
                name: "Project Alpha".to_string(),
                description: "Development team for Project Alpha".to_string(),
                member_count: 6,
                icon: "bi-lock".to_string()
            }
            CirclePreview {
                name: "Book Club".to_string(),
                description: "Monthly book discussions".to_string(),
                member_count: 12,
                icon: "bi-book".to_string()
            }
            
            // Empty state if no private circles
            div { class: "text-center py-3 text-muted d-none",
                p { "You don't have any private circles yet" }
                p { "Create one with the + button above" }
            }
        }
    }
}

#[component]
fn PublicCirclesTab() -> Element {
    rsx! {
        div { class: "circles-tab",
            CirclePreview {
                name: "Tech News".to_string(),
                description: "Latest in technology and development".to_string(),
                member_count: 156,
                icon: "bi-globe".to_string()
            }
            CirclePreview {
                name: "Rust Community".to_string(),
                description: "Discussions about Rust programming".to_string(),
                member_count: 324,
                icon: "bi-gear".to_string()
            }
            
            // Empty state if no public circles
            div { class: "text-center py-3 text-muted d-none",
                p { "You haven't joined any public circles yet" }
                p { "Discover public circles with the search feature" }
            }
        }
    }
}

#[component]
pub fn Circles() -> Element {
    let mut active_tab = use_signal(|| "personal");
    
    rsx! {
        div { class: "circles-page p-3",
            // Top section with tabs and plus icon
            div { class: "d-flex justify-content-between align-items-center mb-4",
                // Tabs as icons with better styling
                div { class: "nav nav-tabs border-0 flex-row",
                    button { 
                        class: if *active_tab.read() == "personal" { "nav-link active text-primary" } else { "nav-link text-secondary" },
                        onclick: move |_| active_tab.set("personal"),
                        i { class: "bi bi-person", style: "font-size: 1.2rem;" }
                        span { class: "ms-2 d-none d-md-inline", "Personal" }
                    }
                    button { 
                        class: if *active_tab.read() == "private" { "nav-link active text-primary" } else { "nav-link text-secondary" },
                        onclick: move |_| active_tab.set("private"),
                        i { class: "bi bi-lock", style: "font-size: 1.2rem;" }
                        span { class: "ms-2 d-none d-md-inline", "Private" }
                    }
                    button { 
                        class: if *active_tab.read() == "public" { "nav-link active text-primary" } else { "nav-link text-secondary" },
                        onclick: move |_| active_tab.set("public"),
                        i { class: "bi bi-globe", style: "font-size: 1.2rem;" }
                        span { class: "ms-2 d-none d-md-inline", "Public" }
                    }
                }
                
                // Plus icon button
                button {
                    class: "btn btn-primary rounded-circle",
                    style: "width: 40px; height: 40px; padding: 0;",
                    onclick: move |_| {
                        match *active_tab.read() {
                            "personal" => log::info!("Create personal circle"),
                            "private" => log::info!("Create private circle"),
                            "public" => log::info!("Join public circle"),
                            _ => {}
                        }
                    },
                    i { class: "bi bi-plus", style: "font-size: 1.5rem;" }
                }
            }
            
            // Tab content
            if *active_tab.read() == "personal" {
                PersonalCirclesTab {}
            } else if *active_tab.read() == "private" {
                PrivateCirclesTab {}
            } else {
                PublicCirclesTab {}
            }
            
            style {
                "
                .nav-tabs .nav-link {{
                    border: none;
                    padding: 0.5rem 1rem;
                    margin-right: 0.5rem;
                }}
                .nav-tabs .nav-link.active {{
                    background-color: transparent;
                    border-bottom: 2px solid #0d6efd;
                }}
                "
            }
        }
    }
}
