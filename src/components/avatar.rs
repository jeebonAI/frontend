use dioxus::prelude::*;

/// A simple avatar component that renders initials
#[component]
pub fn Avatar(
    #[props(into)] initials: String,
    #[props(default = "avatar-md".to_string())] size_class: String,
    #[props(default = "bg-primary".to_string())] color_class: String,
) -> Element {
    rsx! {
        div {
            class: "avatar {size_class} {color_class} d-flex align-items-center justify-content-center",
            "{initials}"
        }
    }
}

/// A placeholder avatar with a user icon
#[component]
pub fn UserAvatar(
    #[props(default = "avatar-md".to_string())] size_class: String,
    #[props(default = "bg-primary".to_string())] color_class: String,
    #[props(default = false)] online: bool,
) -> Element {
    let status_indicator = if online {
        rsx! { span { class: "position-absolute top-0 start-100 translate-middle p-1 bg-success border border-light rounded-circle" } }
    } else {
        rsx! { span { class: "position-absolute top-0 start-100 translate-middle p-1 bg-secondary border border-light rounded-circle" } }
    };

    rsx! {
        div { class: "position-relative",
            div {
                class: "avatar {size_class} {color_class} d-flex align-items-center justify-content-center",
                i { class: "bi bi-person" }
            }
            {status_indicator}
        }
    }
}

/// A placeholder avatar for a group or circle
#[component]
pub fn GroupAvatar(
    #[props(default = "avatar-md".to_string())] size_class: String,
    #[props(default = "bg-secondary".to_string())] color_class: String,
) -> Element {
    rsx! {
        div {
            class: "avatar {size_class} {color_class} d-flex align-items-center justify-content-center",
            i { class: "bi bi-people" }
        }
    }
}
