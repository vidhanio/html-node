use html_node::{html, text};

#[test]
fn basic() {
    let shopping_list = vec!["milk", "eggs", "bread"];

    let html = html! {
        <div>
            <h1>Shopping List</h1>
            <ul>
                { shopping_list.into_iter().zip(1..).map(|(item, i)| html! {
                    <li class="item">
                        <input type="checkbox" id={format!("item-{i}")}>
                        <label for={format!("item-{i}")}>{text!("{item}")}</label>
                    </li>
                }) }
            </ul>
        </div>
    };

    let expected = "\
    <div>\
        <h1>Shopping List</h1>\
        <ul>\
            <li class=\"item\">\
                <input type=\"checkbox\" id=\"item-1\">\
                <label for=\"item-1\">milk</label>\
            </li>\
            <li class=\"item\">\
                <input type=\"checkbox\" id=\"item-2\">\
                <label for=\"item-2\">eggs</label>\
            </li>\
            <li class=\"item\">\
                <input type=\"checkbox\" id=\"item-3\">\
                <label for=\"item-3\">bread</label>\
            </li>\
        </ul>\
    </div>\
    ";

    assert_eq!(html.to_string(), expected);
}

#[test]
fn pretty_printed() {
    let shopping_list = vec!["milk", "eggs", "bread"];

    let html = html! {
        <div>
            <h1>Shopping List</h1>
            <ul>
                { shopping_list.into_iter().zip(1..).map(|(item, i)| html! {
                    <li class="item">
                        <input type="checkbox" id={format!("item-{i}")}>
                        <label for={format!("item-{i}")}>{text!("{item}")}</label>
                    </li>
                }) }
            </ul>
        </div>
    };

    let expected = "\
<div>
    <h1>
        Shopping List
    </h1>
    <ul>
        <li class=\"item\">
            <input type=\"checkbox\" id=\"item-1\">
            <label for=\"item-1\">
                milk
            </label>
        </li>
        <li class=\"item\">
            <input type=\"checkbox\" id=\"item-2\">
            <label for=\"item-2\">
                eggs
            </label>
        </li>
        <li class=\"item\">
            <input type=\"checkbox\" id=\"item-3\">
            <label for=\"item-3\">
                bread
            </label>
        </li>
        
    </ul>
</div>\
";

    let pretty_html = format!("{html:#}");

    assert_eq!(pretty_html, expected);
}
