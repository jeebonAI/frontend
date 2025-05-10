use dioxus::prelude::*;

#[component]
fn MessagePreview(name: String, message: String, msg_time: String, unread: bool, avatar: String) -> Element {
    // No need to check theme - Bootstrap handles it automatically

    rsx! {
        div {
            class: "d-flex align-items-center p-2 border-bottom",
            img {
                class: "rounded-circle",
                src: avatar,
                style: "width: 50px; height: 50px; object-fit: cover;"
            }
            div {
                class: "ms-3 flex-grow-1",
                div {
                    class: "d-flex justify-content-between",
                    span {
                        class: "fw-bold",
                        {name}
                    }
                    span {
                        class: "text-muted small",
                        {msg_time}
                    }
                }
                div {
                    class: "d-flex align-items-center",
                    p {
                        class: if unread {
                            "m-0 fw-bold text-truncate"
                        } else {
                            "m-0 text-secondary text-truncate"
                        },
                        style: "max-width: 250px;",
                        {message}
                    }
                    if unread {
                        span {
                            class: "ms-2 bg-primary rounded-circle",
                            style: "display: inline-block; width: 8px; height: 8px;"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn MessagesTab() -> Element {
    rsx! {
        div { class: "messages-tab",
            MessagePreview {
                name: "Sarah Johnson",
                message: "Hey, how's the project coming along?",
                msg_time: "10:30 AM",
                unread: true,
                avatar: "https://i.pravatar.cc/150?img=1"
            }
            MessagePreview {
                name: "David Lee",
                message: "I've sent you the files you requested",
                msg_time: "Yesterday",
                unread: false,
                avatar: "https://i.pravatar.cc/150?img=2"
            }
            MessagePreview {
                name: "Tech Team",
                message: "Meeting scheduled for tomorrow at 2 PM",
                msg_time: "Yesterday",
                unread: false,
                avatar: "https://i.pravatar.cc/150?img=3"
            }
            MessagePreview {
                name: "Alex Wong",
                message: "Thanks for your help!",
                msg_time: "Monday",
                unread: false,
                avatar: "https://i.pravatar.cc/150?img=4"
            }
        }
    }
}

#[component]
fn AudioCallsTab() -> Element {
    rsx! {
        div { class: "p-3",
            // Placeholder for audio call history
            div {
                class: "text-center py-5",
                i {
                    class: "bi bi-telephone-x display-1 text-muted"
                }
                p {
                    class: "mt-3 fw-bold",
                    "No recent audio calls"
                }
                p {
                    class: "text-muted",
                    "Your audio call history will appear here"
                }
            }
        }
    }
}

#[component]
fn VideoCallsTab() -> Element {
    rsx! {
        div { class: "p-3",
            // Placeholder for video call history
            div {
                class: "text-center py-5",
                i {
                    class: "bi bi-camera-video-off display-1 text-muted"
                }
                p {
                    class: "mt-3 fw-bold",
                    "No recent video calls"
                }
                p {
                    class: "text-muted",
                    "Your video call history will appear here"
                }
            }
        }
    }
}

// Add this component to export Comms for the router
#[component]
pub fn Comms() -> Element {
    let mut active_tab = use_signal(|| "messages");

    rsx! {
        div {
            class: "h-100",
            // Top section with tabs and plus icon
            div {
                class: "d-flex justify-content-between align-items-center p-3",
                // Tabs as icons with better styling
                div { class: "nav nav-tabs border-0 flex-row",
                    button {
                        class: if *active_tab.read() == "messages" {
                            "nav-link active text-primary"
                        } else {
                            "nav-link text-secondary"
                        },
                        onclick: move |_| active_tab.set("messages"),
                        i { class: "bi bi-chat-dots fs-5" }
                    }
                    button {
                        class: if *active_tab.read() == "audio" {
                            "nav-link active text-primary"
                        } else {
                            "nav-link text-secondary"
                        },
                        onclick: move |_| active_tab.set("audio"),
                        i { class: "bi bi-telephone fs-5" }
                    }
                    button {
                        class: if *active_tab.read() == "video" {
                            "nav-link active text-primary"
                        } else {
                            "nav-link text-secondary"
                        },
                        onclick: move |_| active_tab.set("video"),
                        i { class: "bi bi-camera-video fs-5" }
                    }
                }

                // Plus icon button
                button {
                    class: "btn btn-primary rounded-circle d-flex align-items-center justify-content-center",
                    style: "width: 40px; height: 40px;",
                    onclick: move |_| {
                        match *active_tab.read() {
                            "messages" => log::info!("New message button clicked"),
                            "audio" => log::info!("New audio call button clicked"),
                            "video" => log::info!("New video call button clicked"),
                            _ => {}
                        }
                    },
                    i { class: "bi bi-plus fs-4" }
                }
            }

            // Tab content container with appropriate background
            div {
                class: "rounded-3 mx-3 overflow-hidden bg-body-tertiary",

                // Tab content
                if *active_tab.read() == "messages" {
                    MessagesTab {}
                } else if *active_tab.read() == "audio" {
                    AudioCallsTab {}
                } else {
                    VideoCallsTab {}
                }
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
                .bg-dark-subtle {{
                    background-color: #212529;
                }}
                "
            }
        }
    }
}
