use dioxus::prelude::*;

#[component]
pub fn Comms() -> Element {
    // State for active chat
    let mut active_chat = use_signal(|| 0);

    // Sample chat data
    let chats = vec![
        ("Alice", "Hey, how are you?", "10:30 AM", 2),
        ("Bob", "Did you see the new update?", "Yesterday", 0),
        ("Charlie", "Meeting at 3pm", "Yesterday", 0),
        ("Decentralized Tech", "New member joined", "2 days ago", 5),
        ("Web3 Group", "Check out this article", "3 days ago", 1),
    ];

    rsx! {
        div { class: "container-fluid mt-4",
            h1 { "Communications" }

            div { class: "row",
                // Chat list sidebar
                div { class: "col-md-4 col-lg-3 mb-4",
                    div { class: "card",
                        div { class: "card-header d-flex justify-content-between align-items-center",
                            h5 { class: "mb-0", "Chats" }
                            button { class: "btn btn-sm btn-primary",
                                i { class: "bi bi-pencil-square" }
                            }
                        }

                        div { class: "list-group list-group-flush",
                            // Search box
                            div { class: "p-2",
                                div { class: "input-group",
                                    span { class: "input-group-text",
                                        i { class: "bi bi-search" }
                                    }
                                    input {
                                        class: "form-control",
                                        placeholder: "Search chats...",
                                        r#type: "text"
                                    }
                                }
                            }

                            // Chat list
                            {chats.iter().enumerate().map(|(index, (name, message, time, unread))| {
                                let is_active = *active_chat.read() == index;
                                let chat_class = if is_active {
                                    "list-group-item list-group-item-action active"
                                } else {
                                    "list-group-item list-group-item-action"
                                };

                                rsx! {
                                    a {
                                        class: chat_class,
                                        href: "#",
                                        onclick: move |_| active_chat.set(index),

                                        div { class: "d-flex w-100 justify-content-between align-items-center",
                                            h6 { class: "mb-1", "{name}" }
                                            small {
                                                if *unread > 0 {
                                                    span { class: "badge bg-primary rounded-pill me-2", "{unread}" }
                                                }
                                                "{time}"
                                            }
                                        }
                                        p { class: "mb-1 text-truncate", "{message}" }
                                    }
                                }
                            })}
                        }
                    }
                }

                // Chat content
                div { class: "col-md-8 col-lg-9",
                    div { class: "card",
                        // Chat header
                        div { class: "card-header bg-light",
                            div { class: "d-flex align-items-center",
                                div {
                                    class: "rounded-circle bg-primary text-white d-flex align-items-center justify-content-center me-2",
                                    style: "width: 40px; height: 40px;",
                                    i { class: "bi bi-person" }
                                }
                                div {
                                    h5 { class: "mb-0",
                                        if *active_chat.read() < chats.len() {
                                            "{chats[*active_chat.read()].0}"
                                        } else {
                                            "Select a chat"
                                        }
                                    }
                                    small { class: "text-muted", "Online" }
                                }
                            }
                        }

                        // Chat messages
                        div { class: "card-body bg-light",
                            style: "height: 400px; overflow-y: auto;",

                            if *active_chat.read() < chats.len() {
                                // Sample messages
                                div { class: "d-flex flex-column",
                                    // Received message
                                    div { class: "d-flex mb-3",
                                        div {
                                            class: "rounded-circle bg-secondary text-white d-flex align-items-center justify-content-center me-2",
                                            style: "width: 32px; height: 32px; flex-shrink: 0;",
                                            i { class: "bi bi-person" }
                                        }
                                        div {
                                            div { class: "bg-white rounded p-2 px-3 mb-1",
                                                "{chats[*active_chat.read()].1}"
                                            }
                                            small { class: "text-muted", "{chats[*active_chat.read()].2}" }
                                        }
                                    }

                                    // Sent message
                                    div { class: "d-flex flex-row-reverse mb-3",
                                        div {
                                            div { class: "bg-primary text-white rounded p-2 px-3 mb-1",
                                                "Thanks for letting me know!"
                                            }
                                            small { class: "text-muted", "10:32 AM" }
                                        }
                                    }

                                    // Another received message
                                    div { class: "d-flex mb-3",
                                        div {
                                            class: "rounded-circle bg-secondary text-white d-flex align-items-center justify-content-center me-2",
                                            style: "width: 32px; height: 32px; flex-shrink: 0;",
                                            i { class: "bi bi-person" }
                                        }
                                        div {
                                            div { class: "bg-white rounded p-2 px-3 mb-1",
                                                "No problem! Let me know if you need anything else."
                                            }
                                            small { class: "text-muted", "10:33 AM" }
                                        }
                                    }
                                }
                            } else {
                                div { class: "d-flex justify-content-center align-items-center h-100",
                                    p { class: "text-muted", "Select a chat to start messaging" }
                                }
                            }
                        }

                        // Chat input
                        div { class: "card-footer",
                            div { class: "input-group",
                                button { class: "btn btn-outline-secondary",
                                    i { class: "bi bi-paperclip" }
                                }
                                input {
                                    class: "form-control",
                                    placeholder: "Type a message...",
                                    r#type: "text"
                                }
                                button { class: "btn btn-primary",
                                    i { class: "bi bi-send" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
