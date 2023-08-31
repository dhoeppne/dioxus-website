#![allow(non_snake_case)]
use dioxus::prelude::*;


pub fn Contact(cx: Scope) -> Element {
  cx.render(rsx!(
    p {
      "Contact"
    }
  ))
}
