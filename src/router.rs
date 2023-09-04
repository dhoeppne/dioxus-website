#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::components;
use components::{ Err404::Err404, Me::Me, Contact::Contact, Projects::Projects, Jobs::Jobs };

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Route {
    #[layout(NavBarFooter)]
    #[route("/")]
    Me {},
    #[route("/contact")]
    Contact {},
    #[route("/projects")]
    Projects {},
    #[route("/jobs")]
    Jobs {},
    #[end_layout]
    #[route("/:..route")]
    Err404 { route: Vec<String> },
}

#[inline_props]
fn NavBarFooter(cx: Scope) -> Element {
    render! {
        nav {
            ul {
                li {
                    Link {
                        // The Link component will navigate to the route specified
                        // in the target prop which is checked to exist at compile time
                        to: Route::Me {},
                        "Me"
                    }
                    Link {
                        to: Route::Contact {},
                        "Contact"
                    }
                    Link {
                        to: Route::Projects {},
                        "Projects"
                    }
                    Link {
                        to: Route::Jobs {},
                        "Jobs"
                    }
                }
            }
        }
        Outlet::<Route> {}
    }
}