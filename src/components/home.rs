use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Home() -> Element {

    rsx! {
        div { 
            class: "container mt-2",
            div { 
                class: "row row-cols-2 g-3",
                // Profile Box
                div {
                    class: "col",
                    Link {
                        to: Route::Profile {},
                        div {
                            class: "card text-center h-100",
                                    div {
                                class: "card-body d-flex flex-column justify-content-center align-items-center",
                                style: "min-height: 150px;",
                                i { class: "bi bi-person fs-1 mb-2" }
                                span { "Profile" }
                            }
                        }
                    }
                }

                // Communications Box
                div { class: "col",
                    Link {
                        to: Route::Comms {},
                        div {
                            class: "card text-center h-100",
                                    div {
                                class: "card-body d-flex flex-column justify-content-center align-items-center",
                                style: "min-height: 150px;",
                                i { class: "bi bi-chat-dots fs-1 mb-2" }
                                span { "Comms" }
                            }
                        }
                    }
                }

                // Circles Box
                div { class: "col",
                    Link {
                        to: Route::Circles {},
                        div {
                            class: "card text-center h-100",
                                    div {
                                class: "card-body d-flex flex-column justify-content-center align-items-center",
                                style: "min-height: 150px;",
                                i { class: "bi bi-people fs-1 mb-2" }
                                span { "Circles" }
                            }
                        }
                    }
                }

                // Trees Box
                div { class: "col",
                    Link {
                        to: Route::Tree {},
                        div {
                            class: "card text-center h-100",
                                    div {
                                class: "card-body d-flex flex-column justify-content-center align-items-center",
                                            style: "min-height: 150px;",
                                i { class: "bi bi-diagram-3 fs-1 mb-2" }
                                span { "Trees" }
                            }
                        }
                    }
                }
            }
        }
    }
}
