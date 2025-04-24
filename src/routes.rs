use dioxus::prelude::*;
use crate::components::main_layout::MainLayout;
use crate::components::home::Home;
use crate::components::profile::Profile;
use crate::components::comms::Comms; // Changed from messages to comms
use crate::components::circles::Circles;
use crate::components::trees::Trees;
use crate::components::settings::Settings;
use crate::components::system_info::SystemInfo;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(MainLayout)]
    #[route("/")]
    Home {},
    #[route("/profile")]
    Profile {},
    #[route("/comms")]
    Comms {},
    #[route("/circles")]
    Circles {},
    #[route("/trees")]
    Trees {},
    #[route("/settings")]
    Settings {},
    #[route("/system-info")]
    SystemInfo {},
}
