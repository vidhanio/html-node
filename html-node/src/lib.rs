//! A HTML to [`Node`] macro powered by [rstml](https://github.com/rs-tml/rstml).
//!
//! Values returned from braced blocks (`{ ... }`) are expected to return
//! something that implements [`Into<Node>`]. This is already implemented for
//! anything that implements [`IntoIterator<Item = Node>`](IntoIterator), so you
//! can return something like a [`Vec<Node>`] or an
//! [`Iterator<Item = Node>`](Iterator) directly.
//!
//! Due to Rust's trait implementation rules, you cannot directly return
//! [`String`]s. Instead, you can use the [`text!`] macro to convert the
//! [`String`] to a [`Node::Text`].
//!
//! [`Node`] implements [`Display`][std::fmt::Display] (and by extension
//! [`ToString`]), so you can turn it into a string representation easily using
//! [`Node::to_string()`][ToString::to_string].
//!
//! See [the rstml docs](https://docs.rs/rstml/) for supported tags and syntax.
//!
//! # Examples
//!
//! ## Basic
//!
//! ```rust
//! use html_node::{html, text};
//!
//! let shopping_list = vec!["milk", "eggs", "bread"];
//!
//! let html = html! {
//!     <div>
//!         <h1>Shopping List</h1>
//!         <ul>
//!             { shopping_list.into_iter().zip(1..).map(|(item, i)| html! {
//!                 <li class="item">
//!                     <input type="checkbox" id={format!("item-{i}")}>
//!                     <label for={format!("item-{i}")}>{text!("{item}")}</label>
//!                 </li>
//!             }) }
//!         </ul>
//!     </div>
//! };
//!
//! let expected = "\
//! <div>\
//!     <h1>Shopping List</h1>\
//!     <ul>\
//!         <li class=\"item\">\
//!             <input type=\"checkbox\" id=\"item-1\">\
//!             <label for=\"item-1\">milk</label>\
//!         </li>\
//!         <li class=\"item\">\
//!             <input type=\"checkbox\" id=\"item-2\">\
//!             <label for=\"item-2\">eggs</label>\
//!         </li>\
//!         <li class=\"item\">\
//!             <input type=\"checkbox\" id=\"item-3\">\
//!             <label for=\"item-3\">bread</label>\
//!         </li>\
//!     </ul>\
//! </div>\
//! ";
//!
//! assert_eq!(html.to_string(), expected);
//! ```
//!
//! ## Pretty-Printing
//!
//! ```rust
//! use html_node::{html, text};
//!
//! let html = html! {
//!     <div>
//!         <h1>Shopping List</h1>
//!        <ul>
//!            <li>Eggs</li>
//!            <li>Milk</li>
//!            <li>Bread</li>
//!        </ul>
//!     </div>
//! };
//!
//! let expected = "\
//! <div>
//!     <h1>
//!         Shopping List
//!     </h1>
//!     <ul>
//!         <li>
//!             Eggs
//!         </li>
//!         <li>
//!             Milk
//!         </li>
//!         <li>
//!             Bread
//!         </li>
//!     </ul>
//! </div>\
//! ";
//!
//! // note the `#` in the format string, which enables pretty-printing
//! let formatted_html = format!("{html:#}");
//!
//! assert_eq!(formatted_html, expected);
//! ```

#![warn(clippy::cargo)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

mod macros;

pub use html_node_core::*;
/// The HTML to [`Node`] macro.
///
/// See the [crate-level documentation](crate) for more information.
pub use html_node_macro::html;
/// The text to [`Node::Text`] macro which checks element/attribute types.
///
/// # Examples
///
/// ## Passing Type-Checking
///
/// ```rust
/// use html_node::{html, text, typed_html, typed::elements::div};
///
/// let html = typed_html! {
///     <div class="cool" id="hello-world" data-my-attr="hello" aria-label="world">
///         {text!("Hello, world!")}
///     </div>
/// };
///
/// let expected = "\
/// <div class=\"cool\" id=\"hello-world\" data-my-attr=\"hello\" aria-label=\"world\">\
///     Hello, world!\
/// </div>\
/// ";
///
/// assert_eq!(html.to_string(), expected);
/// ```
///
/// ## Failing Type-Checking
///
/// ```compile_fail
/// use html_node::{html, text, typed_html, typed::elements::div};
///                                      // ^^^^^^^^^^^^^^^^^^^^
///                                      // must import to get definition of `div` in macros.
///                                      // can also `use html_node::typed::elements::*;`
///
/// let html = typed_html! {
///     // ERROR: struct `html_node::typed::elements::DivAttributes` has no field named `my_attr`
///     <div class="cool" id="hello-world" my-attr="hello">
///         {text!("Hello, world!")}
///     </div>
/// };
/// ```
pub use html_node_macro::typed_html;

pub use self::macros::*;
