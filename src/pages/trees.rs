use dioxus::prelude::*;
use crate::components::Avatar;

#[component]
pub fn Trees() -> Element {
    rsx! {
        div { class: "page-container",
            // Header with actions
            div { class: "flex justify-between items-center mb-6",
                h2 { class: "page-title", "Trees" }

                div { class: "flex gap-2",
                    div { class: "dropdown dropdown-end",
                        label { class: "btn btn-outline", tabindex: "0", "Filter" }
                        ul { class: "dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52", tabindex: "0",
                            li { a { "All Trees" } }
                            li { a { "My Trees" } }
                            li { a { "Shared with me" } }
                        }
                    }

                    button { class: "btn btn-primary",
                        svg { class: "w-5 h-5 mr-2", xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke: "currentColor",
                            path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M12 6v6m0 0v6m0-6h6m-6 0H6" }
                        }
                        "New Tree"
                    }
                }
            }

            // Tree visualization demo
            div { class: "card bg-base-100 shadow-xl mb-8",
                div { class: "card-body",
                    h3 { class: "card-title", "Family Tree" }

                    // Simple tree visualization
                    div { class: "overflow-x-auto py-8",
                        div { class: "flex flex-col items-center",
                            // Root node
                            div { class: "mb-4",
                                Avatar {
                                    initials: "JS",
                                    size_class: "w-16 h-16".to_string(),
                                    color_class: "bg-primary text-primary-content ring ring-primary ring-offset-base-100 ring-offset-2".to_string()
                                }
                                div { class: "text-center mt-2 font-bold", "John Smith" }
                            }

                            // Connector
                            div { class: "w-0.5 h-8 bg-primary" }

                            // Level 1 nodes
                            div { class: "flex gap-16 mb-4",
                                // Child 1
                                div { class: "flex flex-col items-center",
                                    Avatar {
                                        initials: "SJ",
                                        size_class: "w-12 h-12".to_string(),
                                        color_class: "bg-secondary text-secondary-content ring ring-secondary ring-offset-base-100 ring-offset-2".to_string()
                                    }
                                    div { class: "text-center mt-2 font-medium", "Sarah Johnson" }

                                    // Connector
                                    div { class: "w-0.5 h-8 bg-secondary" }

                                    // Level 2 nodes for Child 1
                                    div { class: "flex gap-8",
                                        // Grandchild 1
                                        div { class: "flex flex-col items-center",
                                            Avatar {
                                                initials: "EM",
                                                size_class: "w-10 h-10".to_string(),
                                                color_class: "bg-accent text-accent-content ring ring-accent ring-offset-base-100 ring-offset-2".to_string()
                                            }
                                            div { class: "text-center mt-2 text-sm", "Emma" }
                                        }

                                        // Grandchild 2
                                        div { class: "flex flex-col items-center",
                                            Avatar {
                                                initials: "JB",
                                                size_class: "w-10 h-10".to_string(),
                                                color_class: "bg-accent text-accent-content ring ring-accent ring-offset-base-100 ring-offset-2".to_string()
                                            }
                                            div { class: "text-center mt-2 text-sm", "Jacob" }
                                        }
                                    }
                                }

                                // Child 2
                                div { class: "flex flex-col items-center",
                                    Avatar {
                                        initials: "MC",
                                        size_class: "w-12 h-12".to_string(),
                                        color_class: "bg-secondary text-secondary-content ring ring-secondary ring-offset-base-100 ring-offset-2".to_string()
                                    }
                                    div { class: "text-center mt-2 font-medium", "Michael Chen" }

                                    // Connector
                                    div { class: "w-0.5 h-8 bg-secondary" }

                                    // Level 2 nodes for Child 2
                                    div { class: "flex gap-8",
                                        // Grandchild 3
                                        div { class: "flex flex-col items-center",
                                            Avatar {
                                                initials: "OL",
                                                size_class: "w-10 h-10".to_string(),
                                                color_class: "bg-accent text-accent-content ring ring-accent ring-offset-base-100 ring-offset-2".to_string()
                                            }
                                            div { class: "text-center mt-2 text-sm", "Olivia" }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    div { class: "flex justify-end gap-2 mt-4",
                        button { class: "btn btn-sm",
                            svg { class: "w-4 h-4 mr-1", xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke: "currentColor",
                                path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M4 8V4m0 0h4M4 4l5 5m11-1V4m0 0h-4m4 0l-5 5M4 16v4m0 0h4m-4 0l5-5m11 5l-5-5m5 5v-4m0 4h-4" }
                            }
                            "Expand"
                        }
                        button { class: "btn btn-sm btn-primary", "Edit Tree" }
                    }
                }
            }

            // Other trees list
            div { class: "card bg-base-100 shadow-xl",
                div { class: "card-body",
                    h3 { class: "card-title mb-4", "My Trees" }

                    div { class: "overflow-x-auto",
                        table { class: "table",
                            thead {
                                tr {
                                    th { "Name" }
                                    th { "Type" }
                                    th { "Members" }
                                    th { "Last Updated" }
                                    th { "Actions" }
                                }
                            }
                            tbody {
                                tr {
                                    td {
                                        div { class: "flex items-center gap-3",
                                            Avatar {
                                                initials: "FT",
                                                size_class: "w-12 h-12".to_string(),
                                                color_class: "bg-primary text-primary-content mask mask-squircle".to_string()
                                            }
                                            div {
                                                div { class: "font-bold", "Family Tree" }
                                                div { class: "text-sm opacity-50", "Personal" }
                                            }
                                        }
                                    }
                                    td { "Genealogy" }
                                    td { "12" }
                                    td { "2 days ago" }
                                    td {
                                        div { class: "flex gap-1",
                                            button { class: "btn btn-xs btn-ghost", "View" }
                                            button { class: "btn btn-xs btn-ghost", "Edit" }
                                            button { class: "btn btn-xs btn-ghost text-error", "Delete" }
                                        }
                                    }
                                }
                                tr {
                                    td {
                                        div { class: "flex items-center gap-3",
                                            Avatar {
                                                initials: "PO",
                                                size_class: "w-12 h-12".to_string(),
                                                color_class: "bg-secondary text-secondary-content mask mask-squircle".to_string()
                                            }
                                            div {
                                                div { class: "font-bold", "Project Organization" }
                                                div { class: "text-sm opacity-50", "Work" }
                                            }
                                        }
                                    }
                                    td { "Organizational" }
                                    td { "8" }
                                    td { "1 week ago" }
                                    td {
                                        div { class: "flex gap-1",
                                            button { class: "btn btn-xs btn-ghost", "View" }
                                            button { class: "btn btn-xs btn-ghost", "Edit" }
                                            button { class: "btn btn-xs btn-ghost text-error", "Delete" }
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
