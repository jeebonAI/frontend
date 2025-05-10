use dioxus::prelude::*;

#[component]
pub fn SystemInfo() -> Element {
    rsx! {
        div { class: "container mt-2",
            div { class: "row",
                div { class: "col-md-6 mb-4",
                    div { class: "card h-100",
                        div { class: "card-header", "Application Information" }
                        div { class: "card-body",
                            div { class: "mb-3",
                                h5 { "Version" }
                                p { "Jeebon v0.0.4 (Demo)" }
                            }
                            div { class: "mb-3",
                                h5 { "Build Date" }
                                p { "April 26, 2023" }
                            }
                            div {
                                h5 { "License" }
                                p { "MIT License" }
                            }
                        }
                    }
                }

                div { class: "col-md-6 mb-4",
                    div { class: "card h-100",
                        div { class: "card-header", "System Status" }
                        div { class: "card-body",
                            div { class: "mb-3",
                                h5 { "Connection Status" }
                                div { class: "d-flex align-items-center",
                                    span { class: "badge bg-success me-2", "Connected" }
                                    "Peer-to-peer network active"
                                }
                            }
                            div { class: "mb-3",
                                h5 { "Active Peers" }
                                p { "42 peers connected" }
                            }
                            div {
                                h5 { "Storage Usage" }
                                div { class: "progress mb-2",
                                    div {
                                        class: "progress-bar",
                                        role: "progressbar",
                                        style: "width: 35%",
                                        "35%"
                                    }
                                }
                                p { class: "small text-muted", "3.5 GB of 10 GB used" }
                            }
                        }
                    }
                }
            }

            div { class: "row",
                div { class: "col-md-12 mb-4",
                    div { class: "card",
                        div { class: "card-header", "Network Diagnostics" }
                        div { class: "card-body",
                            div { class: "table-responsive",
                                table { class: "table",
                                    thead {
                                        tr {
                                            th { "Metric" }
                                            th { "Value" }
                                            th { "Status" }
                                        }
                                    }
                                    tbody {
                                        tr {
                                            td { "Latency" }
                                            td { "45 ms" }
                                            td { span { class: "badge bg-success", "Good" } }
                                        }
                                        tr {
                                            td { "Bandwidth" }
                                            td { "5.2 Mbps" }
                                            td { span { class: "badge bg-success", "Good" } }
                                        }
                                        tr {
                                            td { "Packet Loss" }
                                            td { "0.5%" }
                                            td { span { class: "badge bg-success", "Good" } }
                                        }
                                        tr {
                                            td { "DNS Resolution" }
                                            td { "32 ms" }
                                            td { span { class: "badge bg-success", "Good" } }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            div { class: "row",
                div { class: "col-md-12",
                    div { class: "card",
                        div { class: "card-header d-flex justify-content-between align-items-center",
                            "System Logs"
                            div {
                                select { class: "form-select form-select-sm",
                                    option { value: "all", "All Logs" }
                                    option { value: "error", "Errors Only" }
                                    option { value: "warning", "Warnings & Errors" }
                                    option { value: "info", "Info & Above" }
                                }
                            }
                        }
                        div { class: "card-body",
                            div { class: "log-container",
                                style: "height: 200px; overflow-y: auto; font-family: monospace; font-size: 0.9rem; background-color: #f8f9fa; padding: 10px; border-radius: 4px;",

                                div { class: "log-entry",
                                    span { class: "text-muted", "[2023-04-26 10:15:32] " }
                                    span { class: "text-success", "INFO " }
                                    "Application started successfully"
                                }
                                div { class: "log-entry",
                                    span { class: "text-muted", "[2023-04-26 10:15:33] " }
                                    span { class: "text-success", "INFO " }
                                    "Connected to peer network"
                                }
                                div { class: "log-entry",
                                    span { class: "text-muted", "[2023-04-26 10:15:35] " }
                                    span { class: "text-success", "INFO " }
                                    "Syncing data from network"
                                }
                                div { class: "log-entry",
                                    span { class: "text-muted", "[2023-04-26 10:15:40] " }
                                    span { class: "text-warning", "WARN " }
                                    "Slow response from peer 0x7a3b5c"
                                }
                                div { class: "log-entry",
                                    span { class: "text-muted", "[2023-04-26 10:15:45] " }
                                    span { class: "text-success", "INFO " }
                                    "Data sync completed"
                                }
                                div { class: "log-entry",
                                    span { class: "text-muted", "[2023-04-26 10:16:00] " }
                                    span { class: "text-success", "INFO " }
                                    "User authentication successful"
                                }
                            }
                        }
                        div { class: "card-footer",
                            button { class: "btn btn-sm btn-outline-secondary me-2",
                                i { class: "bi bi-download me-1" }
                                "Download Logs"
                            }
                            button { class: "btn btn-sm btn-outline-secondary",
                                i { class: "bi bi-arrow-clockwise me-1" }
                                "Refresh"
                            }
                        }
                    }
                }
            }
        }
    }
}
