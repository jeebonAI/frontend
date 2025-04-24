use dioxus::prelude::*;
use crate::components::main_layout::MainLayout;
use crate::components::home::Home;
use crate::components::profile::Profile;
use crate::components::messages::Messages;
use crate::components::circles::Circles;
use crate::components::trees::Trees;
use crate::components::settings::Settings;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(MainLayout)]
    #[route("/")]
    Home {},
    #[route("/profile")]
    Profile {},
    #[route("/messages")]
    Messages {},
    #[route("/circles")]
    Circles {},
    #[route("/trees")]
    Trees {},
    #[route("/settings")]
    Settings {},
}