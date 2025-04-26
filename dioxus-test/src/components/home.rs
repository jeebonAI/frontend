use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "container mt-2",
            div { class: "row",
                div { class: "col-md-6 mb-4",
                    div { class: "card h-100",
                        div { class: "card-body",
                            h5 { class: "card-title", "Trees" }
                            p { class: "card-text", "Explore and create knowledge trees to organize information." }
                            a { class: "btn btn-primary", href: "/trees", "Explore Trees" }
                        }
                    }
                }
                div { class: "col-md-6 mb-4",
                    div { class: "card h-100",
                        div { class: "card-body",
                            h5 { class: "card-title", "Circles" }
                            p { class: "card-text", "Join circles to connect with like-minded individuals." }
                            a { class: "btn btn-primary", href: "/circles", "Explore Circles" }
                        }
                    }
                }
            }

            div { class: "row",
                div { class: "col-md-6 mb-4",
                    div { class: "card h-100",
                        div { class: "card-body",
                            h5 { class: "card-title", "Communications" }
                            p { class: "card-text", "Connect and communicate with others in your network." }
                            a { class: "btn btn-primary", href: "/comms", "Open Communications" }
                        }
                    }
                }
                div { class: "col-md-6 mb-4",
                    div { class: "card h-100",
                        div { class: "card-body",
                            h5 { class: "card-title", "Profile" }
                            p { class: "card-text", "View and update your profile information." }
                            a { class: "btn btn-primary", href: "/profile", "View Profile" }
                        }
                    }
                }
            }
        }
    }
}
