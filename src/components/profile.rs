use dioxus::prelude::*;

#[component]
pub fn Profile() -> Element {
    rsx! {
        div { class: "profile-page",
            h2 { "User Profile" }
            p { "This page will display and allow editing of the user profile." }
            p { class: "coming-soon", "Coming Soon" }
        }
    }
}