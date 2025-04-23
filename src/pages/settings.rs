use dioxus::prelude::*;

#[component]
pub fn Settings() -> Element {
    rsx! {
        div { class: "page-container",
            h2 { class: "page-title", "Settings" }
            
            // Settings tabs
            div { class: "tabs tabs-boxed mb-6",
                a { class: "tab tab-active", "General" }
                a { class: "tab", "Account" }
                a { class: "tab", "Privacy" }
                a { class: "tab", "Notifications" }
                a { class: "tab", "Advanced" }
            }
            
            // Settings content
            div { class: "grid grid-cols-1 md:grid-cols-3 gap-6",
                // Left column - General settings
                div { class: "md:col-span-2",
                    div { class: "card bg-base-100 shadow-xl",
                        div { class: "card-body",
                            h3 { class: "card-title", "General Settings" }
                            div { class: "divider" }
                            
                            // Theme settings
                            div { class: "form-control w-full max-w-md mb-6",
                                label { class: "label", span { class: "label-text font-medium", "Theme" } }
                                select { class: "select select-bordered w-full",
                                    option { value: "system", selected: true, "System Default" }
                                    option { value: "light", "Light" }
                                    option { value: "dark", "Dark" }
                                }
                                label { class: "label", span { class: "label-text-alt", "Choose your preferred theme or use system settings" } }
                            }
                            
                            // Language settings
                            div { class: "form-control w-full max-w-md mb-6",
                                label { class: "label", span { class: "label-text font-medium", "Language" } }
                                select { class: "select select-bordered w-full",
                                    option { value: "en", selected: true, "English" }
                                    option { value: "fr", "French" }
                                    option { value: "es", "Spanish" }
                                    option { value: "de", "German" }
                                    option { value: "ja", "Japanese" }
                                }
                                label { class: "label", span { class: "label-text-alt", "Select your preferred language" } }
                            }
                            
                            // Time zone settings
                            div { class: "form-control w-full max-w-md mb-6",
                                label { class: "label", span { class: "label-text font-medium", "Time Zone" } }
                                select { class: "select select-bordered w-full",
                                    option { value: "utc", "UTC (Coordinated Universal Time)" }
                                    option { value: "est", selected: true, "EST (Eastern Standard Time)" }
                                    option { value: "pst", "PST (Pacific Standard Time)" }
                                    option { value: "gmt", "GMT (Greenwich Mean Time)" }
                                }
                                label { class: "label", span { class: "label-text-alt", "Select your time zone" } }
                            }
                            
                            // Date format settings
                            div { class: "form-control w-full max-w-md mb-6",
                                label { class: "label", span { class: "label-text font-medium", "Date Format" } }
                                div { class: "flex gap-4",
                                    div { class: "form-control",
                                        label { class: "label cursor-pointer",
                                            span { class: "label-text mr-2", "MM/DD/YYYY" }
                                            input { r#type: "radio", name: "date-format", class: "radio radio-primary", checked: true }
                                        }
                                    }
                                    div { class: "form-control",
                                        label { class: "label cursor-pointer",
                                            span { class: "label-text mr-2", "DD/MM/YYYY" }
                                            input { r#type: "radio", name: "date-format", class: "radio radio-primary" }
                                        }
                                    }
                                    div { class: "form-control",
                                        label { class: "label cursor-pointer",
                                            span { class: "label-text mr-2", "YYYY-MM-DD" }
                                            input { r#type: "radio", name: "date-format", class: "radio radio-primary" }
                                        }
                                    }
                                }
                            }
                            
                            // Accessibility settings
                            div { class: "form-control w-full max-w-md mb-6",
                                label { class: "label", span { class: "label-text font-medium", "Accessibility" } }
                                div { class: "flex flex-col gap-2",
                                    div { class: "form-control",
                                        label { class: "label cursor-pointer justify-start",
                                            input { r#type: "checkbox", class: "checkbox checkbox-primary mr-2" }
                                            span { class: "label-text", "High contrast mode" }
                                        }
                                    }
                                    div { class: "form-control",
                                        label { class: "label cursor-pointer justify-start",
                                            input { r#type: "checkbox", class: "checkbox checkbox-primary mr-2", checked: true }
                                            span { class: "label-text", "Screen reader support" }
                                        }
                                    }
                                    div { class: "form-control",
                                        label { class: "label cursor-pointer justify-start",
                                            input { r#type: "checkbox", class: "checkbox checkbox-primary mr-2" }
                                            span { class: "label-text", "Reduce animations" }
                                        }
                                    }
                                }
                            }
                            
                            div { class: "flex justify-end gap-2 mt-6",
                                button { class: "btn btn-outline", "Reset to Default" }
                                button { class: "btn btn-primary", "Save Changes" }
                            }
                        }
                    }
                }
                
                // Right column - Quick settings and info
                div { class: "md:col-span-1",
                    // Quick settings card
                    div { class: "card bg-base-100 shadow-xl mb-6",
                        div { class: "card-body",
                            h3 { class: "text-lg font-bold mb-4", "Quick Settings" }
                            
                            div { class: "flex justify-between items-center mb-4",
                                span { "Dark Mode" }
                                input { class: "toggle toggle-primary", r#type: "checkbox" }
                            }
                            
                            div { class: "flex justify-between items-center mb-4",
                                span { "Notifications" }
                                input { class: "toggle toggle-primary", r#type: "checkbox", checked: true }
                            }
                            
                            div { class: "flex justify-between items-center mb-4",
                                span { "Sounds" }
                                input { class: "toggle toggle-primary", r#type: "checkbox", checked: true }
                            }
                            
                            div { class: "flex justify-between items-center",
                                span { "Auto-updates" }
                                input { class: "toggle toggle-primary", r#type: "checkbox", checked: true }
                            }
                        }
                    }
                    
                    // App info card
                    div { class: "card bg-base-100 shadow-xl",
                        div { class: "card-body",
                            h3 { class: "text-lg font-bold mb-4", "App Information" }
                            
                            div { class: "flex flex-col gap-2 text-sm",
                                div { class: "flex justify-between",
                                    span { class: "opacity-70", "Version" }
                                    span { class: "font-medium", "1.0.0" }
                                }
                                
                                div { class: "flex justify-between",
                                    span { class: "opacity-70", "Last Updated" }
                                    span { class: "font-medium", "June 15, 2024" }
                                }
                                
                                div { class: "flex justify-between",
                                    span { class: "opacity-70", "Platform" }
                                    span { class: "font-medium", "Web / Android / iOS" }
                                }
                                
                                div { class: "flex justify-between",
                                    span { class: "opacity-70", "Storage Used" }
                                    span { class: "font-medium", "128 MB" }
                                }
                            }
                            
                            div { class: "divider" }
                            
                            div { class: "flex flex-col gap-2",
                                button { class: "btn btn-sm btn-outline w-full", "Check for Updates" }
                                button { class: "btn btn-sm btn-outline w-full", "View Licenses" }
                                button { class: "btn btn-sm btn-error btn-outline w-full", "Clear Cache" }
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
