use dioxus::prelude::*;
use crate::components::UserAvatar;

#[component]
pub fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        div { class: "container px-4 py-3",
            // Hero section
            div { class: "card text-center bg-light mb-4",
                div { class: "card-body py-5",
                    h1 { class: "display-5 fw-bold", "Welcome to DJibon" }
                    p { class: "lead my-4",
                        "This is a prototype of the DJibon Personal Assistance and Communication Tool. "
                        "It demonstrates the basic structure and navigation of the application."
                    }
                    button { class: "btn btn-primary btn-lg", "Get Started" }
                }
            }

            // Interactive demo card
            div { class: "card mb-4 shadow-sm",
                div { class: "card-body",
                    h2 { class: "card-title", "Interactive Demo" }
                    p { class: "card-text", "Click the button below to increment the counter:" }

                    div { class: "d-flex align-items-center gap-3 my-3",
                        button {
                            class: "btn btn-primary",
                            onclick: move |_| count += 1,
                            "Increment"
                        }

                        // Counter display with badge
                        span { class: "badge bg-primary rounded-pill fs-6", "Count: {count}" }
                    }
                }
            }

            // Features section with cards
            div { class: "mb-4",
                h2 { class: "h3 mb-4", "Key Features" }

                div { class: "row row-cols-1 row-cols-md-2 row-cols-lg-3 g-4",
                    // Feature 1
                    div { class: "col",
                        div { class: "card h-100 shadow-sm",
                            div { class: "card-body",
                                div { class: "d-flex align-items-center justify-content-center rounded-circle bg-primary text-white mb-3",
                                    style: "width: 48px; height: 48px;",
                                    i { class: "bi bi-person-fill" }
                                }
                                h3 { class: "card-title h5", "User Profiles" }
                                p { class: "card-text", "Complete user profiles with authentication and personalization options." }
                            }
                        }
                    }

                    // Feature 2
                    div { class: "col",
                        div { class: "card h-100 shadow-sm",
                            div { class: "card-body",
                                div { class: "d-flex align-items-center justify-content-center rounded-circle bg-info text-white mb-3",
                                    style: "width: 48px; height: 48px;",
                                    i { class: "bi bi-chat-dots-fill" }
                                }
                                h3 { class: "card-title h5", "Messaging" }
                                p { class: "card-text", "Real-time messaging with support for text, images, and other media types." }
                            }
                        }
                    }

                    // Feature 3
                    div { class: "col",
                        div { class: "card h-100 shadow-sm",
                            div { class: "card-body",
                                div { class: "d-flex align-items-center justify-content-center rounded-circle bg-success text-white mb-3",
                                    style: "width: 48px; height: 48px;",
                                    i { class: "bi bi-telephone-fill" }
                                }
                                h3 { class: "card-title h5", "Calls" }
                                p { class: "card-text", "Audio and video calls with excellent quality and reliability." }
                            }
                        }
                    }

                    // Feature 4
                    div { class: "col",
                        div { class: "card h-100 shadow-sm",
                            div { class: "card-body",
                                div { class: "d-flex align-items-center justify-content-center rounded-circle bg-primary text-white mb-3",
                                    style: "width: 48px; height: 48px;",
                                    i { class: "bi bi-people-fill" }
                                }
                                h3 { class: "card-title h5", "Circles" }
                                p { class: "card-text", "Create and manage social circles for group communication and sharing." }
                            }
                        }
                    }

                    // Feature 5
                    div { class: "col",
                        div { class: "card h-100 shadow-sm",
                            div { class: "card-body",
                                div { class: "d-flex align-items-center justify-content-center rounded-circle bg-secondary text-white mb-3",
                                    style: "width: 48px; height: 48px;",
                                    i { class: "bi bi-diagram-3-fill" }
                                }
                                h3 { class: "card-title h5", "Trees" }
                                p { class: "card-text", "Organize information and relationships in hierarchical tree structures." }
                            }
                        }
                    }

                    // Feature 6
                    div { class: "col",
                        div { class: "card h-100 shadow-sm",
                            div { class: "card-body",
                                div { class: "d-flex align-items-center justify-content-center rounded-circle bg-info text-white mb-3",
                                    style: "width: 48px; height: 48px;",
                                    i { class: "bi bi-cloud-check-fill" }
                                }
                                h3 { class: "card-title h5", "Online/Offline" }
                                p { class: "card-text", "Works seamlessly both online and offline with automatic synchronization." }
                            }
                        }
                    }
                }
            }

            // Stats section
            div { class: "row row-cols-1 row-cols-md-3 g-4 mt-2 mb-4",
                // Stat 1
                div { class: "col",
                    div { class: "card text-center h-100 border-0 shadow-sm",
                        div { class: "card-body",
                            div { class: "text-primary mb-2",
                                i { class: "bi bi-heart-fill fs-1" }
                            }
                            h5 { class: "card-title", "Total Users" }
                            div { class: "display-6 fw-bold text-primary", "25.6K" }
                            p { class: "card-text text-muted small", "21% more than last month" }
                        }
                    }
                }

                // Stat 2
                div { class: "col",
                    div { class: "card text-center h-100 border-0 shadow-sm",
                        div { class: "card-body",
                            div { class: "text-secondary mb-2",
                                i { class: "bi bi-lightning-charge-fill fs-1" }
                            }
                            h5 { class: "card-title", "Page Views" }
                            div { class: "display-6 fw-bold text-secondary", "2.6M" }
                            p { class: "card-text text-muted small", "14% more than last month" }
                        }
                    }
                }

                // Stat 3
                div { class: "col",
                    div { class: "card text-center h-100 border-0 shadow-sm",
                        div { class: "card-body",
                            div { class: "mb-2 position-relative d-inline-block",
                                UserAvatar {
                                    size_class: "avatar-lg",
                                    color_class: "bg-secondary",
                                    online: true
                                }
                            }
                            h5 { class: "card-title", "Tasks done" }
                            div { class: "display-6 fw-bold", "86%" }
                            p { class: "card-text text-secondary small", "31 tasks remaining" }
                        }
                    }
                }
            }
        }
    }
}
