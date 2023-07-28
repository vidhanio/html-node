use std::{
    collections::HashMap,
    net::{Ipv4Addr, SocketAddr},
};

use axum::{extract::Query, routing::get, Router, Server};
use html_node::{html, text, Node};

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 3000));

    println!("listening on {addr}...");

    Server::bind(&addr)
        .serve(router().into_make_service())
        .await
        .unwrap()
}

fn router() -> Router {
    Router::new()
        .route("/", get(home))
        .route("/about", get(about))
        .route("/contact", get(contact))
        .route("/greet", get(greet))
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
        .map(|name| name.as_str())
        .unwrap_or("stranger");

    layout(html! {
        <h1>{text!("hello, {name}")}!</h1>
    })
}
