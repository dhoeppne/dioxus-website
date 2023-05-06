#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

// create a component that renders a div with the text "Hello, world!"
pub fn Nav(cx: Scope) -> Element {
    cx.render(rsx! {
        nav {
            a {
                href: "https://www.google.com/",
                target: "_blank",
                "Google"
            }
        }
    })
}
