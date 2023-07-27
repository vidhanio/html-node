# `html-node`

A HTML to node macro powered by [rstml](https://github.com/rs-tml/rstml).

```rust
let grocery_list = vec!["milk", "eggs", "bread"];

html! {
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
