#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

pub fn Me(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);
    let text = use_state(cx, || "...".to_string());

    cx.render(rsx! {
        section {
            "Hi, my name is"
            h1 { "David Hoeppner"}
            h1 {"I build things for people who build things."}
            "I’m a Canadian full-stack software developer specializing in developer operations and tooling. Currently, I’m focused on building out the Search Experience at Workday. This website serves as a portfolio of my work and a proof of concept of Rust for the Web."
        }
        h1 { "High-Five counter: {count}" }
        button { onclick: move |_| count += 1, "Up high!" }
        button { onclick: move |_| count -= 1, "Down low!" }
        button {
            onclick: move |_| {
                to_owned![text];
                async move {
                    if let Ok(data) = get_server_data().await {
                        println!("Client received: {}", data);
                        text.set(data.clone());
                        post_server_data(data).await.unwrap();
                    }
                }
            },
            "Run server function!"
        }
        "Server said: {text}"
        button { onclick: move |_| log!("Hello from the client!"), "Log!" }
    })
}

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    println!("Server received: {}", data);

    Ok(())
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}