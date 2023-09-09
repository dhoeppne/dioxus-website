#![allow(non_snake_case)]

use dioxus_fullstack::prelude::*;

#[macro_use]
mod macros;

mod Router;
mod components;


fn main() {
    let config = LaunchBuilder::<FullstackRouterConfig<crate::Router::Route>>::router();
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
