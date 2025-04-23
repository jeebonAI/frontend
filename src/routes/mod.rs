use dioxus::prelude::*;
use dioxus_router::prelude::*;

// Import pages
use crate::pages::home::Home;
use crate::pages::profile::Profile;
use crate::pages::messages::Messages;
use crate::pages::circles::Circles;
use crate::pages::trees::Trees;
use crate::pages::settings::Settings;

// Import layouts
use crate::layouts::main_layout::MainLayout;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
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
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

pub fn configure_routes() -> Router<Route> {
    Router::with_config(|cfg| {
        cfg.nested(|_| {
            Route::Home {}
        })
        .wrap(MainLayout)
    })
}
