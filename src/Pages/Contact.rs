#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;


pub fn Contact(cx: Scope) -> Element {
  cx.render(rsx!(
    p {
      "Contact"
    }
  ))
}
