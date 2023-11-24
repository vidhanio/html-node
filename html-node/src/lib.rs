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
//! # Typed HTML
//!
//! This crate also supports typed HTML, which is all nested into the [`typed`]
//! module. note that the feature `typed` must be enabled to use it.
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
//! Pretty-printing is supported by default when formatting a [`Node`] using the
//! alternate formatter, specified by a `#` in the format string.
//!
//! If you want to avoid specifying the alternate formatter, enabling the
//! `pretty` feature will provide a convenience method [`Node::pretty()`] that
//! returns a wrapper around the node that will always be pretty-printed.
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
//! // Note the `#` in the format string, which enables pretty-printing
//! let formatted_html = format!("{html:#}");
//!
//! assert_eq!(formatted_html, expected);
//!
//! # #[cfg(feature = "pretty")]
//! # {
//! // Wrap the HTML node in a pretty-printing wrapper.
//! let pretty = html.pretty();
//!
//! // Get the pretty-printed HTML as a string by invoking the [`Display`][std::fmt::Display] trait.
//! let pretty_html_string = pretty.to_string();
//! // Note the '#' is not required here.
//! let pretty_html_format = format!("{pretty}");
//!
//! assert_eq!(pretty_html_string, expected);
//! assert_eq!(pretty_html_format, expected);
//! assert_eq!(pretty_html_string, pretty_html_format);
//! # }
//! ```

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod macros;
#[cfg(feature = "typed")]
pub mod typed;

#[cfg(feature = "pretty")]
pub use html_node_core::pretty;
pub use html_node_core::{Comment, Doctype, Element, Fragment, Node, Text, UnsafeText};
/// The HTML to [`Node`] macro.
///
/// See the [crate-level documentation](crate) for more information.
pub use html_node_macro::html;

pub use self::macros::*;
