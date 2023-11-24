#![allow(missing_docs)]

use std::fmt::Display;

use html_node::typed;

#[derive(Debug, Clone)]
struct Location {
    x: i32,
    y: i32,
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

typed::element! {
    CustomElement("custom-element") {
        location: Location,
    }
}

fn main() {
    let html = typed::html!(<CustomElement location=Location { x: 1, y: 2 } />);

    assert_eq!(
        html.to_string(),
        r#"<custom-element location="1,2"></custom-element>"#
    );
}
