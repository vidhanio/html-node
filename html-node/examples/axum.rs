#![allow(missing_docs)]

use std::{
    collections::HashMap,
    net::{Ipv4Addr, SocketAddr},
};

use axum::{extract::Query, routing::get, Router, Server};
use html_node::{html, text, Node};
use html_node_core::pretty::Pretty;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 3000));

    println!("listening on {addr}...");

    Server::bind(&addr)
        .serve(router().into_make_service())
        .await
        .unwrap();
}

fn router() -> Router {
    Router::new()
        .route("/", get(home))
        .route("/about", get(about))
        .route("/contact", get(contact))
        .route("/greet", get(greet))
        .route("/pretty", get(pretty))
}

fn layout(content: Node) -> Node {
    const NAV_PAGES: &[(&str, &str)] =
        &[("/", "home"), ("/about", "about"), ("/contact", "contact")];

    html! {
        <html>
            <head>
                <title>"my website"</title>
            </head>
            <body>
                <nav>
                    <ul>
                        { NAV_PAGES.iter().map(|(href, title)| html! {
                            <li>
                                <a href={*href}>{text!("{title}")}</a>
                            </li>
                        }) }
                    </ul>
                </nav>
                <main>
                    {content}
                </main>
            </body>
        </html>
    }
}

async fn home() -> Node {
    layout(html! {
        <h1>"home"</h1>
        "welcome to my site!"

        <h2>"use the form to get a personalized greeting!"</h2>
        <form action="/greet">
            <label for="name">"name:"</label>
            <input id="name" name="name" type="text" />
            <button type="submit">"submit"</button>
        </form>
    })
}

async fn about() -> Node {
    layout(html! {
        <h1>"about"</h1>
        "my name is vidhan, and i'm a rust developer and a university student."
    })
}

async fn contact() -> Node {
    layout(html! {
        <h1>"contact"</h1>
    })
}

async fn greet(Query(params): Query<HashMap<String, String>>) -> Node {
    let name = params
        .get("name")
        .map_or("stranger", std::string::String::as_str);

    layout(html! {
        <h1>{text!("hello, {name}")}!</h1>
    })
}

async fn pretty() -> Pretty {
    Pretty(layout(html! {
        <div>
            <h1>Pretty</h1>
        </div>
    }))
}
