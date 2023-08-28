#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

pub fn Home(cx: Scope) -> Element {
    let mut count: &UseState<i32> = use_state(cx, || 0);

    cx.render(rsx! {

        h1 { "High-Five counter: {count}" }
        button { onclick: move |_| count += 1, "Up high!" }
        button { onclick: move |_| count -= 1, "Down low!" }
        button { onclick: move |_| log!("test"), "wasm console.log"}
    })
}

