use dioxus::prelude::*;

#[component]
pub fn Tree() -> Element {
    // State for active tab
    let mut active_tab = use_signal(|| "my-trees");

    // Sample trees data
    let my_trees = vec![
        ("Introduction to Decentralized Networks", "Educational", "A comprehensive guide to understanding decentralized networks."),
        ("Web3 Development Roadmap", "Technical", "Step-by-step guide for becoming a Web3 developer."),
        ("Blockchain Fundamentals", "Educational", "Core concepts and principles of blockchain technology."),
    ];

    let discover_trees = vec![
        ("Cryptocurrency Basics", "Educational", "Learn about different cryptocurrencies and how they work."),
        ("Smart Contract Security", "Technical", "Best practices for secure smart contract development."),
        ("DeFi Explained", "Educational", "Understanding decentralized finance applications and protocols."),
        ("NFT Creation Guide", "Creative", "How to create, mint, and sell NFTs."),
    ];

    rsx! {
        div { class: "container mt-2",
            // Tabs
            ul { class: "nav nav-tabs mb-4",
                li { class: "nav-item",
                    a {
                        class: if *active_tab.read() == "my-trees" { "nav-link active" } else { "nav-link" },
                        href: "#",
                        onclick: move |_| active_tab.set("my-trees"),
                        "My Trees"
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

            // Search and create
            div { class: "mb-4",
                div { class: "input-group",
                    span { class: "input-group-text",
                        i { class: "bi bi-search" }
                    }
                    input {
                        class: "form-control",
                        placeholder: "Search trees...",
                        r#type: "text"
                    }
                    if *active_tab.read() == "my-trees" {
                        button { class: "btn btn-primary",
                            i { class: "bi bi-plus-lg me-2" }
                            "Create Tree"
                        }
                    }
                }
            }

            // Trees content
            if *active_tab.read() == "my-trees" {
                div { class: "row",
                    {my_trees.iter().map(|(name, category, description)| {
                        rsx! {
                            div { class: "col-md-6 col-lg-4 mb-4",
                                div { class: "card h-100",
                                    div { class: "card-body",
                                        h5 { class: "card-title", "{name}" }
                                        span { class: "badge bg-info mb-2", "{category}" }
                                        p { class: "card-text text-muted", "{description}" }
                                    }
                                    div { class: "card-footer bg-transparent d-flex justify-content-between align-items-center",
                                        small { class: "text-muted", "Last updated 3 days ago" }
                                        a { class: "btn btn-sm btn-outline-primary", href: "#", "View" }
                                    }
                                }
                            }
                        }
                    })}
                }
            } else {
                div { class: "row",
                    {discover_trees.iter().map(|(name, category, description)| {
                        rsx! {
                            div { class: "col-md-6 col-lg-4 mb-4",
                                div { class: "card h-100",
                                    div { class: "card-body",
                                        h5 { class: "card-title", "{name}" }
                                        span { class: "badge bg-info mb-2", "{category}" }
                                        p { class: "card-text text-muted", "{description}" }
                                    }
                                    div { class: "card-footer bg-transparent d-flex justify-content-between align-items-center",
                                        div {
                                            small { class: "text-muted me-2", "Created by: User123" }
                                            small { class: "text-muted", "4.5 ★" }
                                        }
                                        a { class: "btn btn-sm btn-primary", href: "#", "Fork" }
                                    }
                                }
                            }
                        }
                    })}
                }
            }

            // Tree visualization placeholder
            div { class: "card mt-4",
                div { class: "card-header", "Tree Visualization" }
                div { class: "card-body text-center p-5",
                    p { class: "mb-4", "Select a tree to view its visualization" }
                    div { class: "tree-placeholder",
                        style: "height: 300px; background-color: #f8f9fa; border-radius: 8px; display: flex; align-items: center; justify-content: center;",
                        i { class: "bi bi-diagram-3 fs-1 text-muted" }
                    }
                }
            }
        }
    }
}
