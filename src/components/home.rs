use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        div { class: "home-page",
            h2 { "Welcome to DJibon" }
            p { class: "intro",
                "This is a prototype of the DJibon Personal Assistance and Communication Tool. "
                "It demonstrates the basic structure and navigation of the application."
            }
            div { class: "demo-card",
                h3 { "Interactive Demo" }
                p { "Click the button below to increment the counter:" }
                div { class: "counter",
                    button {
                        onclick: move |_| count += 1,
                        "Increment"
                    }
                    p { "Count: {count}" }
                }
            }
            div { class: "features-list",
                h3 { "Key Features:" }
                ul {
                    li { "User profiles and authentication" }
                    li { "Real-time messaging" }
                    li { "Audio/video calls" }
                    li { "Circles for group communication" }
                    li { "Trees for hierarchical organization" }
                    li { "Works online and offline" }
                }
            }
        }
    }
}