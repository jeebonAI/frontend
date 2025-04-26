use dioxus::prelude::*;

#[component]
pub fn Circles() -> Element {
    // State for active tab
    let mut active_tab = use_signal(|| "my-circles");

    // Sample circles data
    let my_circles = vec![
        ("Decentralized Technology", 128, "A circle for discussing decentralized technologies and applications."),
        ("Web3 Developers", 256, "Connect with other developers building on Web3 technologies."),
        ("DJibon Community", 512, "Official circle for DJibon users and contributors."),
    ];

    let discover_circles = vec![
        ("Blockchain Enthusiasts", 1024, "Discuss the latest in blockchain technology and applications."),
        ("Crypto Economics", 768, "Explore the economic aspects of cryptocurrencies and tokens."),
        ("Decentralized Governance", 384, "Learn about and discuss governance models in decentralized systems."),
        ("Smart Contract Development", 192, "Share knowledge about smart contract development and security."),
    ];

    rsx! {
        div { class: "container mt-4",
            h1 { "Circles" }
            p { class: "lead", "Connect with communities of like-minded individuals" }

            // Tabs
            ul { class: "nav nav-tabs mb-4",
                li { class: "nav-item",
                    a {
                        class: if *active_tab.read() == "my-circles" { "nav-link active" } else { "nav-link" },
                        href: "#",
                        onclick: move |_| active_tab.set("my-circles"),
                        "My Circles"
                    }
                }
                li { class: "nav-item",
                    a {
                        class: if *active_tab.read() == "discover" { "nav-link active" } else { "nav-link" },
                        href: "#",
                        onclick: move |_| active_tab.set("discover"),
                        "Discover"
                    }
                }
            }

            // Search bar
            div { class: "mb-4",
                div { class: "input-group",
                    span { class: "input-group-text",
                        i { class: "bi bi-search" }
                    }
                    input {
                        class: "form-control",
                        placeholder: "Search circles...",
                        r#type: "text"
                    }
                    if *active_tab.read() == "my-circles" {
                        button { class: "btn btn-primary",
                            i { class: "bi bi-plus-lg me-2" }
                            "Create Circle"
                        }
                    }
                }
            }

            // Circles content
            if *active_tab.read() == "my-circles" {
                div { class: "row",
                    {my_circles.iter().map(|(name, members, description)| {
                        rsx! {
                            div { class: "col-md-6 col-lg-4 mb-4",
                                div { class: "card h-100",
                                    div { class: "card-body",
                                        h5 { class: "card-title", "{name}" }
                                        p { class: "card-text text-muted", "{description}" }
                                    }
                                    div { class: "card-footer bg-transparent d-flex justify-content-between align-items-center",
                                        small { class: "text-muted",
                                            i { class: "bi bi-people me-1" }
                                            "{members} members"
                                        }
                                        a { class: "btn btn-sm btn-outline-primary", href: "#", "View" }
                                    }
                                }
                            }
                        }
                    })}
                }
            } else {
                div { class: "row",
                    {discover_circles.iter().map(|(name, members, description)| {
                        rsx! {
                            div { class: "col-md-6 col-lg-4 mb-4",
                                div { class: "card h-100",
                                    div { class: "card-body",
                                        h5 { class: "card-title", "{name}" }
                                        p { class: "card-text text-muted", "{description}" }
                                    }
                                    div { class: "card-footer bg-transparent d-flex justify-content-between align-items-center",
                                        small { class: "text-muted",
                                            i { class: "bi bi-people me-1" }
                                            "{members} members"
                                        }
                                        a { class: "btn btn-sm btn-primary", href: "#", "Join" }
                                    }
                                }
                            }
                        }
                    })}
                }
            }
        }
    }
}
