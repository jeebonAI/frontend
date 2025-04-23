use dioxus::prelude::*;
use crate::components::UserAvatar;

#[component]
pub fn Profile() -> Element {
    rsx! {
        div { class: "page-container",
            h2 { class: "page-title", "User Profile" }

            // Profile card
            div { class: "card bg-base-100 shadow-xl mb-8",
                div { class: "card-body",
                    div { class: "flex flex-col md:flex-row gap-6",
                        // Avatar section
                        div { class: "flex flex-col items-center",
                            UserAvatar {
                                size_class: "w-24 h-24",
                                color_class: "bg-primary text-primary-content ring ring-primary ring-offset-base-100 ring-offset-2",
                                online: true
                            }
                            div { class: "mt-4 text-center",
                                h3 { class: "font-bold text-lg", "Jane Doe" }
                                p { class: "text-sm opacity-70", "@janedoe" }
                            }
                            button { class: "btn btn-sm btn-outline mt-2", "Change Photo" }
                        }

                        // Profile info section
                        div { class: "flex-1",
                            div { class: "divider divider-start mb-4", "Profile Information" }

                            // Form fields
                            div { class: "form-control w-full max-w-md mb-4",
                                label { class: "label", span { class: "label-text", "Full Name" } }
                                input { class: "input input-bordered w-full", r#type: "text", value: "Jane Doe", placeholder: "Enter your name" }
                            }

                            div { class: "form-control w-full max-w-md mb-4",
                                label { class: "label", span { class: "label-text", "Email" } }
                                input { class: "input input-bordered w-full", r#type: "email", value: "jane.doe@example.com", placeholder: "Enter your email" }
                            }

                            div { class: "form-control w-full max-w-md mb-4",
                                label { class: "label", span { class: "label-text", "Bio" } }
                                textarea { class: "textarea textarea-bordered w-full", placeholder: "Tell us about yourself", "I'm a software developer with a passion for UI design and user experience." }
                            }

                            div { class: "mt-6",
                                button { class: "btn btn-primary mr-2", "Save Changes" }
                                button { class: "btn btn-outline", "Cancel" }
                            }
                        }
                    }
                }
            }

            // Settings section
            div { class: "card bg-base-100 shadow-xl",
                div { class: "card-body",
                    h3 { class: "card-title", "Account Settings" }

                    div { class: "divider" }

                    // Settings options
                    div { class: "flex flex-col gap-4",
                        // Notification settings
                        div { class: "flex justify-between items-center",
                            div {
                                h4 { class: "font-medium", "Email Notifications" }
                                p { class: "text-sm opacity-70", "Receive email notifications for messages and updates" }
                            }
                            input { class: "toggle toggle-primary", r#type: "checkbox", checked: true }
                        }

                        // Theme settings
                        div { class: "flex justify-between items-center",
                            div {
                                h4 { class: "font-medium", "Dark Mode" }
                                p { class: "text-sm opacity-70", "Toggle between light and dark theme" }
                            }
                            input { class: "toggle toggle-primary", r#type: "checkbox" }
                        }

                        // Privacy settings
                        div { class: "flex justify-between items-center",
                            div {
                                h4 { class: "font-medium", "Profile Privacy" }
                                p { class: "text-sm opacity-70", "Make your profile visible to everyone" }
                            }
                            input { class: "toggle toggle-primary", r#type: "checkbox", checked: true }
                        }
                    }

                    div { class: "mt-6",
                        span { class: "badge badge-warning gap-2", "Coming Soon" }
                    }
                }
            }
        }
    }
}
