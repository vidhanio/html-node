use html_node::{html, style};

fn main() {
    bare_style();
    string_style();
    macro_style_unsupported_css();
    macro_style_supported_css();
}

/// Try and directly insert CSS into the style element.
///
/// # Output
///
/// ```text
/// Bare style:
/// <div>
///    <style>
///        ul { outline : 5px solid # CCDDFF ; padding - top : 15px ; }
///    </style>
///    <ul>
///        <li>
///            one
///        </li>
///        <li>
///            two
///        </li>
///    </ul>
/// </div>
/// ```
fn bare_style() {
    let node = html! {
        <div>
            <style>
                ul {
                    outline: 5px solid #CCDDFF;
                    padding-top: 15px;
                }
            </style>
            <ul>
                <li>one</li>
                <li>two</li>
            </ul>
        </div>
    };

    println!("Bare style:\n{node:#}");
}

/// Try and insert CSS into the style element via a string.
///
/// # Output
///
/// ```text
/// String style:
/// <div><style>"
///                 ul {
///                     outline: 5px solid #CCDDFF;
///                     padding-top: 15px;
///                 }
///             "</style><ul><li>one</li><li>two</li></ul></div>
///
/// Pretty string style:
/// <div>
///     <style>
///         "
///                         ul {
///                             outline: 5px solid #CCDDFF;
///                             padding-top: 15px;
///                         }
///                     "
///     </style>
///     <ul>
///         <li>
///             one
///         </li>
///         <li>
///             two
///         </li>
///     </ul>
/// </div>
/// ```
fn string_style() {
    let node = html! {
        <div>
            <style>"
                ul {
                    outline: 5px solid #CCDDFF;
                    padding-top: 15px;
                }
            "</style>
            <ul>
                <li>one</li>
                <li>two</li>
            </ul>
        </div>
    };

    println!("String style:\n{node}");
    println!("Pretty string style:\n{node:#}");
}

/// Insert a style element and inner CSS content.
///
/// The macro naively strips all whitespace from the CSS content, meaning the
/// shorthand version of outline as used below will still be rendered
/// incorrectly. See the next example for a workaround.
///
/// Note that the `<style></style>` tags are inserted by the macro.
///
/// # Output
///
/// ```text
/// Macro + unsupported CSS style:
/// <div>
///     <style>
///         ul{outline:5pxsolid#CCDDFF;padding-top:15px;}
///     </style>
///     <ul>
///         <li>
///             one
///         </li>
///         <li>
///             two
///         </li>
///     </ul>
/// </div>
/// ```
fn macro_style_unsupported_css() {
    let node = html! {
        <div>
            { style! {
                ul {
                    outline: 5px solid #CCDDFF;
                    padding-top: 15px;
                }
            } }
            <ul>
                <li>one</li>
                <li>two</li>
            </ul>
        </div>
    };

    println!("Macro + unsupported CSS style:\n{node:#}");
}

/// Insert a style element and inner CSS content, correctly.
///
/// Since the macro strips all whitespace, use long-form CSS properties to
/// specify the needed selectors and rules.
///
/// Note that the `<style></style>` tags are inserted by the macro.
///
/// # Output
///
/// ```text
/// Macro + CSS style:
/// <div>
///     <style>
///         ul{outline-width:5px;outline-style:solid;outline-color:#CCDDFF;padding-top:15px;}
///     </style>
///     <ul>
///         <li>
///             one
///         </li>
///         <li>
///             two
///         </li>
///     </ul>
/// </div>
/// ```
fn macro_style_supported_css() {
    let node = html! {
        <div>
            { style! {
                ul {
                    outline-width: 5px;
                    outline-style: solid;
                    outline-color: #CCDDFF;
                    padding-top: 15px;
                }
            } }
            <ul>
                <li>one</li>
                <li>two</li>
            </ul>
        </div>
    };

    println!("Macro + CSS style:\n{node:#}");
}
