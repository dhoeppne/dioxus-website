#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use dioxus_router::{Router, Route, Redirect};

#[path = "./pages/mod.rs"]
mod pages;
#[path = "./components/mod.rs"]
mod components;

use self::components::navbar::navbar;

pub fn router(cx: Scope) -> Element {
  cx.render(rsx!{
    Router {
      p { "This is inside the router" }
      self::navbar {} // NEW
      Route { to: "/about", self::pages::About::About {}}
      Route { to: "/contact", self::pages::Contact::Contact {}}
      Route { to: "/work", self::pages::Work::Work {}}
      Route { to: "/experience", self::pages::Experience::Experience {}}
      //  if the current location doesn't match any of the above routes, redirect to "/home"
      Redirect { from: "", to: "/about" }
    }
  })
}
