use dioxus::prelude::*;

#[component]
pub fn Messages() -> Element {
    rsx! {
        div { class: "page-container",
            h2 { class: "page-title", "Messages" }
            
            // Messages interface
            div { class: "card bg-base-100 shadow-xl",
                div { class: "card-body p-0",
                    // Chat interface with sidebar and main chat area
                    div { class: "grid grid-cols-1 md:grid-cols-4 min-h-[500px]",
                        // Contacts sidebar
                        div { class: "bg-base-200 p-4 md:col-span-1 border-r border-base-300",
                            // Search bar
                            div { class: "form-control mb-4",
                                div { class: "input-group",
                                    input { class: "input input-bordered w-full", r#type: "text", placeholder: "Search..." }
                                    button { class: "btn btn-square",
                                        svg { class: "w-5 h-5", xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke: "currentColor",
                                            path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" }
                                        }
                                    }
                                }
                            }
                            
                            // Contact list
                            div { class: "space-y-2",
                                // Contact 1 - active
                                div { class: "flex items-center gap-3 p-2 rounded-lg bg-primary bg-opacity-10 cursor-pointer",
                                    div { class: "avatar online",
                                        div { class: "w-10 rounded-full",
                                            img { src: "https://daisyui.com/images/stock/photo-1534528741775-53994a69daeb.jpg" }
                                        }
                                    }
                                    div { class: "flex-1 min-w-0",
                                        p { class: "font-medium truncate", "Sarah Johnson" }
                                        p { class: "text-xs opacity-70 truncate", "Sure, let's meet tomorrow at 10" }
                                    }
                                    div { class: "badge badge-sm badge-primary", "3" }
                                }
                                
                                // Contact 2
                                div { class: "flex items-center gap-3 p-2 rounded-lg hover:bg-base-300 cursor-pointer",
                                    div { class: "avatar offline",
                                        div { class: "w-10 rounded-full",
                                            img { src: "https://daisyui.com/images/stock/photo-1534528741775-53994a69daeb.jpg" }
                                        }
                                    }
                                    div { class: "flex-1 min-w-0",
                                        p { class: "font-medium truncate", "Michael Chen" }
                                        p { class: "text-xs opacity-70 truncate", "I'll send you the files when I get home" }
                                    }
                                }
                                
                                // Contact 3
                                div { class: "flex items-center gap-3 p-2 rounded-lg hover:bg-base-300 cursor-pointer",
                                    div { class: "avatar online",
                                        div { class: "w-10 rounded-full",
                                            img { src: "https://daisyui.com/images/stock/photo-1534528741775-53994a69daeb.jpg" }
                                        }
                                    }
                                    div { class: "flex-1 min-w-0",
                                        p { class: "font-medium truncate", "Alex Rodriguez" }
                                        p { class: "text-xs opacity-70 truncate", "Did you see the latest update?" }
                                    }
                                    div { class: "badge badge-sm badge-primary", "1" }
                                }
                            }
                        }
                        
                        // Chat area
                        div { class: "flex flex-col md:col-span-3",
                            // Chat header
                            div { class: "p-4 border-b border-base-300 flex items-center justify-between",
                                div { class: "flex items-center gap-3",
                                    div { class: "avatar online",
                                        div { class: "w-10 rounded-full",
                                            img { src: "https://daisyui.com/images/stock/photo-1534528741775-53994a69daeb.jpg" }
                                        }
                                    }
                                    div {
                                        p { class: "font-medium", "Sarah Johnson" }
                                        p { class: "text-xs opacity-70", "Online" }
                                    }
                                }
                                div { class: "flex gap-2",
                                    button { class: "btn btn-sm btn-ghost btn-circle",
                                        svg { class: "w-5 h-5", xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke: "currentColor",
                                            path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z" }
                                        }
                                    }
                                    button { class: "btn btn-sm btn-ghost btn-circle",
                                        svg { class: "w-5 h-5", xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke: "currentColor",
                                            path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z" }
                                        }
                                    }
                                    button { class: "btn btn-sm btn-ghost btn-circle",
                                        svg { class: "w-5 h-5", xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke: "currentColor",
                                            path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M12 5v.01M12 12v.01M12 19v.01M12 6a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2z" }
                                        }
                                    }
                                }
                            }
                            
                            // Messages
                            div { class: "flex-1 p-4 overflow-y-auto space-y-4",
                                // Received message
                                div { class: "chat chat-start",
                                    div { class: "chat-image avatar",
                                        div { class: "w-10 rounded-full",
                                            img { src: "https://daisyui.com/images/stock/photo-1534528741775-53994a69daeb.jpg" }
                                        }
                                    }
                                    div { class: "chat-header",
                                        "Sarah Johnson"
                                        time { class: "text-xs opacity-50 ml-1", "12:45" }
                                    }
                                    div { class: "chat-bubble", "Hi there! How are you doing today?" }
                                    div { class: "chat-footer opacity-50", "Delivered" }
                                }
                                
                                // Sent message
                                div { class: "chat chat-end",
                                    div { class: "chat-image avatar",
                                        div { class: "w-10 rounded-full",
                                            img { src: "https://daisyui.com/images/stock/photo-1534528741775-53994a69daeb.jpg" }
                                        }
                                    }
                                    div { class: "chat-header",
                                        "You"
                                        time { class: "text-xs opacity-50 ml-1", "12:46" }
                                    }
                                    div { class: "chat-bubble chat-bubble-primary", "I'm doing well, thanks for asking! How about you?" }
                                    div { class: "chat-footer opacity-50", "Seen" }
                                }
                                
                                // Received message
                                div { class: "chat chat-start",
                                    div { class: "chat-image avatar",
                                        div { class: "w-10 rounded-full",
                                            img { src: "https://daisyui.com/images/stock/photo-1534528741775-53994a69daeb.jpg" }
                                        }
                                    }
                                    div { class: "chat-header",
                                        "Sarah Johnson"
                                        time { class: "text-xs opacity-50 ml-1", "12:47" }
                                    }
                                    div { class: "chat-bubble", "I'm great! Just wondering if you'd like to meet up tomorrow to discuss the project?" }
                                    div { class: "chat-footer opacity-50", "Delivered" }
                                }
                                
                                // Sent message
                                div { class: "chat chat-end",
                                    div { class: "chat-image avatar",
                                        div { class: "w-10 rounded-full",
                                            img { src: "https://daisyui.com/images/stock/photo-1534528741775-53994a69daeb.jpg" }
                                        }
                                    }
                                    div { class: "chat-header",
                                        "You"
                                        time { class: "text-xs opacity-50 ml-1", "12:48" }
                                    }
                                    div { class: "chat-bubble chat-bubble-primary", "Sure, let's meet tomorrow at 10. Does that work for you?" }
                                    div { class: "chat-footer opacity-50", "Seen" }
                                }
                            }
                            
                            // Message input
                            div { class: "p-4 border-t border-base-300",
                                div { class: "flex items-center gap-2",
                                    button { class: "btn btn-circle btn-ghost",
                                        svg { class: "w-5 h-5", xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke: "currentColor",
                                            path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M15.172 7l-6.586 6.586a2 2 0 102.828 2.828l6.414-6.586a4 4 0 00-5.656-5.656l-6.415 6.585a6 6 0 108.486 8.486L20.5 13" }
                                        }
                                    }
                                    input { class: "input input-bordered flex-1", r#type: "text", placeholder: "Type a message..." }
                                    button { class: "btn btn-circle btn-primary",
                                        svg { class: "w-5 h-5", xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke: "currentColor",
                                            path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M12 19l9 2-9-18-9 18 9-2zm0 0v-8" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            div { class: "mt-6 text-center",
                span { class: "badge badge-warning gap-2", "Demo Only" }
            }
        }
    }
}
