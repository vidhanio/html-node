# `html-node`

HTML nodes in Rust. \[powered by [rstml](https://github.com/rs-tml/rstml)\].

## Note

If you want to use this purely for building websites in rust, consider using my other library, [hypertext](https://github.com/vidhanio/hypertext). It puts performance and type-checking first, and supports this crate's syntax as well as [maud](https://maud.lambda.xyz)'s syntax, which is much cleaner.

## Features

- Text escaping
- Pretty-printing
- Customizable compile-time type-checked elements and attributes ([docs](https://docs.rs/html-node/latest/html_node/typed/index.html))
  - completely optional, and can be mixed with untyped elements when needed!

## Example

```rust
let shopping_list = vec!["milk", "eggs", "bread"];

let shopping_list_html = html! {
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
```

<details open>
<summary>HTML Output</summary>

```rust
// the `#` flag enables pretty-printing
println!("{shopping_list_html:#}");
```

```html
<div>
    <h1>
        Shopping List
    </h1>
    <ul>
        <li class="item">
            <input type="checkbox" id="item-1">
            <label for="item-1">
                milk
            </label>
        </li>
        <li class="item">
            <input type="checkbox" id="item-2">
            <label for="item-2">
                eggs
            </label>
        </li>
        <li class="item">
            <input type="checkbox" id="item-3">
            <label for="item-3">
                bread
            </label>
        </li>
    </ul>
</div>
```

</details open>

<details>
<summary>Rust Output</summary>

```rust
println!("{shopping_list_html:#?}");
```

```rust
Element(
    Element {
        name: "div",
        attributes: [],
        children: Some(
            [
                Element(
                    Element {
                        name: "h1",
                        attributes: [],
                        children: Some(
                            [
                                Text(
                                    Text {
                                        text: "Shopping List",
                                    },
                                ),
                            ],
                        ),
                    },
                ),
                Element(
                    Element {
                        name: "ul",
                        attributes: [],
                        children: Some(
                            [
                                Fragment(
                                    Fragment {
                                        children: [
                                            Element(
                                                Element {
                                                    name: "li",
                                                    attributes: [
                                                        (
                                                            "class",
                                                            Some(
                                                                "item",
                                                            ),
                                                        ),
                                                    ],
                                                    children: Some(
                                                        [
                                                            Element(
                                                                Element {
                                                                    name: "input",
                                                                    attributes: [
                                                                        (
                                                                            "type",
                                                                            Some(
                                                                                "checkbox",
                                                                            ),
                                                                        ),
                                                                        (
                                                                            "id",
                                                                            Some(
                                                                                "item-1",
                                                                            ),
                                                                        ),
                                                                    ],
                                                                    children: None,
                                                                },
                                                            ),
                                                            Element(
                                                                Element {
                                                                    name: "label",
                                                                    attributes: [
                                                                        (
                                                                            "for",
                                                                            Some(
                                                                                "item-1",
                                                                            ),
                                                                        ),
                                                                    ],
                                                                    children: Some(
                                                                        [
                                                                            Text(
                                                                                Text {
                                                                                    text: "milk",
                                                                                },
                                                                            ),
                                                                        ],
                                                                    ),
                                                                },
                                                            ),
                                                        ],
                                                    ),
                                                },
                                            ),
                                            Element(
                                                Element {
                                                    name: "li",
                                                    attributes: [
                                                        (
                                                            "class",
                                                            Some(
                                                                "item",
                                                            ),
                                                        ),
                                                    ],
                                                    children: Some(
                                                        [
                                                            Element(
                                                                Element {
                                                                    name: "input",
                                                                    attributes: [
                                                                        (
                                                                            "type",
                                                                            Some(
                                                                                "checkbox",
                                                                            ),
                                                                        ),
                                                                        (
                                                                            "id",
                                                                            Some(
                                                                                "item-2",
                                                                            ),
                                                                        ),
                                                                    ],
                                                                    children: None,
                                                                },
                                                            ),
                                                            Element(
                                                                Element {
                                                                    name: "label",
                                                                    attributes: [
                                                                        (
                                                                            "for",
                                                                            Some(
                                                                                "item-2",
                                                                            ),
                                                                        ),
                                                                    ],
                                                                    children: Some(
                                                                        [
                                                                            Text(
                                                                                Text {
                                                                                    text: "eggs",
                                                                                },
                                                                            ),
                                                                        ],
                                                                    ),
                                                                },
                                                            ),
                                                        ],
                                                    ),
                                                },
                                            ),
                                            Element(
                                                Element {
                                                    name: "li",
                                                    attributes: [
                                                        (
                                                            "class",
                                                            Some(
                                                                "item",
                                                            ),
                                                        ),
                                                    ],
                                                    children: Some(
                                                        [
                                                            Element(
                                                                Element {
                                                                    name: "input",
                                                                    attributes: [
                                                                        (
                                                                            "type",
                                                                            Some(
                                                                                "checkbox",
                                                                            ),
                                                                        ),
                                                                        (
                                                                            "id",
                                                                            Some(
                                                                                "item-3",
                                                                            ),
                                                                        ),
                                                                    ],
                                                                    children: None,
                                                                },
                                                            ),
                                                            Element(
                                                                Element {
                                                                    name: "label",
                                                                    attributes: [
                                                                        (
                                                                            "for",
                                                                            Some(
                                                                                "item-3",
                                                                            ),
                                                                        ),
                                                                    ],
                                                                    children: Some(
                                                                        [
                                                                            Text(
                                                                                Text {
                                                                                    text: "bread",
                                                                                },
                                                                            ),
                                                                        ],
                                                                    ),
                                                                },
                                                            ),
                                                        ],
                                                    ),
                                                },
                                            ),
                                        ],
                                    },
                                ),
                            ],
                        ),
                    },
                ),
            ],
        ),
    },
)
```

</details open>
