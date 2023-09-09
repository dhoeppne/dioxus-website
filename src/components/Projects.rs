#![allow(non_snake_case)]
use dioxus::prelude::*;


pub fn Projects(cx: Scope) -> Element {
  cx.render(rsx!(
    section {
      "These are the "
      h1 { "Projects" }
      "I’m currently working on."
  }
  ))
}
