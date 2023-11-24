use html_node::{
    text,
    typed::{self, elements::*, html},
};

#[test]
fn basic() {
    let shopping_list = vec!["milk", "eggs", "bread"];

    let html = html! {
        <div>
            <h1>Shopping List</h1>
            <ul>
                { shopping_list.into_iter().zip(1..).map(|(item, i)| html! {
                    <li class="item">
                        <input id={format!("item-{i}")} type="checkbox">
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
                <input id=\"item-1\" type=\"checkbox\">\
                <label for=\"item-1\">milk</label>\
            </li>\
            <li class=\"item\">\
                <input id=\"item-2\" type=\"checkbox\">\
                <label for=\"item-2\">eggs</label>\
            </li>\
            <li class=\"item\">\
                <input id=\"item-3\" type=\"checkbox\">\
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
                        <input id={format!("item-{i}")} type="checkbox">
                        <label for={format!("item-{i}")}>{text!("{item}")}</label>
                    </li>
                }) }
            </ul>
        </div>
    };

    println!("--- pretty-printed ---\n{html:#}");

    let expected = "\
<div>
    <h1>
        Shopping List
    </h1>
    <ul>
        <li class=\"item\">
            <input id=\"item-1\" type=\"checkbox\">
            <label for=\"item-1\">
                milk
            </label>
        </li>
        <li class=\"item\">
            <input id=\"item-2\" type=\"checkbox\">
            <label for=\"item-2\">
                eggs
            </label>
        </li>
        <li class=\"item\">
            <input id=\"item-3\" type=\"checkbox\">
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

#[test]
fn component() {
    typed::component! {
        ShoppingListItem {
            index: i32,
        };

        |ShoppingListItemAttributes { index }, _, children| html! {
            <li class="item">
                <input id=format!("item-{index}") type="checkbox">
                <label for=format!("item-{index}")>{children}</label>
            </li>
        }
    }

    let shopping_list = vec!["milk", "eggs", "bread"];

    let html = html! {
        <div>
            <h1>Shopping List</h1>
            <ul>
                { shopping_list.into_iter().zip(1..).map(|(item, i)| html! {
                    <ShoppingListItem component index=i>{text!("{item}")}</ShoppingListItem>
                }) }
            </ul>
        </div>
    };

    let expected = "\
<div>\
    <h1>Shopping List</h1>\
    <ul>\
        <li class=\"item\">\
            <input id=\"item-1\" type=\"checkbox\">\
            <label for=\"item-1\">milk</label>\
        </li>\
        <li class=\"item\">\
            <input id=\"item-2\" type=\"checkbox\">\
            <label for=\"item-2\">eggs</label>\
        </li>\
        <li class=\"item\">\
            <input id=\"item-3\" type=\"checkbox\">\
            <label for=\"item-3\">bread</label>\
        </li>\
    </ul>\
</div>\
";

    assert_eq!(html.to_string(), expected);
}
