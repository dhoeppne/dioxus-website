#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::Link;

pub fn navbar(cx: Scope) -> Element {
    cx.render(rsx! {
        ul {
            Link { to: "/", "About"}
            br {}
            Link { to: "/experience", "Experience"}
            br {}
            Link { to: "/work", "Work"}
            br {}
            Link { to: "/contact", "Contact"}
        }
    })
}

