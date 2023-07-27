# `html-node`

A HTML to node macro powered by [rstml](https://github.com/rs-tml/rstml).

```rust
let grocery_list = vec!["milk", "eggs", "bread"];

let shopping_list_html = html! {
    <div>
        <h1>Shopping List</h1>
        <ul>
            { grocery_list.into_iter().zip(1..).map(|(item, i)| html! {
                <li class="item">
                    <input type="checkbox" id={format!("item-{i}")}>
                    <label for={format!("item-{i}")}>{text!("{item}")}</label>
                </li>
            }) }
        </ul>
    </div>
};
```

<details><summary>Output</summary>

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
