use dioxus::prelude::*;

#[component]
fn MessagePreview(name: String, message: String, msg_time: String, unread: bool, avatar: String) -> Element {
    rsx! {
        div { class: "message-preview",
            img { class: "avatar", src: avatar, style: "width: 50px; height: 50px; border-radius: 50%; object-fit: cover;" }
            div { class: "message-content",
                div { class: "message-header",
                    span { class: "name", {name} }
                    span { class: "time", {msg_time} }
                }
                div { class: "message-text",
                    p { 
                        class: if unread { "preview unread" } else { "preview" },
                        {message}
                    }
                    if unread {
                        span { class: "unread-badge" }
                    }
                }
            }
            
            // Add inline styles for the message preview layout
            style {
                "
                .message-preview {{
                    display: flex;
                    padding: 10px;
                    border-bottom: 1px solid #eee;
                    align-items: center;
                }}
                .message-content {{
                    margin-left: 15px;
                    flex: 1;
                }}
                .message-header {{
                    display: flex;
                    justify-content: space-between;
                }}
                .name {{
                    font-weight: bold;
                }}
                .time {{
                    color: #777;
                    font-size: 0.8rem;
                }}
                .preview {{
                    margin: 0;
                    color: #555;
                    font-size: 0.9rem;
                    white-space: nowrap;
                    overflow: hidden;
                    text-overflow: ellipsis;
                    max-width: 250px;
                }}
                .unread {{
                    font-weight: bold;
                    color: #000;
                }}
                .unread-badge {{
                    display: inline-block;
                    width: 8px;
                    height: 8px;
                    background-color: #0d6efd;
                    border-radius: 50%;
                    margin-left: 5px;
                }}
                "
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
        div { class: "calls-tab",
            // Placeholder for audio call history
            div { class: "text-center py-5 text-muted",
                i { class: "bi bi-telephone-x display-1" }
                p { class: "mt-3", "No recent audio calls" }
                p { "Your audio call history will appear here" }
            }
        }
    }
}

#[component]
fn VideoCallsTab() -> Element {
    rsx! {
        div { class: "calls-tab",
            // Placeholder for video call history
            div { class: "text-center py-5 text-muted",
                i { class: "bi bi-camera-video-off display-1" }
                p { class: "mt-3", "No recent video calls" }
                p { "Your video call history will appear here" }
            }
        }
    }
}

// Add this component to export Comms for the router
#[component]
pub fn Comms() -> Element {
    let mut active_tab = use_signal(|| "messages");
    
    rsx! {
        div { class: "comms-page p-3",
            // Top section with tabs and plus icon
            div { class: "d-flex justify-content-between align-items-center mb-4",
                // Tabs as icons with better styling
                div { class: "nav nav-tabs border-0 flex-row",
                    button { 
                        class: if *active_tab.read() == "messages" { "nav-link active text-primary" } else { "nav-link text-secondary" },
                        onclick: move |_| active_tab.set("messages"),
                        i { class: "bi bi-chat-dots", style: "font-size: 1.2rem;" }
                    }
                    button { 
                        class: if *active_tab.read() == "audio" { "nav-link active text-primary" } else { "nav-link text-secondary" },
                        onclick: move |_| active_tab.set("audio"),
                        i { class: "bi bi-telephone", style: "font-size: 1.2rem;" }
                    }
                    button { 
                        class: if *active_tab.read() == "video" { "nav-link active text-primary" } else { "nav-link text-secondary" },
                        onclick: move |_| active_tab.set("video"),
                        i { class: "bi bi-camera-video", style: "font-size: 1.2rem;" }
                    }
                }
                
                // Plus icon button
                button {
                    class: "btn btn-primary rounded-circle",
                    style: "width: 40px; height: 40px; padding: 0;",
                    onclick: move |_| {
                        match *active_tab.read() {
                            "messages" => log::info!("New message button clicked"),
                            "audio" => log::info!("New audio call button clicked"),
                            "video" => log::info!("New video call button clicked"),
                            _ => {}
                        }
                    },
                    i { class: "bi bi-plus", style: "font-size: 1.5rem;" }
                }
            }
            
            // Tab content
            if *active_tab.read() == "messages" {
                MessagesTab {}
            } else if *active_tab.read() == "audio" {
                AudioCallsTab {}
            } else {
                VideoCallsTab {}
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
