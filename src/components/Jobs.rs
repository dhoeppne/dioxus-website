#![allow(non_snake_case)]
use dioxus::prelude::*;


pub fn Jobs(cx: Scope) -> Element {
  cx.render(rsx!(
    p {
      "Jobs"
    }
  ))
}
