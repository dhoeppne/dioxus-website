#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

// user files
mod pages;
mod router;
mod components;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    // let container_style = r#"
    //     text-align: center;
    //     color: red
    // "#;
    cx.render(rsx! {
        router::router {}
        // header {
        //     "This is a header"
        // },
        // div {
        //     style: "{container_style}",
        //     "Hello, world!"
        // },
        // img {
        //     src: "https://i.imgflip.com/2zh47r.jpg",
        //     height: "500px",
        // }
    })
}
