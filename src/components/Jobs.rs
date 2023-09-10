#![allow(non_snake_case)]
use dioxus::prelude::*;


pub fn Jobs(cx: Scope) -> Element {
  cx.render(rsx!(
    "These are the "
      h1 { "Jobs" }
      "I’ve had the pleasure of holding."
  ))
}
