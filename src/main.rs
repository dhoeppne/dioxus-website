#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use dioxus_router::prelude::*;

#[macro_use]
mod logger;

mod components;
use components::{ Err404::Err404, Home::Home, Contact::Contact };

fn main() {
    let config = LaunchBuilder::<FullstackRouterConfig<Route>>::router();
    #[cfg(feature = "ssr")]
    config
        .incremental(
            IncrementalRendererConfig::default()
                .invalidate_after(std::time::Duration::from_secs(120)),
        )
        .launch();

    #[cfg(not(feature = "ssr"))]
    config.launch();
}

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/contact")]
    Contact {},
    #[route("/:..route")]
    Err404 { route: Vec<String> },
}

// #[inline_props]
// fn Blog(cx: Scope, id: i32) -> Element {
//     render! {
//         Link { to: Route::Home {}, "Go to counter" }
//         table {
//             tbody {
//                 for _ in 0..*id {
//                     tr {
//                         for _ in 0..*id {
//                             td { "hello world!" }
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }
