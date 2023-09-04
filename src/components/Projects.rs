#![allow(non_snake_case)]
use dioxus::prelude::*;


pub fn Projects(cx: Scope) -> Element {
  cx.render(rsx!(
    p {
      "Projects"
    }
  ))
}
