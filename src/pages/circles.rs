use dioxus::prelude::*;
use crate::components::{UserAvatar, GroupAvatar};

#[component]
pub fn Circles() -> Element {
    rsx! {
        div { class: "page-container",
            // Header with tabs
            div { class: "flex justify-between items-center mb-6",
                h2 { class: "page-title", "Circles" }

                div { class: "tabs tabs-boxed",
                    a { class: "tab tab-active", "My Circles" }
                    a { class: "tab", "Discover" }
                    a { class: "tab", "Invites" }
                }

                button { class: "btn btn-primary",
                    svg { class: "w-5 h-5 mr-2", xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke: "currentColor",
                        path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M12 6v6m0 0v6m0-6h6m-6 0H6" }
                    }
                    "Create Circle"
                }
            }

            // Circles grid
            div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                // Circle 1
                div { class: "card bg-base-100 shadow-xl",
                    figure { class: "px-10 pt-10 flex justify-center",
                        GroupAvatar {
                            size_class: "w-32 h-32",
                            color_class: "bg-primary text-primary-content"
                        }
                    }
                    div { class: "card-body",
                        h3 { class: "card-title",
                            "Family"
                            div { class: "badge badge-secondary", "8 members" }
                        }
                        p { "Keep in touch with your family members and share important updates." }
                        div { class: "flex -space-x-4 mt-4",
                            div { class: "-mr-2",
                                UserAvatar {
                                    size_class: "w-8 h-8",
                                    color_class: "ring ring-primary ring-offset-base-100 ring-offset-2",
                                    online: true
                                }
                            }
                            div { class: "-mr-2",
                                UserAvatar {
                                    size_class: "w-8 h-8",
                                    color_class: "bg-secondary text-secondary-content ring ring-primary ring-offset-base-100 ring-offset-2",
                                    online: false
                                }
                            }
                            div { class: "-mr-2",
                                UserAvatar {
                                    size_class: "w-8 h-8",
                                    color_class: "bg-accent text-accent-content ring ring-primary ring-offset-base-100 ring-offset-2",
                                    online: true
                                }
                            }
                            div { class: "avatar placeholder",
                                div { class: "w-8 h-8 rounded-full bg-neutral-focus text-neutral-content ring ring-primary ring-offset-base-100 ring-offset-2",
                                    span { "+5" }
                                }
                            }
                        }
                        div { class: "card-actions justify-end mt-4",
                            button { class: "btn btn-sm btn-primary", "View" }
                            button { class: "btn btn-sm btn-outline", "Share" }
                        }
                    }
                }

                // Circle 2
                div { class: "card bg-base-100 shadow-xl",
                    figure { class: "px-10 pt-10 flex justify-center",
                        GroupAvatar {
                            size_class: "w-32 h-32",
                            color_class: "bg-secondary text-secondary-content"
                        }
                    }
                    div { class: "card-body",
                        h3 { class: "card-title",
                            "Work Team"
                            div { class: "badge badge-secondary", "12 members" }
                        }
                        p { "Collaborate with your work team on projects and tasks." }
                        div { class: "flex -space-x-4 mt-4",
                            div { class: "-mr-2",
                                UserAvatar {
                                    size_class: "w-8 h-8",
                                    color_class: "ring ring-primary ring-offset-base-100 ring-offset-2",
                                    online: true
                                }
                            }
                            div { class: "-mr-2",
                                UserAvatar {
                                    size_class: "w-8 h-8",
                                    color_class: "bg-secondary text-secondary-content ring ring-primary ring-offset-base-100 ring-offset-2",
                                    online: false
                                }
                            }
                            div { class: "avatar placeholder",
                                div { class: "w-8 h-8 rounded-full bg-neutral-focus text-neutral-content ring ring-primary ring-offset-base-100 ring-offset-2",
                                    span { "+10" }
                                }
                            }
                        }
                        div { class: "card-actions justify-end mt-4",
                            button { class: "btn btn-sm btn-primary", "View" }
                            button { class: "btn btn-sm btn-outline", "Share" }
                        }
                    }
                }

                // Circle 3
                div { class: "card bg-base-100 shadow-xl",
                    figure { class: "px-10 pt-10 flex justify-center",
                        GroupAvatar {
                            size_class: "w-32 h-32",
                            color_class: "bg-accent text-accent-content"
                        }
                    }
                    div { class: "card-body",
                        h3 { class: "card-title",
                            "Friends"
                            div { class: "badge badge-secondary", "15 members" }
                        }
                        p { "Stay connected with your friends and plan activities together." }
                        div { class: "flex -space-x-4 mt-4",
                            div { class: "-mr-2",
                                UserAvatar {
                                    size_class: "w-8 h-8",
                                    color_class: "ring ring-primary ring-offset-base-100 ring-offset-2",
                                    online: true
                                }
                            }
                            div { class: "-mr-2",
                                UserAvatar {
                                    size_class: "w-8 h-8",
                                    color_class: "bg-secondary text-secondary-content ring ring-primary ring-offset-base-100 ring-offset-2",
                                    online: true
                                }
                            }
                            div { class: "-mr-2",
                                UserAvatar {
                                    size_class: "w-8 h-8",
                                    color_class: "bg-accent text-accent-content ring ring-primary ring-offset-base-100 ring-offset-2",
                                    online: false
                                }
                            }
                            div { class: "avatar placeholder",
                                div { class: "w-8 h-8 rounded-full bg-neutral-focus text-neutral-content ring ring-primary ring-offset-base-100 ring-offset-2",
                                    span { "+12" }
                                }
                            }
                        }
                        div { class: "card-actions justify-end mt-4",
                            button { class: "btn btn-sm btn-primary", "View" }
                            button { class: "btn btn-sm btn-outline", "Share" }
                        }
                    }
                }

                // Create new circle card
                div { class: "card bg-base-100 shadow-xl border-2 border-dashed border-base-300",
                    div { class: "card-body flex items-center justify-center text-center h-full",
                        div { class: "py-8",
                            div { class: "rounded-full bg-base-200 w-16 h-16 flex items-center justify-center mx-auto mb-4",
                                svg { class: "w-8 h-8 text-primary", xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke: "currentColor",
                                    path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M12 6v6m0 0v6m0-6h6m-6 0H6" }
                                }
                            }
                            h3 { class: "font-bold text-lg mb-2", "Create New Circle" }
                            p { class: "mb-4", "Start a new group for your friends, family, or colleagues." }
                            button { class: "btn btn-primary", "Create Circle" }
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
