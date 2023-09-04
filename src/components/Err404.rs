use dioxus::prelude::*;

#[inline_props]
pub fn Err404(cx: Scope, route: Vec<String>) -> Element {
    // error!("404: {:?}", route);
    println!("404: {:?}", route);
    let route_error = format!("log:\nattemped to navigate to: {:?}", route);

    render! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre {
            color: "red",
            route_error
        }
    }
}