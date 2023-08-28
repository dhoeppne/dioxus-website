#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use dioxus_router::prelude::*;
use serde::{Deserialize, Serialize};

#[macro_use]
mod logger;

mod router;
mod Pages;

use Pages::{Home::Home, Err404::Err404, Contact::Contact};

/// An enum of all of the possible routes in the app.
#[derive(Clone, Routable, Debug, PartialEq, Serialize, Deserialize)]
pub enum Route {
    // The home page is at the / route
    #[route("/")]
    Contact {},
    // #[route("/contact")]
    // Contact {},
    // If the name of the component and variant are the same you can omit the component and props name
    // If they are different you can specify them like this:
    // #[route("/", ComponentName, PropsName)]
    #[route("/:..route")]
    Err404 { route: Vec<String> },
}

fn main() {
    LaunchBuilder::new(app).launch();
}

fn app(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}

