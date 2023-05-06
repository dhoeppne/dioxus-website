#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use dioxus_router::{Router, Route, Redirect};

use crate::pages::{About::About, Contact::Contact, Work::Work, Experience::Experience};
use crate::components::navbar;

pub fn router(cx: Scope) -> Element {
  cx.render(rsx!{
    Router {
      p { "This is inside the router" }
      navbar::navbar {}
      Route { to: "/about", About {}}
      Route { to: "/contact", Contact {}}
      Route { to: "/work", Work {}}
      Route { to: "/experience", Experience {}}
      //  if the current location doesn't match any of the above routes, redirect to "/home"
      Redirect { from: "", to: "/about" }
    }
  })
}
