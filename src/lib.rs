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
//! [`Node`] implements [`Display`] (and by extension [`ToString`]), so you can
//! turn it into a string representation easily using
//! [`Node::to_string()`][ToString::to_string].
//!
//! See [the rstml docs](https://docs.rs/rstml/) for supported tags and syntax.
//!
//! # Example
//!
//! ```rust
//! use html_node::{html, text};
//!
//! # fn main () {
//! let grocery_list = vec!["milk", "eggs", "bread"];
//!
//! let html = html! {
//!     <div>
//!         <h1>Shopping List</h1>
//!         <ul>
//!             { grocery_list.into_iter().zip(1..).map(|(item, i)| html! {
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
//! # }
//! ```

#![warn(clippy::cargo)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]

mod macros;

use std::fmt::{self, Display, Formatter};

/// The [`html!`] macro.
///
/// This returns a [`Node`].
///
/// Refer to the [crate-level documentation](crate) for more information.
pub use html_node_macro::html;

/// An HTML node.
#[derive(Clone, Debug)]
pub enum Node {
    /// A comment.
    ///
    /// ```html
    /// <!-- I'm a comment! -->
    /// ```
    Comment(Comment),

    /// A doctype.
    ///
    /// ```html
    /// <!DOCTYPE html>
    /// ```
    Doctype(Doctype),

    /// A fragment.
    ///
    /// ```html
    /// <>
    ///     I'm in a fragment!
    /// </>
    /// ```
    Fragment(Fragment),

    /// An element.
    ///
    /// ```html
    /// <div class="container">
    ///     I'm in an element!
    /// </div>
    /// ```
    Element(Element),

    /// A text node.
    ///
    /// This can be created using [`text!`].
    ///
    /// ```html
    /// <div>
    ///     I'm a text node!
    /// </div>
    /// ```
    Text(Text),

    /// An unsafe text node.
    ///
    /// This can be created using [`unsafe_text!`].
    ///
    /// # Warning
    ///
    /// [`Node::UnsafeText`] is not escaped when rendered, and as such, can
    /// allow for XSS attacks. Use with caution!
    UnsafeText(UnsafeText),
}

impl<I> From<I> for Node
where
    I: IntoIterator<Item = Self>,
{
    fn from(iter: I) -> Self {
        Self::Fragment(Fragment {
            children: iter.into_iter().collect(),
        })
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Self::Comment(comment) => write!(f, "{comment}"),
            Self::Doctype(doctype) => write!(f, "{doctype}"),
            Self::Fragment(fragment) => write!(f, "{fragment}"),
            Self::Element(element) => write!(f, "{element}"),
            Self::Text(text) => write!(f, "{text}"),
            Self::UnsafeText(text) => write!(f, "{text}"),
        }
    }
}

/// A comment.
///
/// ```html
/// <!-- I'm a comment! -->
/// ```
#[derive(Clone, Debug)]
pub struct Comment {
    /// The text of the comment.
    ///
    /// ```html
    /// <!-- comment -->
    /// ```
    pub comment: String,
}

impl Display for Comment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<!-- {} -->", self.comment)
    }
}

/// A doctype.
///
/// ```html
/// <!DOCTYPE html>
/// ```
#[derive(Clone, Debug)]
pub struct Doctype {
    /// The value of the doctype.
    ///
    /// ```html
    /// <!DOCTYPE synax>
    /// ```
    pub syntax: String,
}

impl Display for Doctype {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<!DOCTYPE {}>", self.syntax)
    }
}

/// A fragment.
///
/// ```html
/// <>
///     I'm in a fragment!
/// </>
/// ```
#[derive(Clone, Debug)]
pub struct Fragment {
    /// The children of the fragment.
    ///
    /// ```html
    /// <>
    ///     <!-- I'm a child! -->
    ///     <child>I'm another child!</child>
    /// </>
    pub children: Vec<Node>,
}

impl Display for Fragment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for child in &self.children {
            write!(f, "{child}")?;
        }

        Ok(())
    }
}

/// An element.
///
/// ```html
/// <div class="container">
///     I'm in an element!
/// </div>
/// ```
#[derive(Clone, Debug)]
pub struct Element {
    /// The name of the element.
    ///
    /// ```html
    /// <name>
    /// ```
    pub name: String,

    /// The attributes of the element.
    ///
    /// ```html
    /// <div attribute="value">
    /// ```
    pub attributes: Vec<(String, Option<String>)>,

    /// The children of the element.
    ///
    /// ```html
    /// <div>
    ///     <!-- I'm a child! -->
    ///     <child>I'm another child!</child>
    /// </div>
    /// ```
    pub children: Option<Vec<Node>>,
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<{}", self.name)?;

        for (key, value) in &self.attributes {
            write!(f, " {key}")?;

            if let Some(value) = value {
                let encoded_value = html_escape::encode_double_quoted_attribute(value);
                write!(f, r#"="{encoded_value}""#)?;
            }
        }
        write!(f, ">")?;

        if let Some(children) = &self.children {
            for child in children {
                write!(f, "{child}")?;
            }

            write!(f, "</{}>", self.name)?;
        };

        Ok(())
    }
}

/// A text node.
///
/// This can be created using [`text!`].
///
/// ```html
/// <div>
///    I'm a text node!
/// </div>
#[derive(Clone, Debug)]
pub struct Text {
    /// The text of the node.
    ///
    /// ```html
    /// <div>
    ///    text
    /// </div>
    pub text: String,
}

impl Display for Text {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let encoded_value = html_escape::encode_text_minimal(&self.text);
        write!(f, "{encoded_value}")
    }
}

/// An unsafe text node.
///
/// This can be created using [`unsafe_text!`].
///
/// # Warning
///
/// [`Node::UnsafeText`] is not escaped when rendered, and as such, can allow
/// for XSS attacks. Use with caution!
#[derive(Clone, Debug)]
pub struct UnsafeText {
    /// The text of the node.
    pub text: String,
}

impl Display for UnsafeText {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}
