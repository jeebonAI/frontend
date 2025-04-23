use dioxus::prelude::*;

#[component]
pub fn AppLogo() -> Element {
    rsx! {
        svg {
            class: "app-logo",
            width: "40",
            height: "40",
            view_box: "0 0 40 40",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",

            // Circle background
            circle {
                cx: "20",
                cy: "20",
                r: "18",
                fill: "#6C5CE7",
                stroke: "#FFFFFF",
                stroke_width: "2"
            }

            // Letter D
            path {
                d: "M12 10H18C21.3137 10 24 12.6863 24 16V24C24 27.3137 21.3137 30 18 30H12V10Z",
                fill: "#FFFFFF"
            }

            // Connection lines
            path {
                d: "M26 14H30C31.1046 14 32 14.8954 32 16V16C32 17.1046 31.1046 18 30 18H26V14Z",
                fill: "#FFFFFF"
            }

            path {
                d: "M26 22H30C31.1046 22 32 22.8954 32 24V24C32 25.1046 31.1046 26 30 26H26V22Z",
                fill: "#FFFFFF"
            }

            // Dots
            circle {
                cx: "26",
                cy: "16",
                r: "2",
                fill: "#00CEC9"
            }

            circle {
                cx: "26",
                cy: "24",
                r: "2",
                fill: "#00CEC9"
            }
        }
    }
}
