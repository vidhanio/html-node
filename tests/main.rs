use html_node::{html, text};

#[test]
fn basic() {
    let html = html!(
        <div>
            <ul class="bg-red-500">
                { (0..5).map(|i| html!(<li>{ text!("Item {i}") }</li>)) }
            </ul>
        </div>
    );

    let expected = concat!(
        "<div>",
        r#"<ul class="bg-red-500">"#,
        "<li>Item 0</li>",
        "<li>Item 1</li>",
        "<li>Item 2</li>",
        "<li>Item 3</li>",
        "<li>Item 4</li>",
        "</ul>",
        "</div>",
    );

    assert_eq!(html.to_string(), expected);
}
