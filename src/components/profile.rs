use dioxus::prelude::*;

#[component]
pub fn Profile() -> Element {
    // Mock data - would come from a user store in a real app
    let mut user_name = use_signal(|| String::from("Jane Doe"));
    let mut bio = use_signal(|| String::from("Communication specialist and family historian. I love connecting people and preserving family stories."));
    let mut location = use_signal(|| String::from("San Francisco, CA"));
    let joined_date = use_signal(|| String::from("April 2025"));
    let mut edit_mode = use_signal(|| false);
    
    // Stats
    let connections = 127;
    let family_members = 42;
    let circles = 8;
    
    rsx! {
        div { class: "profile-container",
            // Profile header with cover photo
            div { class: "profile-header position-relative mb-5",
                div { class: "cover-photo bg-primary bg-gradient w-100",
                    style: "height: 180px; border-radius: 0 0 10px 10px;"
                }
                
                // Profile picture
                div { class: "profile-picture position-absolute start-50 translate-middle-x",
                    style: "bottom: -60px;",
                    div { class: "rounded-circle bg-light d-flex justify-content-center align-items-center border border-3 border-white",
                        style: "width: 120px; height: 120px;",
                        i { class: "bi bi-person-circle text-primary", style: "font-size: 80px;" }
                    }
                }
            }
            
            // Profile info section
            div { class: "profile-info text-center mb-4",
                // Edit button
                div { class: "d-flex justify-content-end mb-2",
                    button {
                        class: "btn btn-sm btn-outline-primary",
                        onclick: move |_| {
                            let current = *edit_mode.read();
                            edit_mode.set(!current);
                        },
                        if *edit_mode.read() {
                            "Save Profile"
                        } else {
                            "Edit Profile"
                        }
                    }
                }
                
                // User name
                if *edit_mode.read() {
                    input {
                        class: "form-control form-control-lg text-center mb-2",
                        value: "{user_name.read()}",
                        oninput: move |evt| user_name.set(evt.value().clone())
                    }
                } else {
                    h2 { class: "mb-1", "{user_name.read()}" }
                }
                
                // Bio
                if *edit_mode.read() {
                    textarea {
                        class: "form-control mb-3",
                        rows: "3",
                        value: "{bio.read()}",
                        oninput: move |evt| bio.set(evt.value().clone())
                    }
                } else {
                    p { class: "text-muted mb-3", "{bio.read()}" }
                }
                
                // Location and join date
                div { class: "d-flex justify-content-center gap-3 text-muted small mb-3",
                    if *edit_mode.read() {
                        div { class: "d-flex align-items-center",
                            i { class: "bi bi-geo-alt me-1" }
                            input {
                                class: "form-control form-control-sm",
                                value: "{location.read()}",
                                oninput: move |evt| location.set(evt.value().clone())
                            }
                        }
                    } else {
                        div { class: "d-flex align-items-center",
                            i { class: "bi bi-geo-alt me-1" }
                            span { "{location.read()}" }
                        }
                    }
                    div { class: "d-flex align-items-center",
                        i { class: "bi bi-calendar3 me-1" }
                        span { "Joined {joined_date.read()}" }
                    }
                }
                
                // Stats
                div { class: "d-flex justify-content-center gap-4 mb-4",
                    div { class: "text-center",
                        div { class: "fw-bold", "{connections}" }
                        div { class: "small text-muted", "Connections" }
                    }
                    div { class: "text-center",
                        div { class: "fw-bold", "{family_members}" }
                        div { class: "small text-muted", "Family Members" }
                    }
                    div { class: "text-center",
                        div { class: "fw-bold", "{circles}" }
                        div { class: "small text-muted", "Circles" }
                    }
                }
            }
            
            // Tabs for different sections
            ul { class: "nav nav-tabs mb-4",
                li { class: "nav-item",
                    a { class: "nav-link active", href: "#", "About" }
                }
                li { class: "nav-item",
                    a { class: "nav-link", href: "#", "Family Tree" }
                }
                li { class: "nav-item",
                    a { class: "nav-link", href: "#", "Circles" }
                }
                li { class: "nav-item",
                    a { class: "nav-link", href: "#", "Activity" }
                }
            }
            
            // About section
            div { class: "about-section",
                h4 { "About" }
                
                div { class: "card mb-3",
                    div { class: "card-body",
                        h5 { class: "card-title", "Contact Information" }
                        
                        div { class: "mb-2",
                            div { class: "fw-bold", "Email" }
                            div { "jane.doe@example.com" }
                        }
                        
                        div { class: "mb-2",
                            div { class: "fw-bold", "Phone" }
                            div { "+1 (555) 123-4567" }
                        }
                        
                        div {
                            div { class: "fw-bold", "Communication Preferences" }
                            div { "Text messages for urgent matters, email for everything else" }
                        }
                    }
                }
                
                div { class: "card mb-3",
                    div { class: "card-body",
                        h5 { class: "card-title", "Personal Information" }
                        
                        div { class: "mb-2",
                            div { class: "fw-bold", "Birthday" }
                            div { "April 15" }
                        }
                        
                        div { class: "mb-2",
                            div { class: "fw-bold", "Languages" }
                            div { "English, Spanish" }
                        }
                        
                        div {
                            div { class: "fw-bold", "Interests" }
                            div { "Family history, photography, hiking, cooking" }
                        }
                    }
                }
                
                div { class: "card",
                    div { class: "card-body",
                        h5 { class: "card-title", "Privacy Settings" }
                        
                        div { class: "form-check form-switch mb-2",
                            input { class: "form-check-input", type: "checkbox", id: "shareProfile", checked: true }
                            label { class: "form-check-label", for: "shareProfile", "Share profile with connections" }
                        }
                        
                        div { class: "form-check form-switch mb-2",
                            input { class: "form-check-input", type: "checkbox", id: "shareActivity", checked: false }
                            label { class: "form-check-label", for: "shareActivity", "Share activity updates" }
                        }
                        
                        div { class: "form-check form-switch",
                            input { class: "form-check-input", type: "checkbox", id: "shareLocation", checked: false }
                            label { class: "form-check-label", for: "shareLocation", "Share location with family members" }
                        }
                    }
                }
            }
        }
    }
}
