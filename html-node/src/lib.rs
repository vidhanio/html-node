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
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod macros;
#[cfg(feature = "typed")]
pub mod typed;

pub use html_node_core::{Comment, Doctype, Element, Fragment, Node, Text, UnsafeText};
/// The HTML to [`Node`] macro.
///
/// See the [crate-level documentation](crate) for more information.
pub use html_node_macro::html;

pub use self::macros::*;
