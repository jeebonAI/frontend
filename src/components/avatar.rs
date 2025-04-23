use dioxus::prelude::*;

/// A simple avatar component that renders an SVG with initials
#[component]
pub fn Avatar(
    #[props(into)] initials: String,
    #[props(default = "w-10 h-10".to_string())] size_class: String,
    #[props(default = "bg-primary text-primary-content".to_string())] color_class: String,
) -> Element {
    rsx! {
        div {
            class: "avatar {size_class}",
            div {
                class: "rounded-full {color_class} flex items-center justify-center",
                span {
                    class: "text-center font-bold",
                    "{initials}"
                }
            }
        }
    }
}

/// A placeholder avatar with a user icon
#[component]
pub fn UserAvatar(
    #[props(default = "w-10 h-10".to_string())] size_class: String,
    #[props(default = "bg-primary text-primary-content".to_string())] color_class: String,
    #[props(default = false)] online: bool,
) -> Element {
    let status_class = if online { "online" } else { "offline" };

    rsx! {
        div {
            class: "avatar {status_class}",
            div {
                class: "{size_class} rounded-full {color_class} flex items-center justify-center",
                svg {
                    class: "w-1/2 h-1/2",
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke: "currentColor",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        d: "M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
                    }
                }
            }
        }
    }
}

/// A placeholder avatar for a group or circle
#[component]
pub fn GroupAvatar(
    #[props(default = "w-10 h-10".to_string())] size_class: String,
    #[props(default = "bg-secondary text-secondary-content".to_string())] color_class: String,
) -> Element {
    rsx! {
        div {
            class: "avatar",
            div {
                class: "{size_class} rounded-full {color_class} flex items-center justify-center",
                svg {
                    class: "w-1/2 h-1/2",
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke: "currentColor",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        d: "M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"
                    }
                }
            }
        }
    }
}
