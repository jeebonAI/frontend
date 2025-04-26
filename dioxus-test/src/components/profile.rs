use dioxus::prelude::*;

#[component]
pub fn Profile() -> Element {
    rsx! {
        div { class: "container mt-4",
            h1 { "Profile" }
            
            div { class: "card mb-4",
                div { class: "card-body",
                    div { class: "d-flex align-items-center mb-3",
                        div { class: "me-3",
                            // Profile avatar
                            div { 
                                class: "rounded-circle bg-primary text-white d-flex align-items-center justify-content-center",
                                style: "width: 80px; height: 80px;",
                                i { class: "bi bi-person-fill fs-1" }
                            }
                        }
                        div {
                            h3 { class: "mb-1", "User Name" }
                            p { class: "text-muted mb-0", "@username" }
                        }
                    }
                    
                    div { class: "mb-3",
                        h5 { "Bio" }
                        p { "This is a sample bio for the user profile. Users can add information about themselves here." }
                    }
                    
                    div { class: "mb-3",
                        h5 { "Contact Information" }
                        p { i { class: "bi bi-envelope me-2" } "user@example.com" }
                    }
                    
                    button { class: "btn btn-primary", "Edit Profile" }
                }
            }
            
            div { class: "card mb-4",
                div { class: "card-header", "Activity" }
                div { class: "card-body",
                    div { class: "mb-3 pb-3 border-bottom",
                        div { class: "d-flex align-items-center mb-2",
                            span { class: "badge bg-primary me-2", "Tree" }
                            span { class: "text-muted", "2 days ago" }
                        }
                        p { "Created a new knowledge tree: 'Introduction to Decentralized Networks'" }
                    }
                    
                    div { class: "mb-3 pb-3 border-bottom",
                        div { class: "d-flex align-items-center mb-2",
                            span { class: "badge bg-success me-2", "Circle" }
                            span { class: "text-muted", "5 days ago" }
                        }
                        p { "Joined the 'Decentralized Technology' circle" }
                    }
                    
                    div {
                        div { class: "d-flex align-items-center mb-2",
                            span { class: "badge bg-info me-2", "Comment" }
                            span { class: "text-muted", "1 week ago" }
                        }
                        p { "Commented on 'The Future of Web3'" }
                    }
                }
            }
            
            div { class: "card",
                div { class: "card-header", "Connections" }
                div { class: "card-body",
                    div { class: "d-flex flex-wrap",
                        div { class: "me-3 mb-3 text-center",
                            div { 
                                class: "rounded-circle bg-secondary text-white d-flex align-items-center justify-content-center mb-2",
                                style: "width: 50px; height: 50px;",
                                i { class: "bi bi-person" }
                            }
                            p { class: "mb-0 small", "User 1" }
                        }
                        
                        div { class: "me-3 mb-3 text-center",
                            div { 
                                class: "rounded-circle bg-secondary text-white d-flex align-items-center justify-content-center mb-2",
                                style: "width: 50px; height: 50px;",
                                i { class: "bi bi-person" }
                            }
                            p { class: "mb-0 small", "User 2" }
                        }
                        
                        div { class: "me-3 mb-3 text-center",
                            div { 
                                class: "rounded-circle bg-secondary text-white d-flex align-items-center justify-content-center mb-2",
                                style: "width: 50px; height: 50px;",
                                i { class: "bi bi-person" }
                            }
                            p { class: "mb-0 small", "User 3" }
                        }
                        
                        div { class: "me-3 mb-3 text-center",
                            div { 
                                class: "rounded-circle bg-primary text-white d-flex align-items-center justify-content-center mb-2",
                                style: "width: 50px; height: 50px;",
                                i { class: "bi bi-plus-lg" }
                            }
                            p { class: "mb-0 small", "Add New" }
                        }
                    }
                }
            }
        }
    }
}
